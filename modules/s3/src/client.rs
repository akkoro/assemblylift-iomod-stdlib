use std::fmt;
use std::str::FromStr;

use http::header::{HeaderMap, HeaderName, HeaderValue};
use hyper;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use serde::export::Formatter;
use rusoto_signature::{Region, SignedRequest};
use rusoto_signature::credential::AwsCredentials;

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

pub type HyperClient = hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;

pub struct Client {
    service: String,
    region: String,
    client: HyperClient,
    aws_key: Option<(String, String)>,
}

impl Client {
    pub fn new(service: String, region: String) -> Self {
        let https = HttpsConnector::new();
        let client = hyper::Client::builder()
            .build::<_, hyper::Body>(https);

        Self {
            service,
            region,
            client,
            aws_key: None,
        }
    }

    pub fn set_credentials(&mut self, id: String, key: String) {
        self.aws_key = Some((id, key));
    }

    pub async fn call<T: Serialize>(&self, method: &str, path: &str, protocol: &str, input: T) -> Result<Vec<u8>, ClientError> {
        let mut aws_req = SignedRequest::new(
            method, 
            &self.service, 
            &Region::from_str(&self.region).unwrap_or(Region::UsEast1), 
            path,
        );
        if let Some(key) = &self.aws_key {
            aws_req.sign(&AwsCredentials::new(
                &key.0, 
                &key.1, 
                None, 
                None,
            ));
        }

        let mut headers = HeaderMap::new();
        for h in aws_req.headers().iter() {
            let name = h.0.parse::<HeaderName>().unwrap();
            for v in h.1.iter() {
                let value = HeaderValue::from_bytes(v).unwrap();
                headers.append(&name, value);
            }
        }
        headers.insert("user-agent", HeaderValue::from_str("AssemblyLift").unwrap());

        let mut final_uri = format!(
            "{}://{}{}",
            aws_req.scheme(),
            aws_req.hostname(),
            aws_req.canonical_path()
        );
        if !aws_req.canonical_query_string().is_empty() {
            final_uri = final_uri + &format!("?{}", aws_req.canonical_query_string());
        }

        let mut http_req = hyper::Request::builder().method(method).uri(final_uri);
        *http_req.headers_mut().unwrap() = headers;

        let body = match protocol {
            "json" => {
                serde_json::to_string(&input).unwrap()
            },
            "rest-xml" => {
                serde_xml_rs::to_string(&input).unwrap()
            },
            _ => panic!("unknown client protocol"),
        };

        match self.client.request(http_req.body(hyper::Body::from(body)).unwrap()).await {
            Ok(resp) => {
                let b = hyper::body::to_bytes(resp.into_body()).await.unwrap();
                Ok(Vec::from(&*b))
            },
            Err(err) => Err(ClientError { why: err.to_string() })
        }
        
    }
}
