use crate::{BoxError, BoxResult};
use futures::future::join_all;
use log::info;
use reqwest::{Client, ClientBuilder};
use std::time::{Duration, Instant};

pub async fn rank_mirrors(uris: &[&str]) -> BoxResult<Vec<String>> {
    let client = ClientBuilder::new().timeout(Duration::new(5, 0)).build()?;

    let machine = uname::uname()?.machine;

    let rankings_iter = uris.iter().map(|mirror| {
        let client = &client;
        let machine = &machine;
        async move {
            let duration = measure_mirror(client, mirror, machine).await?;
            info!("Server {} responded in {}ms", mirror, duration.as_millis());

            Ok::<(String, Duration), BoxError>((mirror.to_string(), duration))
        }
    });

    let rankings_result = join_all(rankings_iter).await;
    let mut rankings: Vec<(String, Duration)> =
        rankings_result.into_iter().filter_map(Result::ok).collect();

    rankings.sort_by(|(_, a), (_, b)| a.cmp(b));

    let sorted_mirrors: Vec<String> = rankings.into_iter().map(|(mirror, _)| mirror).collect();

    Ok(sorted_mirrors)
}

async fn measure_mirror(client: &Client, mirror: &str, machine: &str) -> BoxResult<Duration> {
    let uri = mirror
        .to_string()
        .replace("$arch", machine)
        .replace("$repo", "core");

    let now = Instant::now();

    #[cfg(not(test))]
    let path = uri;
    #[cfg(test)]
    let path = tests::SERVER.url(uri);

    client.get(path.as_str()).send().await?.error_for_status()?;

    Ok(now.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::Method::GET;
    use httpmock::MockServer;

    lazy_static::lazy_static! {
        pub static ref SERVER: MockServer = MockServer::start();
    }

    #[tokio::test]
    async fn it_can_rank_mirrors() {
        let slow_delay = Duration::from_millis(200);
        let slower_delay = Duration::from_millis(250);

        let mirrors = vec!["/slower_mirror", "/slow_mirror", "/fast_mirror"];

        let fast = SERVER.mock(|when, then| {
            when.method(GET).path("/fast_mirror");
            then.status(200).body("");
        });

        let slow = SERVER.mock(|when, then| {
            when.method(GET).path("/slow_mirror");
            then.status(200).body("").delay(slow_delay);
        });

        let slower = SERVER.mock(|when, then| {
            when.method(GET).path("/slower_mirror");
            then.status(200).body("").delay(slower_delay);
        });

        let ranked_mirrors = (rank_mirrors(&mirrors).await).unwrap();

        fast.assert();
        slow.assert();
        slower.assert();

        assert_eq!(
            ranked_mirrors,
            vec![
                String::from("/fast_mirror"),
                String::from("/slow_mirror"),
                String::from("/slower_mirror"),
            ]
        );
    }

    #[tokio::test]
    async fn it_handles_dead_mirrors() {
        let mirrors = vec!["/dead_mirror", "/mirror"];

        let mirror = SERVER.mock(|when, then| {
            when.method(GET).path("/mirror");
            then.status(200).body("");
        });

        let living_mirrors = (rank_mirrors(&mirrors).await).unwrap();

        mirror.assert();
        assert_eq!(living_mirrors, vec![String::from("/mirror")]);
    }
}
