use std::fmt;

use http::header::{HeaderMap, HeaderName, HeaderValue};
use hyper::client;
use hyper::Response;
use rusoto_signature::credential::AwsCredentials;
use rusoto_signature::SignedRequest;
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

pub type HyperClient = client::Client<hyper_rustls::HttpsConnector<client::HttpConnector>>;

pub struct Client {
    client: HyperClient,
    aws_key: Option<(String, String, Option<String>)>,
}

impl Client {
    pub fn new() -> Self {
        let https = hyper_rustls::HttpsConnector::with_native_roots();
        let client = client::Client::builder().build::<_, hyper::Body>(https);

        Self {
            client,
            aws_key: None,
        }
    }

    pub fn set_credentials(&mut self, id: String, key: String, token: Option<String>) {
        self.aws_key = Some((id, key, token));
    }

    pub async fn call(&self, mut request: SignedRequest) -> Result<Response<hyper::Body>, ClientError> {
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

        let mut http_req = hyper::Request::builder()
            .method(request.method())
            .uri(final_uri)
            .body(match request.payload {
                Some(payload) => payload.into_body(),
                None => hyper::Body::empty(),
            })
            .unwrap();
        *http_req.headers_mut() = headers;

        println!("request: {:?}", http_req);
        match self.client.request(http_req).await {
            Ok(resp) => Ok(resp),
            Err(err) => Err(ClientError {
                why: format!("{:?}", err),
                data: Default::default(),
            }),
        }
    }
}
