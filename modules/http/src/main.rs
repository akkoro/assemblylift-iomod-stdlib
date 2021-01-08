mod client;

use assemblylift_core_iomod::iomod;
use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use hyper::StatusCode;
use rusoto_signature::{Region, SignedRequest};

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
    });
}

