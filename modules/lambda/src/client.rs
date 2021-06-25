use std::fmt;
use std::fmt::Formatter;

use http::header::{HeaderMap, HeaderName, HeaderValue};
use hyper::Response;
//use hyper;
//use hyper::Response;
use hyper_tls::HttpsConnector;
use rusoto_signature::credential::AwsCredentials;
use rusoto_signature::SignedRequest;

use serde::{Deserialize, Serialize};

use hyper::body::Bytes;
use reqwest;

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

// pub type HyperClient = client::Client<hyper_rustls::HttpsConnector<client::HttpConnector>>;
pub type HyperClient = hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;

pub struct Client {
    client: HyperClient,
    aws_key: Option<(String, String, Option<String>)>,
}

impl Client {
    pub fn new() -> Self {
        // let https = hyper_rustls::HttpsConnector::with_native_roots();
        // let client = client::Client::builder().build::<_, hyper::Body>(https);
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

    pub async fn call(&self, mut request: SignedRequest) -> Result<Response<Bytes>, ClientError> {
        if let Some(key) = &self.aws_key {
            let token = key.2.clone();
            request.sign(&AwsCredentials::new(&key.0, &key.1, token, None));
        }

        let mut headers = HeaderMap::new();
        for h in request.headers().iter() {
            let name = h.0.parse::<HeaderName>().unwrap();
            for v in h.1.iter() {
                let value = HeaderValue::from_bytes(v).unwrap();
                headers.append(&name, value);
            }
        }
        headers.insert("user-agent", HeaderValue::from_str("AssemblyLift").unwrap());

        let mut final_uri = format!(
            "{}://{}{}",
            request.scheme(),
            request.hostname(),
            request.canonical_path()
        );
        if !request.canonical_query_string().is_empty() {
            final_uri = final_uri + &format!("?{}", request.canonical_query_string());
        }

        let mut client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        let method = request.method();
        match method {
            "GET" => {
                let response = client.get(final_uri).send().await;
                match response {
                    Ok(res) => Ok(Response::builder()
                        .status(res.status())
                        .body(res.bytes().await.unwrap())
                        .unwrap()),
                    Err(why) => Err(ClientError {
                        why: why.to_string(),
                        data: Default::default(),
                    }),
                }
            }
            "POST" => {
                let response = client
                    .post(final_uri)
                    .body(match request.payload {
                        Some(payload) => {
                            hyper::body::to_bytes(payload.into_body()).await.unwrap()
                            // std::str::from_utf8(&*bytes).unwrap()
                        }
                        None => Bytes::new(),
                    })
                    .send()
                    .await;

                println!("DEBUG: response={:?}", response);
                match response {
                    Ok(res) => Ok(Response::builder()
                        .status(res.status())
                        .body(res.bytes().await.unwrap())
                        .unwrap()),
                    Err(why) => Err(ClientError {
                        why: why.to_string(),
                        data: Default::default(),
                    }),
                }
            }
            m => Err(ClientError {
                why: format!("Unknown HTTP method {}", m),
                data: Default::default(),
            }),
        }
    }
}
