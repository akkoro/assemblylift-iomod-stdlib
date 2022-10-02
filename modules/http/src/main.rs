use std::io::Read;
use std::str::FromStr;

use assemblylift_core_iomod::{iomod, registry};
use clap::Parser;
use flate2::read::GzDecoder;
use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use rusoto_signature::{Region, SignedRequest};
use tracing::{debug, error, info, instrument, Level};
use tracing_subscriber::FmtSubscriber;

use guest::structs::*;

mod client;

static CLIENT: Lazy<client::Client> = Lazy::new(|| {
    use std::env;
    let mut c = client::Client::new();
    if env::var("AWS_ACCESS_KEY_ID").is_ok() {
        c.set_aws_credentials(
            env::var("AWS_ACCESS_KEY_ID").unwrap(),
            env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
            match env::var("AWS_SESSION_TOKEN") {
                Ok(token) => Some(token),
                Err(_) => None,
            },
        );
    }
    c
});

#[derive(Parser)]
struct Args {
    #[clap(long)]
    registry_ip: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let ip: String = match args.registry_ip.as_deref() {
        Some(ip) => ip.to_string(),
        None => {
            std::env::var("IOMOD_REGISTRY_ADDR")
                .unwrap_or("127.0.0.1".to_string())
        }
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    iomod!(ip, akkoro.std.http => {
        request => request,
    });
}

pub fn request(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    info!("request received");
    let deserialized: HttpRequest = match serde_json::from_slice(input.as_slice()) {
        Ok(r) => r,
        Err(e) => {
            error!("could not deserialize request: {:?}", e.to_string());
            Default::default()
        },
    };

    let default_service = String::from("execute-api");
    let method = deserialized.method.clone();
    let mut http_request = SignedRequest::new(
        &method.clone(),
        &*{
            match deserialized.auth {
                Some(ref auth) => auth.service.as_ref().unwrap_or(&default_service),
                None => &default_service,
            }
        },
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &deserialized.path.clone(),
    );

    for header in deserialized.headers.unwrap_or(Headers::default()) {
        http_request.add_header(&header.0, &header.1);
    }

    for query_param in deserialized.query_parameters.unwrap_or(QueryParameters::default()) {
        http_request.add_param(&query_param.0, &query_param.1);
    }

    http_request.set_hostname(Some(deserialized.host.clone()));
    if let Some(content_type) = deserialized.content_type {
        http_request.set_content_type(content_type.clone());
    }
    if !method.clone().eq("GET") {
        if let Some(body) = deserialized.body.clone() {
            http_request.set_payload(Some(body));
        }
    }

    Box::pin(async move {
        match CLIENT
            .call(http_request, deserialized.auth.clone())
            .await
        {
            Ok(response) => {
                let status = response.status();
                match status {
                    status => {
                        let code = status.as_u16();
                        let headers: Headers = response
                            .headers()
                            .iter()
                            .map(|(k, v)| {
                                (String::from(k.as_str()), String::from(v.to_str().unwrap()))
                            })
                            .collect();
                        let content_type: Option<&String> = headers.get("content-type");
                        let content_encoding: Option<&String> = headers.get("content-encoding");

                        info!("response content_type={:?}", content_type);
                        info!("response content_type={:?}", content_encoding);
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let res = HttpResponse {
                            code,
                            headers: headers.clone(),
                            body: match body.len() == 0 {
                                true => None,
                                false => {
                                    if let Some(encoding) = content_encoding {
                                        let enc = &*encoding.clone();
                                        match enc {
                                            "gzip" => {
                                                let mut out = String::new();
                                                let mut gzd = GzDecoder::new(body);
                                                gzd.read_to_string(&mut out).expect("could not decompress gzip-encoded body");
                                                Some(out)
                                            }
                                            _ => Some(String::from(std::str::from_utf8(body.as_ref()).unwrap()))
                                        }
                                    } else {
                                        Some(String::from(std::str::from_utf8(body.as_ref()).unwrap()))
                                    }
                                },
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
