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
    pub duration: Secs,
    pub nr_connections: usize,
    pub mix: RequestMix,
}

pub async fn run(config: Config) -> Result<Vec<Duration>, Error> {
    let mut handles = Vec::new();
    for _ in 0..config.nr_connections {
        let rps = (config.rps as f64 / config.nr_connections as f64).round();
        let lambda = (rps / 1e9).recip(); // requests_per_ns.recip()
        let workload = ConnWorkload::builder()
            .url(config.url.clone())
            .deltas(Exp::new(lambda).map_err(|_| Error::InvalidWorkload)?)
            .duration(Duration::from_secs(config.duration.into_inner()))
            .mix(config.mix)
            .build();
        handles.push(task::spawn(run_one_connection(workload)));
    }
    let mut latencies = Vec::new();
    for handle in handles {
        latencies.extend(handle.await??);
    }
    Ok(latencies)
}

#[builder]
#[derive(Debug)]
struct ConnWorkload {
    url: Url,
    deltas: Exp<f64>,
    duration: Duration,
    mix: RequestMix,
}

const HOST: &str = "onlineboutique.serviceweaver.dev";

async fn run_one_connection(workload: ConnWorkload) -> Result<Vec<Duration>, Error> {
    let client = Client::new();
    let mut handles = Vec::new();
    let now = Instant::now();
    let mut cur = now;
    let mut rng = StdRng::from_entropy();
    while cur - now < workload.duration {
        let delta = workload.deltas.sample(&mut rng).round() as u64;
        cur += Duration::from_nanos(delta);
        // Gather request data.
        let url = workload.url.clone();
        let client = client.clone();
        let send_fut = match workload.mix.sample(&mut rng) {
            RequestData::Home(form) => client.get(url).header("Host", HOST).form(&form).send(),
            RequestData::Product(not_a_form) => {
                let url = format!("{url}/{}", not_a_form.id);
                client.get(url).header("Host", HOST).send()
            }
            RequestData::ViewCart(form) => {
                let url = format!("{url}/cart");
                client.get(url).header("Host", HOST).form(&form).send()
            }
            RequestData::AddToCart(form) => {
                let url = format!("{url}/cart");
                client.post(url).header("Host", HOST).form(&form).send()
            }
            RequestData::EmptyCart(form) => {
                let url = format!("{url}/cart/empty");
                client.post(url).header("Host", HOST).form(&form).send()
            }
            RequestData::SetCurrency(form) => {
                let url = format!("{url}/setCurrency");
                client.post(url).header("Host", HOST).form(&form).send()
            }
            RequestData::Logout(form) => {
                let url = format!("{url}/logout");
                client.get(url).header("Host", HOST).form(&form).send()
            }
            RequestData::PlaceOrder(form) => {
                let url = format!("{url}/cart/checkout");
                client.post(url).header("Host", HOST).form(&form).send()
            }
        };
        // Schedule the next request.
        let handle = task::spawn(async move {
            let mut interval = time::interval(cur - now);
            interval.tick().await; // ticks immediately
            interval.tick().await; // ticks after `cur - now`
            let now = Instant::now();
            let _ = send_fut.await?.error_for_status()?;
            let elapsed = now.elapsed();
            Result::<_, Error>::Ok(elapsed)
        });
        handles.push(handle);
    }
    let mut latencies = Vec::new();
    for handle in handles {
        let elapsed = handle.await??;
        latencies.push(elapsed);
    }
    Ok(latencies)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid workload")]
    InvalidWorkload,

    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Tokio join error: {0}")]
    Join(#[from] tokio::task::JoinError),
}
