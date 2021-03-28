use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub type QueryParameters = HashMap<String, String>;
pub type Headers = HashMap<String, String>;
pub type Body = Vec<u8>;

#[derive(Serialize, Deserialize)]
pub struct HttpAuth {
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
pub struct HttpRequest {
    pub auth: Option<HttpAuth>,
    pub method: String,
    pub host: String,
    pub path: String,
    pub query_parameters: Option<QueryParameters>,
    pub content_type: String,
    pub headers: Option<Headers>,
    pub body: Option<Body>,
}

#[derive(Serialize, Deserialize)]
pub struct HttpResponse {
    pub code: u16,
    pub headers: Headers,
    pub body: Option<Body>,
}
