use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub type QueryParameters = HashMap<String, String>;
pub type Headers = HashMap<String, String>;
pub type Body = String;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct HttpAuth {
    pub r#type: String,
    pub service: Option<String>,
}

// TODO is_base64_encoded?
#[derive(Serialize, Deserialize, Default)]
pub struct HttpRequest {
    pub auth: Option<HttpAuth>,
    pub method: String,
    pub host: String,
    pub path: String,
    pub query_parameters: Option<QueryParameters>,
    pub content_type: Option<String>,
    pub headers: Option<Headers>,
    pub body: Option<Body>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct HttpResponse {
    pub code: u16,
    pub headers: Headers,
    pub body: Option<Body>,
}
