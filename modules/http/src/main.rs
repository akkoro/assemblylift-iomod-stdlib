mod client;

use std::str::FromStr;

use assemblylift_core_iomod::iomod;
use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use rusoto_signature::{Region, SignedRequest};

use guest::structs::*;

static CLIENT: Lazy<client::Client> = Lazy::new(|| {
    use std::env;
    let mut c = client::Client::new();
    c.set_credentials(
        env::var("AWS_ACCESS_KEY_ID").unwrap(),
        env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
        match env::var("AWS_SESSION_TOKEN") {
            Ok(token) => Some(token),
            Err(_) => None,
        },
    );
    c
});

#[tokio::main]
async fn main() {
    iomod!(akkoro.std.http => {
        request => request,
    });
}

pub fn request(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: HttpRequest = serde_json::from_slice(input.as_slice()).unwrap();

    let mut http_request = SignedRequest::new(
        &deserialized.method.clone(),
        "execute-api",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &deserialized.path.clone(),
    );
    http_request.set_hostname(Some(deserialized.host.clone()));

    Box::pin(async move {
        match crate::CLIENT.call(http_request, deserialized.auth.as_ref()).await {
            Ok(response) => {
                let status = response.status();
                match status {
                    status => {
                        let code = status.as_u16();
                        let headers = response
                                .headers()
                                .iter()
                                .map(|(k,v)| (String::from(k.as_str()), String::from(v.to_str().unwrap())))
                                .collect();
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let res = HttpResponse {
                            code,
                            headers,
                            body: match body.len() == 0 {
                                true => None,
                                false => Some(Vec::from(body)),
                            }, 
                        };
                        serde_json::to_vec(&Result::<HttpResponse, guest::Error>::Ok(res)).unwrap()
                    }
                }
            }
            Err(why) => {
                serde_json::to_vec(&Result::<HttpResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                .unwrap()
            }
        }
    })
}
