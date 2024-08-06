mod request;

pub use request::*;
use reqwest::Url;

// TODO: Define top-level configuration
#[derive(Debug)]
pub struct Config {
    pub url: Url,
}
