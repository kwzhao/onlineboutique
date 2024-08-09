mod request;

pub use request::*;
use reqwest::Url;

#[derive(Debug)]
pub struct Config {
    pub url: Url,
    pub rps: u32,
    pub nr_connections: usize,
}

// Decide what a reasonable access pattern is.
