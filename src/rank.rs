use crate::BoxResult;
use futures::future::join_all;
use reqwest::{Client, ClientBuilder};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct RankedMirror(String, Duration);

impl Display for RankedMirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "# Responded in {}ms\nServer = {}",
            self.1.as_millis(),
            self.0
        )
    }
}

#[derive(Debug)]
pub struct Ranker {
    client: Client,
}

impl Ranker {
    pub fn new() -> BoxResult<Self> {
        let client = ClientBuilder::new().timeout(Duration::new(2, 0)).build()?;

        Ok(Self { client })
    }

    pub async fn rank_mirrors(&self, uris: &[&str]) -> BoxResult<Vec<RankedMirror>> {
        let futures = uris.iter().map(|uri| self.measure_mirror(uri));
        let results = join_all(futures).await;
        let mut rankings: Vec<RankedMirror> = results.into_iter().filter_map(Result::ok).collect();
        rankings.sort_by(|a, b| a.1.cmp(&b.1));
        Ok(rankings)
    }

    async fn measure_mirror(&self, mirror: &str) -> BoxResult<RankedMirror> {
        let uri = mirror
            .to_string()
            .replace("$arch", "x86_64")
            .replace("$repo", "core");

        let now = Instant::now();

        #[cfg(not(test))]
        let path = uri;
        #[cfg(test)]
        let path = tests::SERVER.url(uri);

        self.client
            .get(path.as_str())
            .send()
            .await?
            .error_for_status()?;

        Ok(RankedMirror(mirror.to_string(), now.elapsed()))
    }
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

        let ranker = Ranker::new().unwrap();
        let ranked_mirrors = ranker.rank_mirrors(&mirrors).await.unwrap();

        fast.assert();
        slow.assert();
        slower.assert();

        let fast_mirror = &ranked_mirrors[0];
        let slow_mirror = &ranked_mirrors[1];
        let slower_mirror = &ranked_mirrors[2];

        assert_eq!("/fast_mirror", fast_mirror.0.as_str());
        assert!(fast_mirror.1.as_millis() < 200);
        assert_eq!("/slow_mirror", slow_mirror.0.as_str());
        assert!(slow_mirror.1.as_millis() < 250);
        assert_eq!("/slower_mirror", slower_mirror.0.as_str());
        assert!(slower_mirror.1.as_millis() >= 250);
    }

    #[tokio::test]
    async fn it_handles_dead_mirrors() {
        let mirrors = vec!["/dead_mirror", "/mirror"];

        let mirror = SERVER.mock(|when, then| {
            when.method(GET).path("/mirror");
            then.status(200).body("");
        });

        let ranker = Ranker::new().unwrap();
        let living_mirrors = ranker.rank_mirrors(&mirrors).await.unwrap();

        mirror.assert();

        assert_eq!("/mirror", (&living_mirrors[0]).0.as_str());
    }
}
