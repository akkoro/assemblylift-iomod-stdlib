use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use http::header::{HeaderMap, HeaderName, HeaderValue};
use hyper;
use hyper::StatusCode;
use hyper_tls::HttpsConnector;
use rusoto_signature::credential::AwsCredentials;
use rusoto_signature::{Region, SignedRequest};
use serde::de::DeserializeOwned;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use xml;
use xml::reader::{EventReader, ParserConfig};

use guest::xml_util;
use guest::xml_util::util::{Next, XmlParseError, XmlResponse};

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
pub type XmlDeserializer<'a, T> = fn(&str, &mut XmlResponse<'a>) -> Result<T, XmlParseError>;

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

    pub async fn call<'a, T: DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        protocol: &str,
        input: ClientInput,
        deserialize: Option<XmlDeserializer<'a, T>>,
    ) -> Result<T, ClientError> {
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
        match self.client.request(request).await {
            Ok(resp) => {
                let status = resp.status();
                let body = &*hyper::body::to_bytes(resp.into_body()).await.unwrap();

                match status {
                    StatusCode::OK => {
                        let body = Vec::from(body);
                        match protocol {
                            "json" => serde_json::from_slice(body.as_slice()).unwrap(),
                            "rest-xml" => {
                                // TODO fix lifetime of body
                                let reader = EventReader::new_with_config(
                                    body.as_slice(),
                                    ParserConfig::new().trim_whitespace(false),
                                );
                                let mut stack =
                                    xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                                let _start_document = stack.next();
                                let actual_tag_name =
                                    xml_util::util::peek_at_name(&mut stack).unwrap();
                                match deserialize.unwrap()(&actual_tag_name, &mut stack) {
                                    Ok(response) => Ok(response),
                                    _ => panic!("unhandled branch"),
                                }
                            }
                        }
                    }
                    status => {
                        let data = String::from(std::str::from_utf8(body).unwrap());
                        Err(ClientError {
                            why: String::from(status.canonical_reason().unwrap()),
                            data,
                        })
                    }
                }
            }
            Err(err) => Err(ClientError {
                why: err.to_string(),
                data: Default::default(),
            }),
        }
    }
}
