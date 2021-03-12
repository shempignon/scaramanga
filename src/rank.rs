use crate::{BoxError, BoxResult};
use futures::future::join_all;
use log::info;
use reqwest::{Client, ClientBuilder};
use std::time::{Duration, Instant};

pub async fn rank_mirrors(mirrors: &Vec<String>) -> BoxResult<Vec<String>> {
    let client = ClientBuilder::new().timeout(Duration::new(5, 0)).build()?;

    let machine = uname::uname()?.machine;

    let rankings_iter = mirrors.iter().map(|mirror| {
        let client = &client;
        let machine = &machine;
        async move {
            let duration = measure_mirror(client, mirror.as_str(), machine).await?;
            info!("Server {} responded in {}ms", mirror, duration.as_millis());

            Ok::<(String, Duration), BoxError>((mirror.to_string().to_owned(), duration.to_owned()))
        }
    });

    let rankings_result = join_all(rankings_iter).await;
    let mut rankings: Vec<(String, Duration)> =
        rankings_result.into_iter().filter_map(Result::ok).collect();

    rankings.sort_by(|(_, a), (_, b)| a.cmp(b));

    let sorted_mirrors = rankings
        .iter()
        .map(|(mirror, _)| mirror.to_string())
        .collect::<Vec<String>>();

    Ok(sorted_mirrors)
}

async fn measure_mirror(client: &Client, mirror: &str, machine: &str) -> BoxResult<Duration> {
    let uri = mirror
        .to_string()
        .replace("$arch", machine)
        .replace("$repo", "core");

    let now = Instant::now();

    client.get(uri.as_str()).send().await?.error_for_status()?;

    Ok(now.elapsed())
}
