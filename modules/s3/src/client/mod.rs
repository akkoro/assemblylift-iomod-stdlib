use std::fmt;

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
    client: HyperClient,
    aws_key: Option<(String, String, Option<String>)>,
}

impl Client {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);

        Self {
            client,
            aws_key: None,
        }
    }

    pub fn set_credentials(&mut self, id: String, key: String, token: Option<String>) {
        self.aws_key = Some((id, key, token));
    }

    pub async fn call(&self, mut request: SignedRequest) -> Result<Response<String>, ClientError> {
        let mut headers = HeaderMap::new();

        if let Some(key) = &self.aws_key {
            let token = key.2.clone();
            request.sign(&AwsCredentials::new(&key.0, &key.1, token, None));
        }

        for h in request.headers().iter() {
            let name = h.0.parse::<HeaderName>().unwrap();
            for v in h.1.iter() {
                let value = HeaderValue::from_bytes(v).unwrap();
                headers.append(&name, value);
            }
        }
        headers.insert("user-agent", HeaderValue::from_str("AssemblyLift").unwrap());

        for (h, v) in headers.iter() {
            println!("{}:{:?}", h.as_str(), v);
        }

        let mut final_uri = format!(
            "{}://{}{}",
            request.scheme(),
            request.hostname(),
            request.canonical_path()
        );
        if !request.canonical_query_string().is_empty() {
            final_uri = final_uri + &format!("?{}", request.canonical_query_string());
        }

        let mut http_req = hyper::Request::builder()
            .method(request.method())
            .uri(final_uri)
            .body(match request.payload {
                Some(payload) => payload.into_body(),
                None => hyper::Body::empty(),
            })
            .unwrap();
        *http_req.headers_mut() = headers;

        match self.call_internal(http_req).await {
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
