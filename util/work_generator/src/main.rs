use std::path::PathBuf;

use clap::Parser;
use reqwest::Url;
use work_generator::{Config, RequestMix, Secs};

#[derive(Debug, Clone, Parser)]
pub struct Opt {
    #[arg(short, long)]
    pub url: Url,
    #[arg(short, long)]
    pub rps: u32,
    #[arg(short, long, default_value_t = Secs::new(10))]
    pub duration: Secs,
    #[arg(short, long, default_value_t = 10)]
    pub nr_conns: usize,
    #[arg(short, long, default_value = "mixes/default.json")]
    pub mix_path: PathBuf,
    #[arg(short, long)]
    pub out: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    let mix = std::fs::read_to_string(&opt.mix_path)?;
    let mix: RequestMix = serde_json::from_str(&mix)?;
    let config = Config::builder()
        .url(opt.url)
        .rps(opt.rps)
        .duration(opt.duration)
        .nr_conns(opt.nr_conns)
        .mix(mix)
        .build();
    let records = work_generator::run(&config).await?;
    let mut wtr = csv::Writer::from_path(&opt.out)?;
    for record in records {
        wtr.serialize(record)?;
    }
    Ok(())
}
