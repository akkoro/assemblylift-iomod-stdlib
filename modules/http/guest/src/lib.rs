pub mod structs;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use assemblylift_core_iomod_guest::{call, iomod};
use serde::{Deserialize, Serialize};

use crate::structs::*;

iomod!(akkoro.std.http);

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub why: String,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}
impl std::error::Error for Error {}

call!(request, HttpRequest => Result<HttpResponse, Error>);

pub struct HttpRequestBuilder {
    req: HttpRequest,
}

impl HttpRequestBuilder {
    pub fn new() -> Self {
        Self {
            req: Default::default(),
        }
    }

    pub fn method(mut self, method: &str) -> HttpRequestBuilder {
        self.req.method = method.into();
        self
    }

    pub fn host(mut self, host: &str) -> HttpRequestBuilder {
        self.req.host = host.into();
        self
    }

    pub fn path(mut self, path: &str) -> HttpRequestBuilder {
        self.req.path = path.into();
        self
    }

    pub fn content_type(mut self, content_type: &str) -> HttpRequestBuilder {
        self.req.content_type = Some(content_type.into());
        self
    }

    pub fn header(mut self, name: &str, value: &str) -> HttpRequestBuilder {
        if self.req.headers.is_none() {
            self.req.headers = Some(HashMap::new());
        }
        self.req
            .headers
            .as_mut()
            .unwrap()
            .insert(name.into(), value.into());
        self
    }

    pub fn body(mut self, body: &str) -> HttpRequestBuilder {
        self.req.body = Some(body.into());
        self
    }

    pub fn auth(mut self, r#type: &str, options: HashMap<&str, String>) -> HttpRequestBuilder {
        self.req.auth = Some(HttpAuth {
            r#type: r#type.to_string(),
            service: match options.get("service") {
                Some(s) => Some(s.clone()),
                None => Some("execute-api".to_string()),
            },
        });
        self
    }

    pub fn build(self) -> HttpRequest {
        self.req
    }
}
