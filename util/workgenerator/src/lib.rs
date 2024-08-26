mod request;
mod units;

use bon::builder;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, Exp};
pub use request::*;
use reqwest::{Client, Url};
use tokio::time::{Duration, Instant};
use tokio::{task, time};
pub use units::*;

#[derive(Debug, Clone)]
pub struct Config {
    pub url: Url,
    pub rps: u32,
    pub pct_post: f64,
    pub duration: Secs,
    pub nr_connections: usize,
}

async fn run(config: Config) -> Result<(), Error> {
    let mut handles = Vec::new();
    for _ in 0..config.nr_connections {
        let rps = (config.rps as f64 / config.nr_connections as f64).round();
        let lambda = (rps / 1e9).recip(); // requests_per_ns.recip()
        let workload = ConnWorkload::builder()
            .deltas(Exp::new(lambda).map_err(|_| Error::InvalidWorkload)?)
            .duration(Duration::from_secs(config.duration.into_inner()))
            .pct_post(config.pct_post)
            .build();
        handles.push(task::spawn(run_one_connection(workload)));
    }
    todo!()
}

#[builder]
#[derive(Debug)]
struct ConnWorkload {
    deltas: Exp<f64>,
    duration: Duration,
    pct_post: f64,
}

async fn run_one_connection(workload: ConnWorkload) -> Result<(), Error> {
    let client = Client::new();
    let mut handles = Vec::new();
    let now = Instant::now();
    let mut cur = now;
    let mut rng = StdRng::from_entropy();
    while cur - now < workload.duration {
        let delta = workload.deltas.sample(&mut rng).round() as u64;
        cur += Duration::from_nanos(delta);
        // Schedule the next request.
        let mut client = client.clone();
        let handle = task::spawn(async move {
            let mut interval = time::interval(cur - now);
            interval.tick().await; // ticks immediately
            interval.tick().await; // ticks after `cur - now`
            let now = Instant::now();
            // TODO: pick either GET or POST and generate a random request of that type
            todo!()
        });
        handles.push(handle);
        todo!()
    }

    todo!()
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid workload")]
    InvalidWorkload,
}
