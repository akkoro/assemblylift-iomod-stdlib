use std::fmt;

use http::header::{HeaderMap, HeaderName, HeaderValue};
use hyper;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use serde::export::Formatter;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientError {
    pub why: String,
}
impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}
impl std::error::Error for ClientError {}

pub struct Client {
    service: &'static str,
    region: &'static str,
}

impl Client {
    pub fn new(service: &'static str, region: &'static str) -> Self {
        Self {
            service,
            region
        }
    }

    pub fn call<T: Serialize>(&self, method: &str, path: &str) -> Result<T, ClientError> {
        Ok(())
    }
}
