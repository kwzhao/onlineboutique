mod request;
mod units;

use bon::builder;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, Exp};
pub use request::*;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{Duration, Instant};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::StreamExt;
pub use units::*;

#[builder]
#[derive(Debug, Clone)]
pub struct Config {
    pub url: Url,
    pub rps: u32,
    pub duration: Secs,
    pub nr_conns: usize,
    pub mix: RequestMix,
}

pub async fn run(config: &Config) -> Result<Vec<Record>, Error> {
    let mut handles = Vec::new();
    for _ in 0..config.nr_conns {
        let rps = (config.rps as f64 / config.nr_conns as f64).round();
        let lambda = rps / 1e9; // requests per nanosecond
        let workload = ConnWorkload::builder()
            .url(config.url.clone())
            .deltas(Exp::new(lambda).map_err(|_| Error::InvalidWorkload)?)
            .duration(Duration::from_secs(config.duration.into_inner()))
            .mix(config.mix)
            .build();
        handles.push(task::spawn(run_one_connection(workload)));
    }
    let mut records = Vec::new();
    for handle in handles {
        records.extend(handle.await??);
    }
    Ok(records)
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

async fn run_one_connection(workload: ConnWorkload) -> Result<Vec<Record>, Error> {
    let client = Client::new();
    let mut now = Instant::now();
    let end = now + workload.duration;
    let (tx, rx) = mpsc::unbounded_channel();
    let mut stream = UnboundedReceiverStream::new(rx);
    let mut rng = StdRng::from_entropy();
    tokio::spawn(async move {
        while now < end {
            // Wait until the next RPC time.
            let delta = workload.deltas.sample(&mut rng).round() as u64;
            let timer = async_timer::new_timer(Duration::from_nanos(delta));
            timer.await;

            // Gather request data.
            let url = workload.url.clone();
            let client = client.clone();
            let send_fut = match workload.mix.sample(&mut rng) {
                RequestData::Home(form) => client.get(url).header("Host", HOST).form(&form).send(),
                RequestData::Product(not_a_form) => {
                    let url = format!("{url}{}", not_a_form.id);
                    client.get(url).header("Host", HOST).send()
                }
                RequestData::ViewCart(form) => {
                    let url = format!("{url}cart");
                    client.get(url).header("Host", HOST).form(&form).send()
                }
                RequestData::AddToCart(form) => {
                    let url = format!("{url}cart");
                    client.post(url).header("Host", HOST).form(&form).send()
                }
                RequestData::EmptyCart(form) => {
                    let url = format!("{url}cart/empty");
                    client
                        .post(url)
                        .header("Host", HOST)
                        .header("Content-Length", "0") // required
                        .form(&form)
                        .send()
                }
                RequestData::SetCurrency(form) => {
                    let url = format!("{url}setCurrency");
                    client.post(url).header("Host", HOST).form(&form).send()
                }
                RequestData::Logout(form) => {
                    let url = format!("{url}logout");
                    client.get(url).header("Host", HOST).form(&form).send()
                }
                RequestData::PlaceOrder(form) => {
                    let url = format!("{url}cart/checkout");
                    client.post(url).header("Host", HOST).form(&form).send()
                }
            };

            // Send the next request.
            let tx = tx.clone();
            task::spawn(async move {
                let now = Instant::now();
                let _ = send_fut.await?.error_for_status()?;
                let elapsed = now.elapsed();
                let latency = Microsecs::new(elapsed.as_micros());
                let _ = tx.send(Token::Latency(latency));
                Result::<_, Error>::Ok(())
            });
            now = Instant::now();
        }
        tx.send(Token::Stop).unwrap();
    });
    let mut records = Vec::new();
    while let Some(Token::Latency(latency)) = stream.next().await {
        records.push(Record { latency })
    }
    Ok(records)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub latency: Microsecs,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Latency(Microsecs),
    Stop,
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
