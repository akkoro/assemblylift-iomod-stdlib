// This file is generated!
// See https://github.com/akkoro/asml-aws-codegen

use std::str::FromStr;

use assemblylift_core_iomod::iomod;
use clap::Parser;
use futures::future::BoxFuture;
use hyper::StatusCode;
use once_cell::sync::Lazy;
use rusoto_signature::{Region, SignedRequest};
use tracing::field::debug;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

use guest::structs::*;

mod client;

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

    iomod!(ip, akkoro.aws.secretsmanager => {
      cancel_rotate_secret => cancel_rotate_secret,
      create_secret => create_secret,
      delete_resource_policy => delete_resource_policy,
      delete_secret => delete_secret,
      describe_secret => describe_secret,
      get_random_password => get_random_password,
      get_resource_policy => get_resource_policy,
      get_secret_value => get_secret_value,
      list_secret_version_ids => list_secret_version_ids,
      list_secrets => list_secrets,
      put_resource_policy => put_resource_policy,
      put_secret_value => put_secret_value,
      remove_regions_from_replication => remove_regions_from_replication,
      replicate_secret_to_regions => replicate_secret_to_regions,
      restore_secret => restore_secret,
      rotate_secret => rotate_secret,
      stop_replication_to_replica => stop_replication_to_replica,
      tag_resource => tag_resource,
      untag_resource => untag_resource,
      update_secret => update_secret,
      update_secret_version_stage => update_secret_version_stage,
      validate_resource_policy => validate_resource_policy,
    });
}

#[allow(dead_code)]
pub fn cancel_rotate_secret(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CancelRotateSecretRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __cancel_rotate_secret(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __cancel_rotate_secret(input: CancelRotateSecretRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.CancelRotateSecret");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: CancelRotateSecretResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: CancelRotateSecretResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;
                        output.version_id = body.version_id;

                        serde_json::to_vec(&Result::<CancelRotateSecretResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<CancelRotateSecretResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<CancelRotateSecretResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn create_secret(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateSecretRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_secret(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __create_secret(input: CreateSecretRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.CreateSecret");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: CreateSecretResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        debug!("body = {:?}", std::str::from_utf8(body).unwrap());
                        let body: CreateSecretResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;
                        output.version_id = body.version_id;
                        output.replication_status = body.replication_status;

                        serde_json::to_vec(&Result::<CreateSecretResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<CreateSecretResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<CreateSecretResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn delete_resource_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteResourcePolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_resource_policy(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_resource_policy(input: DeleteResourcePolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.DeleteResourcePolicy");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DeleteResourcePolicyResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: DeleteResourcePolicyResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;

                        serde_json::to_vec(&Result::<DeleteResourcePolicyResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DeleteResourcePolicyResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DeleteResourcePolicyResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn delete_secret(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteSecretRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_secret(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_secret(input: DeleteSecretRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.DeleteSecret");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DeleteSecretResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: DeleteSecretResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;
                        output.deletion_date = body.deletion_date;

                        serde_json::to_vec(&Result::<DeleteSecretResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DeleteSecretResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DeleteSecretResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_secret(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeSecretRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_secret(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_secret(input: DescribeSecretRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.DescribeSecret");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeSecretResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: DescribeSecretResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;
                        output.description = body.description;
                        output.kms_key_id = body.kms_key_id;
                        output.rotation_enabled = body.rotation_enabled;
                        output.rotation_lambda_arn = body.rotation_lambda_arn;
                        output.rotation_rules = body.rotation_rules;
                        output.last_rotated_date = body.last_rotated_date;
                        output.last_changed_date = body.last_changed_date;
                        output.last_accessed_date = body.last_accessed_date;
                        output.deleted_date = body.deleted_date;
                        output.tags = body.tags;
                        output.version_ids_to_stages = body.version_ids_to_stages;
                        output.owning_service = body.owning_service;
                        output.created_date = body.created_date;
                        output.primary_region = body.primary_region;
                        output.replication_status = body.replication_status;

                        serde_json::to_vec(&Result::<DescribeSecretResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeSecretResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeSecretResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_random_password(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetRandomPasswordRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_random_password(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_random_password(input: GetRandomPasswordRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.GetRandomPassword");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetRandomPasswordResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetRandomPasswordResponse = serde_json::from_slice(body).unwrap();
                        output.random_password = body.random_password;

                        serde_json::to_vec(&Result::<GetRandomPasswordResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetRandomPasswordResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetRandomPasswordResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_resource_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetResourcePolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_resource_policy(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_resource_policy(input: GetResourcePolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.GetResourcePolicy");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetResourcePolicyResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetResourcePolicyResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;
                        output.resource_policy = body.resource_policy;

                        serde_json::to_vec(&Result::<GetResourcePolicyResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetResourcePolicyResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetResourcePolicyResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_secret_value(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetSecretValueRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_secret_value(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_secret_value(input: GetSecretValueRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.GetSecretValue");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetSecretValueResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetSecretValueResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;
                        output.version_id = body.version_id;
                        output.secret_binary = body.secret_binary;
                        output.secret_string = body.secret_string;
                        output.version_stages = body.version_stages;
                        output.created_date = body.created_date;

                        serde_json::to_vec(&Result::<GetSecretValueResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetSecretValueResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetSecretValueResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_secret_version_ids(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListSecretVersionIdsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_secret_version_ids(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_secret_version_ids(input: ListSecretVersionIdsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.ListSecretVersionIds");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListSecretVersionIdsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListSecretVersionIdsResponse = serde_json::from_slice(body).unwrap();
                        output.versions = body.versions;
                        output.next_token = body.next_token;
                        output.arn = body.arn;
                        output.name = body.name;

                        serde_json::to_vec(&Result::<ListSecretVersionIdsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListSecretVersionIdsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListSecretVersionIdsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_secrets(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListSecretsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_secrets(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_secrets(input: ListSecretsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.ListSecrets");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListSecretsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListSecretsResponse = serde_json::from_slice(body).unwrap();
                        output.secret_list = body.secret_list;
                        output.next_token = body.next_token;

                        serde_json::to_vec(&Result::<ListSecretsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListSecretsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListSecretsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn put_resource_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutResourcePolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_resource_policy(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_resource_policy(input: PutResourcePolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.PutResourcePolicy");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: PutResourcePolicyResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: PutResourcePolicyResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;

                        serde_json::to_vec(&Result::<PutResourcePolicyResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PutResourcePolicyResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PutResourcePolicyResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn put_secret_value(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutSecretValueRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_secret_value(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_secret_value(input: PutSecretValueRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.PutSecretValue");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: PutSecretValueResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: PutSecretValueResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;
                        output.version_id = body.version_id;
                        output.version_stages = body.version_stages;

                        serde_json::to_vec(&Result::<PutSecretValueResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PutSecretValueResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PutSecretValueResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn remove_regions_from_replication(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: RemoveRegionsFromReplicationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __remove_regions_from_replication(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __remove_regions_from_replication(input: RemoveRegionsFromReplicationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.RemoveRegionsFromReplication");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: RemoveRegionsFromReplicationResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: RemoveRegionsFromReplicationResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.replication_status = body.replication_status;

                        serde_json::to_vec(&Result::<RemoveRegionsFromReplicationResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<RemoveRegionsFromReplicationResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<RemoveRegionsFromReplicationResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn replicate_secret_to_regions(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ReplicateSecretToRegionsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __replicate_secret_to_regions(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __replicate_secret_to_regions(input: ReplicateSecretToRegionsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.ReplicateSecretToRegions");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ReplicateSecretToRegionsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ReplicateSecretToRegionsResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.replication_status = body.replication_status;

                        serde_json::to_vec(&Result::<ReplicateSecretToRegionsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ReplicateSecretToRegionsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ReplicateSecretToRegionsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn restore_secret(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: RestoreSecretRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __restore_secret(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __restore_secret(input: RestoreSecretRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.RestoreSecret");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: RestoreSecretResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: RestoreSecretResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;

                        serde_json::to_vec(&Result::<RestoreSecretResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<RestoreSecretResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<RestoreSecretResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn rotate_secret(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: RotateSecretRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __rotate_secret(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __rotate_secret(input: RotateSecretRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.RotateSecret");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: RotateSecretResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: RotateSecretResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;
                        output.version_id = body.version_id;

                        serde_json::to_vec(&Result::<RotateSecretResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<RotateSecretResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<RotateSecretResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn stop_replication_to_replica(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: StopReplicationToReplicaRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __stop_replication_to_replica(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __stop_replication_to_replica(input: StopReplicationToReplicaRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.StopReplicationToReplica");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: StopReplicationToReplicaResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: StopReplicationToReplicaResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;

                        serde_json::to_vec(&Result::<StopReplicationToReplicaResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<StopReplicationToReplicaResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<StopReplicationToReplicaResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn tag_resource(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: TagResourceRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __tag_resource(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __tag_resource(input: TagResourceRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.TagResource");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn untag_resource(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UntagResourceRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __untag_resource(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __untag_resource(input: UntagResourceRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.UntagResource");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_secret(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateSecretRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __update_secret(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_secret(input: UpdateSecretRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.UpdateSecret");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateSecretResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: UpdateSecretResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;
                        output.version_id = body.version_id;

                        serde_json::to_vec(&Result::<UpdateSecretResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateSecretResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateSecretResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_secret_version_stage(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateSecretVersionStageRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __update_secret_version_stage(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_secret_version_stage(input: UpdateSecretVersionStageRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.UpdateSecretVersionStage");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateSecretVersionStageResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: UpdateSecretVersionStageResponse = serde_json::from_slice(body).unwrap();
                        output.arn = body.arn;
                        output.name = body.name;

                        serde_json::to_vec(&Result::<UpdateSecretVersionStageResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateSecretVersionStageResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateSecretVersionStageResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn validate_resource_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ValidateResourcePolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __validate_resource_policy(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __validate_resource_policy(input: ValidateResourcePolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "secretsmanager",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );

    if path_params.len() > 0 {
        match path_params.find('=') {
            None => http_request.add_param(path_params, "true".to_string()),
            Some(_) => {
                let pairs: Vec<&str> = path_params.split('=').collect();
                for idx in (0..pairs.len()).step_by(2) {
                    http_request.add_param(pairs[idx], pairs[idx + 1]);
                }
            }
        }
    }

    http_request.set_payload(Some(serde_json::to_string(&input).unwrap()));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "secretsmanager.ValidateResourcePolicy");
    http_request.set_content_type(String::from("application/x-amz-json-1.1"));


    Box::pin(async move {
        match CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ValidateResourcePolicyResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ValidateResourcePolicyResponse = serde_json::from_slice(body).unwrap();
                        output.policy_validation_passed = body.policy_validation_passed;
                        output.validation_errors = body.validation_errors;

                        serde_json::to_vec(&Result::<ValidateResourcePolicyResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ValidateResourcePolicyResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ValidateResourcePolicyResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}
