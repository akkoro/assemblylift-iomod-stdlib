use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use http::header::{HeaderMap, HeaderName, HeaderValue};
use hyper;
use hyper::{Request, Response, StatusCode};
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

    // pub async fn call<'a, T: DeserializeOwned>(
    pub async fn call(
        &self,
        method: &str,
        path: &str,
        protocol: &str,
        input: ClientInput,
        // deserialize: Option<XmlDeserializer<'a, T>>,
    // ) -> Result<T, ClientError> {
    ) -> Result<(StatusCode, String), ClientError> {
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
            Ok(resp) => {
                Ok(resp)
                // let status = resp.0;
                // let body = resp.1;

                // FIXME probably don't do serde here -- just return the status,body tuple
                // match status {
                //     StatusCode::OK => match protocol {
                //         "json" => serde_json::from_slice(body.as_ref()).unwrap(),
                //         "rest-xml" => {
                //             let mut stack = Client::make_stack(body);
                //             let _start_document = stack.next();
                //             let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                //             match deserialize.unwrap()(&actual_tag_name, &mut stack) {
                //                 Ok(response) => Ok(response),
                //                 _ => panic!("unhandled branch"),
                //             }
                //         }
                //         _ => panic!("unknown client protocol"),
                //     },
                //     status => Err(ClientError {
                //         why: String::from(status.canonical_reason().unwrap()),
                //         data: body,
                //     }),
                // }
            }
            Err(err) => Err(ClientError {
                why: err.to_string(),
                data: Default::default(),
            }),
        }
    }

    async fn call_internal(
        &self,
        request: Request<hyper::Body>,
    ) -> Result<(StatusCode, String), ClientError> {
        match self.client.request(request).await {
            Ok(resp) => {
                let status = resp.status();
                let body = &*hyper::body::to_bytes(resp.into_body()).await.unwrap();
                let body = String::from(std::str::from_utf8(body).unwrap());
                Ok((status, body))
            }
            Err(err) => Err(ClientError {
                why: err.to_string(),
                data: Default::default(),
            }),
        }
    }

    // fn make_stack<'a>(body: String) -> XmlResponse<'a> {
    //     let reader =
    //         EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
    //     xml_util::util::XmlResponse::new(reader.into_iter().peekable())
    // }
}
