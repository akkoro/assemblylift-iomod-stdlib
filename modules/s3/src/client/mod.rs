use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use http::header::{HeaderMap, HeaderName, HeaderValue};
use hyper;
use hyper::{Request, Response, StatusCode};
use hyper_tls::HttpsConnector;
use rusoto_signature::credential::AwsCredentials;
use rusoto_signature::{Region, SignedRequest};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientError {
    pub why: String,
    pub data: String,
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
    aws_key: Option<(String, String, Option<String>)>,
}

pub struct ClientInput {
    pub body: HashMap<&'static str, String>,
    pub headers: HashMap<&'static str, String>,
}

impl Client {
    pub fn new(service: String, region: String) -> Self {
        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);

        Self {
            service,
            region,
            client,
            aws_key: None,
        }
    }

    pub fn set_credentials(&mut self, id: String, key: String, token: Option<String>) {
        self.aws_key = Some((id, key, token));
    }

    pub async fn call(
        &self,
        method: &str,
        path: &str,
        protocol: &str,
        input: ClientInput,
    ) -> Result<Response<String>, ClientError> {
        let mut aws_req = SignedRequest::new(
            method,
            &self.service,
            &Region::from_str(&self.region).unwrap_or(Region::UsEast1),
            path,
        );
        if let Some(key) = &self.aws_key {
            let token = key.2.clone();
            aws_req.sign(&AwsCredentials::new(&key.0, &key.1, token, None));
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
            "json" => serde_json::to_string(&input.body).unwrap(),
            "rest-xml" => serde_xml_rs::to_string(&input.body).unwrap(),
            _ => panic!("unknown client protocol"),
        };

        let request = http_req.body(hyper::Body::from(body)).unwrap();
        match self.call_internal(request).await {
            Ok(resp) => Ok(resp),
            Err(err) => Err(ClientError {
                why: err.to_string(),
                data: Default::default(),
            }),
        }
    }

    async fn call_internal(
        &self,
        request: Request<hyper::Body>,
    ) -> Result<Response<String>, ClientError> {
        match self.client.request(request).await {
            Ok(resp) => {
                let status = resp.status();
                let mut headers = HeaderMap::new();
                for h in resp.headers().iter() {
                    let name = &*h.0;
                    let value = &*h.1;
                    headers.append(name, HeaderValue::from(value));
                }
                let body = &*hyper::body::to_bytes(resp.into_body()).await.unwrap();
                let body = String::from(std::str::from_utf8(body).unwrap());
                let mut response = Response::builder()
                    .status(status)
                    .body(body)
                    .unwrap();
                *response.headers_mut() = headers;
                Ok(response)
            }
            Err(err) => Err(ClientError {
                why: err.to_string(),
                data: Default::default(),
            }),
        }
    }
}
