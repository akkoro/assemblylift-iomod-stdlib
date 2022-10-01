// This file is generated!
// See https://github.com/akkoro/asml-aws-codegen

use std::str::FromStr;

use assemblylift_core_iomod::iomod;
use clap::Parser;
use futures::future::BoxFuture;
use hyper::StatusCode;
use once_cell::sync::Lazy;
use rusoto_signature::{Region, SignedRequest};
use tracing::Level;
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
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    iomod!(ip, akkoro.aws.lambda => {
      add_layer_version_permission => add_layer_version_permission,
      add_permission => add_permission,
      create_alias => create_alias,
      create_code_signing_config => create_code_signing_config,
      create_event_source_mapping => create_event_source_mapping,
      create_function => create_function,
      create_function_url_config => create_function_url_config,
      delete_alias => delete_alias,
      delete_code_signing_config => delete_code_signing_config,
      delete_event_source_mapping => delete_event_source_mapping,
      delete_function => delete_function,
      delete_function_code_signing_config => delete_function_code_signing_config,
      delete_function_concurrency => delete_function_concurrency,
      delete_function_event_invoke_config => delete_function_event_invoke_config,
      delete_function_url_config => delete_function_url_config,
      delete_layer_version => delete_layer_version,
      delete_provisioned_concurrency_config => delete_provisioned_concurrency_config,
      get_account_settings => get_account_settings,
      get_alias => get_alias,
      get_code_signing_config => get_code_signing_config,
      get_event_source_mapping => get_event_source_mapping,
      get_function => get_function,
      get_function_code_signing_config => get_function_code_signing_config,
      get_function_concurrency => get_function_concurrency,
      get_function_configuration => get_function_configuration,
      get_function_event_invoke_config => get_function_event_invoke_config,
      get_function_url_config => get_function_url_config,
      get_layer_version => get_layer_version,
      get_layer_version_by_arn => get_layer_version_by_arn,
      get_layer_version_policy => get_layer_version_policy,
      get_policy => get_policy,
      get_provisioned_concurrency_config => get_provisioned_concurrency_config,
      invoke => invoke,
      invoke_async => invoke_async,
      list_aliases => list_aliases,
      list_code_signing_configs => list_code_signing_configs,
      list_event_source_mappings => list_event_source_mappings,
      list_function_event_invoke_configs => list_function_event_invoke_configs,
      list_function_url_configs => list_function_url_configs,
      list_functions => list_functions,
      list_functions_by_code_signing_config => list_functions_by_code_signing_config,
      list_layer_versions => list_layer_versions,
      list_layers => list_layers,
      list_provisioned_concurrency_configs => list_provisioned_concurrency_configs,
      list_tags => list_tags,
      list_versions_by_function => list_versions_by_function,
      publish_layer_version => publish_layer_version,
      publish_version => publish_version,
      put_function_code_signing_config => put_function_code_signing_config,
      put_function_concurrency => put_function_concurrency,
      put_function_event_invoke_config => put_function_event_invoke_config,
      put_provisioned_concurrency_config => put_provisioned_concurrency_config,
      remove_layer_version_permission => remove_layer_version_permission,
      remove_permission => remove_permission,
      tag_resource => tag_resource,
      untag_resource => untag_resource,
      update_alias => update_alias,
      update_code_signing_config => update_code_signing_config,
      update_event_source_mapping => update_event_source_mapping,
      update_function_code => update_function_code,
      update_function_configuration => update_function_configuration,
      update_function_event_invoke_config => update_function_event_invoke_config,
      update_function_url_config => update_function_url_config,
    });
}

#[allow(dead_code)]
pub fn add_layer_version_permission(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: AddLayerVersionPermissionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __add_layer_version_permission(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __add_layer_version_permission(input: AddLayerVersionPermissionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2018-10-31/layers/{LayerName}/versions/{VersionNumber}/policy");
    path = match path.find("{LayerName}") {
        Some(_) => path.replace("{LayerName}", &input.layer_name.to_string()),
        None => path.to_string(),
    };
    path = match path.find("{VersionNumber}") {
        Some(_) => path.replace("{VersionNumber}", &input.version_number.to_string()),
        None => path.to_string(),
    };

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
        "lambda",
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
    http_request.add_header("x-amz-target", "AddLayerVersionPermission");
    http_request.set_content_type(String::from("application/json"));

    if let Some(revision_id) = input.revision_id {
        http_request.add_param("RevisionId", &revision_id);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: AddLayerVersionPermissionResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: AddLayerVersionPermissionResponse = serde_json::from_slice(body).unwrap();
                        output.statement = body.statement;
                        output.revision_id = body.revision_id;

                        serde_json::to_vec(&Result::<AddLayerVersionPermissionResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<AddLayerVersionPermissionResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<AddLayerVersionPermissionResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn add_permission(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: AddPermissionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __add_permission(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __add_permission(input: AddPermissionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/policy");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

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
        "lambda",
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
    http_request.add_header("x-amz-target", "AddPermission");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: AddPermissionResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: AddPermissionResponse = serde_json::from_slice(body).unwrap();
                        output.statement = body.statement;

                        serde_json::to_vec(&Result::<AddPermissionResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<AddPermissionResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<AddPermissionResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn create_alias(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateAliasRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_alias(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __create_alias(input: CreateAliasRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/aliases");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

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
        "lambda",
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
    http_request.add_header("x-amz-target", "CreateAlias");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: AliasConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: AliasConfiguration = serde_json::from_slice(body).unwrap();
                        output.alias_arn = body.alias_arn;
                        output.name = body.name;
                        output.function_version = body.function_version;
                        output.description = body.description;
                        output.routing_config = body.routing_config;
                        output.revision_id = body.revision_id;

                        serde_json::to_vec(&Result::<AliasConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<AliasConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<AliasConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn create_code_signing_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateCodeSigningConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_code_signing_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __create_code_signing_config(input: CreateCodeSigningConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2020-04-22/code-signing-configs/");

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
        "lambda",
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
    http_request.add_header("x-amz-target", "CreateCodeSigningConfig");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: CreateCodeSigningConfigResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: CreateCodeSigningConfigResponse = serde_json::from_slice(body).unwrap();
                        output.code_signing_config = body.code_signing_config;

                        serde_json::to_vec(&Result::<CreateCodeSigningConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<CreateCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<CreateCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn create_event_source_mapping(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateEventSourceMappingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_event_source_mapping(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __create_event_source_mapping(input: CreateEventSourceMappingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/event-source-mappings/");

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
        "lambda",
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
    http_request.add_header("x-amz-target", "CreateEventSourceMapping");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: EventSourceMappingConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: EventSourceMappingConfiguration = serde_json::from_slice(body).unwrap();
                        output.uuid = body.uuid;
                        output.starting_position = body.starting_position;
                        output.starting_position_timestamp = body.starting_position_timestamp;
                        output.batch_size = body.batch_size;
                        output.maximum_batching_window_in_seconds = body.maximum_batching_window_in_seconds;
                        output.parallelization_factor = body.parallelization_factor;
                        output.event_source_arn = body.event_source_arn;
                        output.filter_criteria = body.filter_criteria;
                        output.function_arn = body.function_arn;
                        output.last_modified = body.last_modified;
                        output.last_processing_result = body.last_processing_result;
                        output.state = body.state;
                        output.state_transition_reason = body.state_transition_reason;
                        output.destination_config = body.destination_config;
                        output.topics = body.topics;
                        output.queues = body.queues;
                        output.source_access_configurations = body.source_access_configurations;
                        output.self_managed_event_source = body.self_managed_event_source;
                        output.maximum_record_age_in_seconds = body.maximum_record_age_in_seconds;
                        output.bisect_batch_on_function_error = body.bisect_batch_on_function_error;
                        output.maximum_retry_attempts = body.maximum_retry_attempts;
                        output.tumbling_window_in_seconds = body.tumbling_window_in_seconds;
                        output.function_response_types = body.function_response_types;
                        output.amazon_managed_kafka_event_source_config = body.amazon_managed_kafka_event_source_config;
                        output.self_managed_kafka_event_source_config = body.self_managed_kafka_event_source_config;

                        serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn create_function(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateFunctionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_function(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __create_function(input: CreateFunctionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions");

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
        "lambda",
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
    http_request.add_header("x-amz-target", "CreateFunction");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: FunctionConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: FunctionConfiguration = serde_json::from_slice(body).unwrap();
                        output.function_name = body.function_name;
                        output.function_arn = body.function_arn;
                        output.runtime = body.runtime;
                        output.role = body.role;
                        output.handler = body.handler;
                        output.code_size = body.code_size;
                        output.description = body.description;
                        output.timeout = body.timeout;
                        output.memory_size = body.memory_size;
                        output.last_modified = body.last_modified;
                        output.code_sha256 = body.code_sha256;
                        output.version = body.version;
                        output.vpc_config = body.vpc_config;
                        output.dead_letter_config = body.dead_letter_config;
                        output.environment = body.environment;
                        output.kms_key_arn = body.kms_key_arn;
                        output.tracing_config = body.tracing_config;
                        output.master_arn = body.master_arn;
                        output.revision_id = body.revision_id;
                        output.layers = body.layers;
                        output.state = body.state;
                        output.state_reason = body.state_reason;
                        output.state_reason_code = body.state_reason_code;
                        output.last_update_status = body.last_update_status;
                        output.last_update_status_reason = body.last_update_status_reason;
                        output.last_update_status_reason_code = body.last_update_status_reason_code;
                        output.file_system_configs = body.file_system_configs;
                        output.package_type = body.package_type;
                        output.image_config_response = body.image_config_response;
                        output.signing_profile_version_arn = body.signing_profile_version_arn;
                        output.signing_job_arn = body.signing_job_arn;
                        output.architectures = body.architectures;
                        output.ephemeral_storage = body.ephemeral_storage;

                        serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn create_function_url_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateFunctionUrlConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_function_url_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __create_function_url_config(input: CreateFunctionUrlConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2021-10-31/functions/{FunctionName}/url");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

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
        "lambda",
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
    http_request.add_header("x-amz-target", "CreateFunctionUrlConfig");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: CreateFunctionUrlConfigResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: CreateFunctionUrlConfigResponse = serde_json::from_slice(body).unwrap();
                        output.function_url = body.function_url;
                        output.function_arn = body.function_arn;
                        output.auth_type = body.auth_type;
                        output.cors = body.cors;
                        output.creation_time = body.creation_time;

                        serde_json::to_vec(&Result::<CreateFunctionUrlConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<CreateFunctionUrlConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<CreateFunctionUrlConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn delete_alias(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteAliasRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_alias(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_alias(input: DeleteAliasRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/aliases/{Name}");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };
    path = match path.find("{Name}") {
        Some(_) => path.replace("{Name}", &input.name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "DeleteAlias");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
pub fn delete_code_signing_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteCodeSigningConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_code_signing_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_code_signing_config(input: DeleteCodeSigningConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2020-04-22/code-signing-configs/{CodeSigningConfigArn}");
    path = match path.find("{CodeSigningConfigArn}") {
        Some(_) => path.replace("{CodeSigningConfigArn}", &input.code_signing_config_arn.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "DeleteCodeSigningConfig");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DeleteCodeSigningConfigResponse = Default::default();


                        serde_json::to_vec(&Result::<DeleteCodeSigningConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DeleteCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DeleteCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn delete_event_source_mapping(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteEventSourceMappingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_event_source_mapping(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_event_source_mapping(input: DeleteEventSourceMappingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/event-source-mappings/{UUID}");
    path = match path.find("{UUID}") {
        Some(_) => path.replace("{UUID}", &input.uuid.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "DeleteEventSourceMapping");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: EventSourceMappingConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: EventSourceMappingConfiguration = serde_json::from_slice(body).unwrap();
                        output.uuid = body.uuid;
                        output.starting_position = body.starting_position;
                        output.starting_position_timestamp = body.starting_position_timestamp;
                        output.batch_size = body.batch_size;
                        output.maximum_batching_window_in_seconds = body.maximum_batching_window_in_seconds;
                        output.parallelization_factor = body.parallelization_factor;
                        output.event_source_arn = body.event_source_arn;
                        output.filter_criteria = body.filter_criteria;
                        output.function_arn = body.function_arn;
                        output.last_modified = body.last_modified;
                        output.last_processing_result = body.last_processing_result;
                        output.state = body.state;
                        output.state_transition_reason = body.state_transition_reason;
                        output.destination_config = body.destination_config;
                        output.topics = body.topics;
                        output.queues = body.queues;
                        output.source_access_configurations = body.source_access_configurations;
                        output.self_managed_event_source = body.self_managed_event_source;
                        output.maximum_record_age_in_seconds = body.maximum_record_age_in_seconds;
                        output.bisect_batch_on_function_error = body.bisect_batch_on_function_error;
                        output.maximum_retry_attempts = body.maximum_retry_attempts;
                        output.tumbling_window_in_seconds = body.tumbling_window_in_seconds;
                        output.function_response_types = body.function_response_types;
                        output.amazon_managed_kafka_event_source_config = body.amazon_managed_kafka_event_source_config;
                        output.self_managed_kafka_event_source_config = body.self_managed_kafka_event_source_config;

                        serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn delete_function(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteFunctionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_function(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_function(input: DeleteFunctionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "DeleteFunction");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
pub fn delete_function_code_signing_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteFunctionCodeSigningConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_function_code_signing_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_function_code_signing_config(input: DeleteFunctionCodeSigningConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2020-06-30/functions/{FunctionName}/code-signing-config");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "DeleteFunctionCodeSigningConfig");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
pub fn delete_function_concurrency(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteFunctionConcurrencyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_function_concurrency(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_function_concurrency(input: DeleteFunctionConcurrencyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2017-10-31/functions/{FunctionName}/concurrency");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "DeleteFunctionConcurrency");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
pub fn delete_function_event_invoke_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteFunctionEventInvokeConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_function_event_invoke_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_function_event_invoke_config(input: DeleteFunctionEventInvokeConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2019-09-25/functions/{FunctionName}/event-invoke-config");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "DeleteFunctionEventInvokeConfig");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
pub fn delete_function_url_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteFunctionUrlConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_function_url_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_function_url_config(input: DeleteFunctionUrlConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2021-10-31/functions/{FunctionName}/url");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "DeleteFunctionUrlConfig");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
pub fn delete_layer_version(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteLayerVersionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_layer_version(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_layer_version(input: DeleteLayerVersionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2018-10-31/layers/{LayerName}/versions/{VersionNumber}");
    path = match path.find("{LayerName}") {
        Some(_) => path.replace("{LayerName}", &input.layer_name.to_string()),
        None => path.to_string(),
    };
    path = match path.find("{VersionNumber}") {
        Some(_) => path.replace("{VersionNumber}", &input.version_number.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "DeleteLayerVersion");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
pub fn delete_provisioned_concurrency_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteProvisionedConcurrencyConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_provisioned_concurrency_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_provisioned_concurrency_config(input: DeleteProvisionedConcurrencyConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2019-09-30/functions/{FunctionName}/provisioned-concurrency");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "DeleteProvisionedConcurrencyConfig");
    http_request.set_content_type(String::from("application/json"));

    http_request.add_param("Qualifier", &input.qualifier);

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
pub fn get_account_settings(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetAccountSettingsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_account_settings(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_account_settings(input: GetAccountSettingsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2016-08-19/account-settings/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetAccountSettings");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetAccountSettingsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetAccountSettingsResponse = serde_json::from_slice(body).unwrap();
                        output.account_limit = body.account_limit;
                        output.account_usage = body.account_usage;

                        serde_json::to_vec(&Result::<GetAccountSettingsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetAccountSettingsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetAccountSettingsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_alias(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetAliasRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_alias(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_alias(input: GetAliasRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/aliases/{Name}");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };
    path = match path.find("{Name}") {
        Some(_) => path.replace("{Name}", &input.name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetAlias");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: AliasConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: AliasConfiguration = serde_json::from_slice(body).unwrap();
                        output.alias_arn = body.alias_arn;
                        output.name = body.name;
                        output.function_version = body.function_version;
                        output.description = body.description;
                        output.routing_config = body.routing_config;
                        output.revision_id = body.revision_id;

                        serde_json::to_vec(&Result::<AliasConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<AliasConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<AliasConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_code_signing_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetCodeSigningConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_code_signing_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_code_signing_config(input: GetCodeSigningConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2020-04-22/code-signing-configs/{CodeSigningConfigArn}");
    path = match path.find("{CodeSigningConfigArn}") {
        Some(_) => path.replace("{CodeSigningConfigArn}", &input.code_signing_config_arn.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetCodeSigningConfig");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetCodeSigningConfigResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetCodeSigningConfigResponse = serde_json::from_slice(body).unwrap();
                        output.code_signing_config = body.code_signing_config;

                        serde_json::to_vec(&Result::<GetCodeSigningConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_event_source_mapping(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetEventSourceMappingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_event_source_mapping(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_event_source_mapping(input: GetEventSourceMappingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/event-source-mappings/{UUID}");
    path = match path.find("{UUID}") {
        Some(_) => path.replace("{UUID}", &input.uuid.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetEventSourceMapping");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: EventSourceMappingConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: EventSourceMappingConfiguration = serde_json::from_slice(body).unwrap();
                        output.uuid = body.uuid;
                        output.starting_position = body.starting_position;
                        output.starting_position_timestamp = body.starting_position_timestamp;
                        output.batch_size = body.batch_size;
                        output.maximum_batching_window_in_seconds = body.maximum_batching_window_in_seconds;
                        output.parallelization_factor = body.parallelization_factor;
                        output.event_source_arn = body.event_source_arn;
                        output.filter_criteria = body.filter_criteria;
                        output.function_arn = body.function_arn;
                        output.last_modified = body.last_modified;
                        output.last_processing_result = body.last_processing_result;
                        output.state = body.state;
                        output.state_transition_reason = body.state_transition_reason;
                        output.destination_config = body.destination_config;
                        output.topics = body.topics;
                        output.queues = body.queues;
                        output.source_access_configurations = body.source_access_configurations;
                        output.self_managed_event_source = body.self_managed_event_source;
                        output.maximum_record_age_in_seconds = body.maximum_record_age_in_seconds;
                        output.bisect_batch_on_function_error = body.bisect_batch_on_function_error;
                        output.maximum_retry_attempts = body.maximum_retry_attempts;
                        output.tumbling_window_in_seconds = body.tumbling_window_in_seconds;
                        output.function_response_types = body.function_response_types;
                        output.amazon_managed_kafka_event_source_config = body.amazon_managed_kafka_event_source_config;
                        output.self_managed_kafka_event_source_config = body.self_managed_kafka_event_source_config;

                        serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_function(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetFunctionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_function(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_function(input: GetFunctionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetFunction");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetFunctionResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetFunctionResponse = serde_json::from_slice(body).unwrap();
                        output.configuration = body.configuration;
                        output.code = body.code;
                        output.tags = body.tags;
                        output.concurrency = body.concurrency;

                        serde_json::to_vec(&Result::<GetFunctionResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetFunctionResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetFunctionResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_function_code_signing_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetFunctionCodeSigningConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_function_code_signing_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_function_code_signing_config(input: GetFunctionCodeSigningConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2020-06-30/functions/{FunctionName}/code-signing-config");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetFunctionCodeSigningConfig");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetFunctionCodeSigningConfigResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetFunctionCodeSigningConfigResponse = serde_json::from_slice(body).unwrap();
                        output.code_signing_config_arn = body.code_signing_config_arn;
                        output.function_name = body.function_name;

                        serde_json::to_vec(&Result::<GetFunctionCodeSigningConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetFunctionCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetFunctionCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_function_concurrency(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetFunctionConcurrencyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_function_concurrency(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_function_concurrency(input: GetFunctionConcurrencyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2019-09-30/functions/{FunctionName}/concurrency");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetFunctionConcurrency");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetFunctionConcurrencyResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetFunctionConcurrencyResponse = serde_json::from_slice(body).unwrap();
                        output.reserved_concurrent_executions = body.reserved_concurrent_executions;

                        serde_json::to_vec(&Result::<GetFunctionConcurrencyResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetFunctionConcurrencyResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetFunctionConcurrencyResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_function_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetFunctionConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_function_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_function_configuration(input: GetFunctionConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/configuration");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetFunctionConfiguration");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: FunctionConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: FunctionConfiguration = serde_json::from_slice(body).unwrap();
                        output.function_name = body.function_name;
                        output.function_arn = body.function_arn;
                        output.runtime = body.runtime;
                        output.role = body.role;
                        output.handler = body.handler;
                        output.code_size = body.code_size;
                        output.description = body.description;
                        output.timeout = body.timeout;
                        output.memory_size = body.memory_size;
                        output.last_modified = body.last_modified;
                        output.code_sha256 = body.code_sha256;
                        output.version = body.version;
                        output.vpc_config = body.vpc_config;
                        output.dead_letter_config = body.dead_letter_config;
                        output.environment = body.environment;
                        output.kms_key_arn = body.kms_key_arn;
                        output.tracing_config = body.tracing_config;
                        output.master_arn = body.master_arn;
                        output.revision_id = body.revision_id;
                        output.layers = body.layers;
                        output.state = body.state;
                        output.state_reason = body.state_reason;
                        output.state_reason_code = body.state_reason_code;
                        output.last_update_status = body.last_update_status;
                        output.last_update_status_reason = body.last_update_status_reason;
                        output.last_update_status_reason_code = body.last_update_status_reason_code;
                        output.file_system_configs = body.file_system_configs;
                        output.package_type = body.package_type;
                        output.image_config_response = body.image_config_response;
                        output.signing_profile_version_arn = body.signing_profile_version_arn;
                        output.signing_job_arn = body.signing_job_arn;
                        output.architectures = body.architectures;
                        output.ephemeral_storage = body.ephemeral_storage;

                        serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_function_event_invoke_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetFunctionEventInvokeConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_function_event_invoke_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_function_event_invoke_config(input: GetFunctionEventInvokeConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2019-09-25/functions/{FunctionName}/event-invoke-config");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetFunctionEventInvokeConfig");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: FunctionEventInvokeConfig = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: FunctionEventInvokeConfig = serde_json::from_slice(body).unwrap();
                        output.last_modified = body.last_modified;
                        output.function_arn = body.function_arn;
                        output.maximum_retry_attempts = body.maximum_retry_attempts;
                        output.maximum_event_age_in_seconds = body.maximum_event_age_in_seconds;
                        output.destination_config = body.destination_config;

                        serde_json::to_vec(&Result::<FunctionEventInvokeConfig, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<FunctionEventInvokeConfig, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<FunctionEventInvokeConfig, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_function_url_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetFunctionUrlConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_function_url_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_function_url_config(input: GetFunctionUrlConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2021-10-31/functions/{FunctionName}/url");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetFunctionUrlConfig");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetFunctionUrlConfigResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetFunctionUrlConfigResponse = serde_json::from_slice(body).unwrap();
                        output.function_url = body.function_url;
                        output.function_arn = body.function_arn;
                        output.auth_type = body.auth_type;
                        output.cors = body.cors;
                        output.creation_time = body.creation_time;
                        output.last_modified_time = body.last_modified_time;

                        serde_json::to_vec(&Result::<GetFunctionUrlConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetFunctionUrlConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetFunctionUrlConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_layer_version(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetLayerVersionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_layer_version(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_layer_version(input: GetLayerVersionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2018-10-31/layers/{LayerName}/versions/{VersionNumber}");
    path = match path.find("{LayerName}") {
        Some(_) => path.replace("{LayerName}", &input.layer_name.to_string()),
        None => path.to_string(),
    };
    path = match path.find("{VersionNumber}") {
        Some(_) => path.replace("{VersionNumber}", &input.version_number.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetLayerVersion");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetLayerVersionResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetLayerVersionResponse = serde_json::from_slice(body).unwrap();
                        output.content = body.content;
                        output.layer_arn = body.layer_arn;
                        output.layer_version_arn = body.layer_version_arn;
                        output.description = body.description;
                        output.created_date = body.created_date;
                        output.version = body.version;
                        output.compatible_runtimes = body.compatible_runtimes;
                        output.license_info = body.license_info;
                        output.compatible_architectures = body.compatible_architectures;

                        serde_json::to_vec(&Result::<GetLayerVersionResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetLayerVersionResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetLayerVersionResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_layer_version_by_arn(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetLayerVersionByArnRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_layer_version_by_arn(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_layer_version_by_arn(input: GetLayerVersionByArnRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2018-10-31/layers?find=LayerVersion");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetLayerVersionByArn");
    http_request.set_content_type(String::from("application/json"));

    http_request.add_param("Arn", &input.arn);

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetLayerVersionResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetLayerVersionResponse = serde_json::from_slice(body).unwrap();
                        output.content = body.content;
                        output.layer_arn = body.layer_arn;
                        output.layer_version_arn = body.layer_version_arn;
                        output.description = body.description;
                        output.created_date = body.created_date;
                        output.version = body.version;
                        output.compatible_runtimes = body.compatible_runtimes;
                        output.license_info = body.license_info;
                        output.compatible_architectures = body.compatible_architectures;

                        serde_json::to_vec(&Result::<GetLayerVersionResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetLayerVersionResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetLayerVersionResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_layer_version_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetLayerVersionPolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_layer_version_policy(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_layer_version_policy(input: GetLayerVersionPolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2018-10-31/layers/{LayerName}/versions/{VersionNumber}/policy");
    path = match path.find("{LayerName}") {
        Some(_) => path.replace("{LayerName}", &input.layer_name.to_string()),
        None => path.to_string(),
    };
    path = match path.find("{VersionNumber}") {
        Some(_) => path.replace("{VersionNumber}", &input.version_number.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetLayerVersionPolicy");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetLayerVersionPolicyResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetLayerVersionPolicyResponse = serde_json::from_slice(body).unwrap();
                        output.policy = body.policy;
                        output.revision_id = body.revision_id;

                        serde_json::to_vec(&Result::<GetLayerVersionPolicyResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetLayerVersionPolicyResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetLayerVersionPolicyResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetPolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_policy(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_policy(input: GetPolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/policy");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetPolicy");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetPolicyResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetPolicyResponse = serde_json::from_slice(body).unwrap();
                        output.policy = body.policy;
                        output.revision_id = body.revision_id;

                        serde_json::to_vec(&Result::<GetPolicyResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetPolicyResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetPolicyResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_provisioned_concurrency_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetProvisionedConcurrencyConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_provisioned_concurrency_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_provisioned_concurrency_config(input: GetProvisionedConcurrencyConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2019-09-30/functions/{FunctionName}/provisioned-concurrency");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "GetProvisionedConcurrencyConfig");
    http_request.set_content_type(String::from("application/json"));

    http_request.add_param("Qualifier", &input.qualifier);

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetProvisionedConcurrencyConfigResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: GetProvisionedConcurrencyConfigResponse = serde_json::from_slice(body).unwrap();
                        output.requested_provisioned_concurrent_executions = body.requested_provisioned_concurrent_executions;
                        output.available_provisioned_concurrent_executions = body.available_provisioned_concurrent_executions;
                        output.allocated_provisioned_concurrent_executions = body.allocated_provisioned_concurrent_executions;
                        output.status = body.status;
                        output.status_reason = body.status_reason;
                        output.last_modified = body.last_modified;

                        serde_json::to_vec(&Result::<GetProvisionedConcurrencyConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetProvisionedConcurrencyConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetProvisionedConcurrencyConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn invoke(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: InvocationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __invoke(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __invoke(input: InvocationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/invocations");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

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
        "lambda",
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

    http_request.set_payload(input.payload);

    if let Some(invocation_type) = input.invocation_type {
        http_request.add_header("X-Amz-Invocation-Type", &invocation_type);
    };
    if let Some(log_type) = input.log_type {
        http_request.add_header("X-Amz-Log-Type", &log_type);
    };
    if let Some(client_context) = input.client_context {
        http_request.add_header("X-Amz-Client-Context", &client_context);
    };
    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "Invoke");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: InvocationResponse = Default::default();
                        output.function_error = match response.headers().get("X-Amz-Function-Error") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.log_result = match response.headers().get("X-Amz-Log-Result") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.executed_version = match response.headers().get("X-Amz-Executed-Version") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        output.payload = Some(Vec::from(body));

                        serde_json::to_vec(&Result::<InvocationResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<InvocationResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<InvocationResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn invoke_async(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: InvokeAsyncRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __invoke_async(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __invoke_async(input: InvokeAsyncRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2014-11-13/functions/{FunctionName}/invoke-async/");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

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
        "lambda",
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

    http_request.set_payload(Some(input.invoke_args));

    http_request.add_header("accept-encoding", "identity");
    http_request.add_header("x-amz-target", "InvokeAsync");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: InvokeAsyncResponse = Default::default();


                        serde_json::to_vec(&Result::<InvokeAsyncResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<InvokeAsyncResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<InvokeAsyncResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_aliases(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListAliasesRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_aliases(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_aliases(input: ListAliasesRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/aliases");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListAliases");
    http_request.set_content_type(String::from("application/json"));

    if let Some(function_version) = input.function_version {
        http_request.add_param("FunctionVersion", &function_version);
    };
    if let Some(marker) = input.marker {
        http_request.add_param("Marker", &marker);
    };
    if let Some(max_items) = input.max_items {
        http_request.add_param("MaxItems", &serde_json::to_string(&max_items).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListAliasesResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListAliasesResponse = serde_json::from_slice(body).unwrap();
                        output.next_marker = body.next_marker;
                        output.aliases = body.aliases;

                        serde_json::to_vec(&Result::<ListAliasesResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListAliasesResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListAliasesResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_code_signing_configs(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListCodeSigningConfigsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_code_signing_configs(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_code_signing_configs(input: ListCodeSigningConfigsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2020-04-22/code-signing-configs/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListCodeSigningConfigs");
    http_request.set_content_type(String::from("application/json"));

    if let Some(marker) = input.marker {
        http_request.add_param("Marker", &marker);
    };
    if let Some(max_items) = input.max_items {
        http_request.add_param("MaxItems", &serde_json::to_string(&max_items).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListCodeSigningConfigsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListCodeSigningConfigsResponse = serde_json::from_slice(body).unwrap();
                        output.next_marker = body.next_marker;
                        output.code_signing_configs = body.code_signing_configs;

                        serde_json::to_vec(&Result::<ListCodeSigningConfigsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListCodeSigningConfigsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListCodeSigningConfigsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_event_source_mappings(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListEventSourceMappingsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_event_source_mappings(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_event_source_mappings(input: ListEventSourceMappingsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/event-source-mappings/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListEventSourceMappings");
    http_request.set_content_type(String::from("application/json"));

    if let Some(event_source_arn) = input.event_source_arn {
        http_request.add_param("EventSourceArn", &event_source_arn);
    };
    if let Some(function_name) = input.function_name {
        http_request.add_param("FunctionName", &function_name);
    };
    if let Some(marker) = input.marker {
        http_request.add_param("Marker", &marker);
    };
    if let Some(max_items) = input.max_items {
        http_request.add_param("MaxItems", &serde_json::to_string(&max_items).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListEventSourceMappingsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListEventSourceMappingsResponse = serde_json::from_slice(body).unwrap();
                        output.next_marker = body.next_marker;
                        output.event_source_mappings = body.event_source_mappings;

                        serde_json::to_vec(&Result::<ListEventSourceMappingsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListEventSourceMappingsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListEventSourceMappingsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_function_event_invoke_configs(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListFunctionEventInvokeConfigsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_function_event_invoke_configs(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_function_event_invoke_configs(input: ListFunctionEventInvokeConfigsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2019-09-25/functions/{FunctionName}/event-invoke-config/list");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListFunctionEventInvokeConfigs");
    http_request.set_content_type(String::from("application/json"));

    if let Some(marker) = input.marker {
        http_request.add_param("Marker", &marker);
    };
    if let Some(max_items) = input.max_items {
        http_request.add_param("MaxItems", &serde_json::to_string(&max_items).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListFunctionEventInvokeConfigsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListFunctionEventInvokeConfigsResponse = serde_json::from_slice(body).unwrap();
                        output.function_event_invoke_configs = body.function_event_invoke_configs;
                        output.next_marker = body.next_marker;

                        serde_json::to_vec(&Result::<ListFunctionEventInvokeConfigsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListFunctionEventInvokeConfigsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListFunctionEventInvokeConfigsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_function_url_configs(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListFunctionUrlConfigsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_function_url_configs(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_function_url_configs(input: ListFunctionUrlConfigsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2021-10-31/functions/{FunctionName}/urls");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListFunctionUrlConfigs");
    http_request.set_content_type(String::from("application/json"));

    if let Some(marker) = input.marker {
        http_request.add_param("Marker", &marker);
    };
    if let Some(max_items) = input.max_items {
        http_request.add_param("MaxItems", &serde_json::to_string(&max_items).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListFunctionUrlConfigsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListFunctionUrlConfigsResponse = serde_json::from_slice(body).unwrap();
                        output.function_url_configs = body.function_url_configs;
                        output.next_marker = body.next_marker;

                        serde_json::to_vec(&Result::<ListFunctionUrlConfigsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListFunctionUrlConfigsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListFunctionUrlConfigsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_functions(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListFunctionsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_functions(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_functions(input: ListFunctionsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListFunctions");
    http_request.set_content_type(String::from("application/json"));

    if let Some(master_region) = input.master_region {
        http_request.add_param("MasterRegion", &master_region);
    };
    if let Some(function_version) = input.function_version {
        http_request.add_param("FunctionVersion", &function_version);
    };
    if let Some(marker) = input.marker {
        http_request.add_param("Marker", &marker);
    };
    if let Some(max_items) = input.max_items {
        http_request.add_param("MaxItems", &serde_json::to_string(&max_items).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListFunctionsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListFunctionsResponse = serde_json::from_slice(body).unwrap();
                        output.next_marker = body.next_marker;
                        output.functions = body.functions;

                        serde_json::to_vec(&Result::<ListFunctionsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListFunctionsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListFunctionsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_functions_by_code_signing_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListFunctionsByCodeSigningConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_functions_by_code_signing_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_functions_by_code_signing_config(input: ListFunctionsByCodeSigningConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2020-04-22/code-signing-configs/{CodeSigningConfigArn}/functions");
    path = match path.find("{CodeSigningConfigArn}") {
        Some(_) => path.replace("{CodeSigningConfigArn}", &input.code_signing_config_arn.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListFunctionsByCodeSigningConfig");
    http_request.set_content_type(String::from("application/json"));

    if let Some(marker) = input.marker {
        http_request.add_param("Marker", &marker);
    };
    if let Some(max_items) = input.max_items {
        http_request.add_param("MaxItems", &serde_json::to_string(&max_items).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListFunctionsByCodeSigningConfigResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListFunctionsByCodeSigningConfigResponse = serde_json::from_slice(body).unwrap();
                        output.next_marker = body.next_marker;
                        output.function_arns = body.function_arns;

                        serde_json::to_vec(&Result::<ListFunctionsByCodeSigningConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListFunctionsByCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListFunctionsByCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_layer_versions(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListLayerVersionsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_layer_versions(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_layer_versions(input: ListLayerVersionsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2018-10-31/layers/{LayerName}/versions");
    path = match path.find("{LayerName}") {
        Some(_) => path.replace("{LayerName}", &input.layer_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListLayerVersions");
    http_request.set_content_type(String::from("application/json"));

    if let Some(compatible_runtime) = input.compatible_runtime {
        http_request.add_param("CompatibleRuntime", &compatible_runtime);
    };
    if let Some(marker) = input.marker {
        http_request.add_param("Marker", &marker);
    };
    if let Some(max_items) = input.max_items {
        http_request.add_param("MaxItems", &serde_json::to_string(&max_items).unwrap());
    };
    if let Some(compatible_architecture) = input.compatible_architecture {
        http_request.add_param("CompatibleArchitecture", &compatible_architecture);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListLayerVersionsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListLayerVersionsResponse = serde_json::from_slice(body).unwrap();
                        output.next_marker = body.next_marker;
                        output.layer_versions = body.layer_versions;

                        serde_json::to_vec(&Result::<ListLayerVersionsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListLayerVersionsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListLayerVersionsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_layers(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListLayersRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_layers(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_layers(input: ListLayersRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2018-10-31/layers");

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListLayers");
    http_request.set_content_type(String::from("application/json"));

    if let Some(compatible_runtime) = input.compatible_runtime {
        http_request.add_param("CompatibleRuntime", &compatible_runtime);
    };
    if let Some(marker) = input.marker {
        http_request.add_param("Marker", &marker);
    };
    if let Some(max_items) = input.max_items {
        http_request.add_param("MaxItems", &serde_json::to_string(&max_items).unwrap());
    };
    if let Some(compatible_architecture) = input.compatible_architecture {
        http_request.add_param("CompatibleArchitecture", &compatible_architecture);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListLayersResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListLayersResponse = serde_json::from_slice(body).unwrap();
                        output.next_marker = body.next_marker;
                        output.layers = body.layers;

                        serde_json::to_vec(&Result::<ListLayersResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListLayersResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListLayersResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_provisioned_concurrency_configs(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListProvisionedConcurrencyConfigsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_provisioned_concurrency_configs(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_provisioned_concurrency_configs(input: ListProvisionedConcurrencyConfigsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2019-09-30/functions/{FunctionName}/provisioned-concurrency?List=ALL");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListProvisionedConcurrencyConfigs");
    http_request.set_content_type(String::from("application/json"));

    if let Some(marker) = input.marker {
        http_request.add_param("Marker", &marker);
    };
    if let Some(max_items) = input.max_items {
        http_request.add_param("MaxItems", &serde_json::to_string(&max_items).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListProvisionedConcurrencyConfigsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListProvisionedConcurrencyConfigsResponse = serde_json::from_slice(body).unwrap();
                        output.provisioned_concurrency_configs = body.provisioned_concurrency_configs;
                        output.next_marker = body.next_marker;

                        serde_json::to_vec(&Result::<ListProvisionedConcurrencyConfigsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListProvisionedConcurrencyConfigsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListProvisionedConcurrencyConfigsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_tags(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListTagsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_tags(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_tags(input: ListTagsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2017-03-31/tags/{ARN}");
    path = match path.find("{ARN}") {
        Some(_) => path.replace("{ARN}", &input.resource.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListTags");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListTagsResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListTagsResponse = serde_json::from_slice(body).unwrap();
                        output.tags = body.tags;

                        serde_json::to_vec(&Result::<ListTagsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListTagsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListTagsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_versions_by_function(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListVersionsByFunctionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_versions_by_function(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_versions_by_function(input: ListVersionsByFunctionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/versions");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "lambda",
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
    http_request.add_header("x-amz-target", "ListVersionsByFunction");
    http_request.set_content_type(String::from("application/json"));

    if let Some(marker) = input.marker {
        http_request.add_param("Marker", &marker);
    };
    if let Some(max_items) = input.max_items {
        http_request.add_param("MaxItems", &serde_json::to_string(&max_items).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListVersionsByFunctionResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: ListVersionsByFunctionResponse = serde_json::from_slice(body).unwrap();
                        output.next_marker = body.next_marker;
                        output.versions = body.versions;

                        serde_json::to_vec(&Result::<ListVersionsByFunctionResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListVersionsByFunctionResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListVersionsByFunctionResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn publish_layer_version(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PublishLayerVersionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __publish_layer_version(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __publish_layer_version(input: PublishLayerVersionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2018-10-31/layers/{LayerName}/versions");
    path = match path.find("{LayerName}") {
        Some(_) => path.replace("{LayerName}", &input.layer_name.to_string()),
        None => path.to_string(),
    };

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
        "lambda",
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
    http_request.add_header("x-amz-target", "PublishLayerVersion");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: PublishLayerVersionResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: PublishLayerVersionResponse = serde_json::from_slice(body).unwrap();
                        output.content = body.content;
                        output.layer_arn = body.layer_arn;
                        output.layer_version_arn = body.layer_version_arn;
                        output.description = body.description;
                        output.created_date = body.created_date;
                        output.version = body.version;
                        output.compatible_runtimes = body.compatible_runtimes;
                        output.license_info = body.license_info;
                        output.compatible_architectures = body.compatible_architectures;

                        serde_json::to_vec(&Result::<PublishLayerVersionResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PublishLayerVersionResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PublishLayerVersionResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn publish_version(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PublishVersionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __publish_version(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __publish_version(input: PublishVersionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/versions");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

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
        "lambda",
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
    http_request.add_header("x-amz-target", "PublishVersion");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: FunctionConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: FunctionConfiguration = serde_json::from_slice(body).unwrap();
                        output.function_name = body.function_name;
                        output.function_arn = body.function_arn;
                        output.runtime = body.runtime;
                        output.role = body.role;
                        output.handler = body.handler;
                        output.code_size = body.code_size;
                        output.description = body.description;
                        output.timeout = body.timeout;
                        output.memory_size = body.memory_size;
                        output.last_modified = body.last_modified;
                        output.code_sha256 = body.code_sha256;
                        output.version = body.version;
                        output.vpc_config = body.vpc_config;
                        output.dead_letter_config = body.dead_letter_config;
                        output.environment = body.environment;
                        output.kms_key_arn = body.kms_key_arn;
                        output.tracing_config = body.tracing_config;
                        output.master_arn = body.master_arn;
                        output.revision_id = body.revision_id;
                        output.layers = body.layers;
                        output.state = body.state;
                        output.state_reason = body.state_reason;
                        output.state_reason_code = body.state_reason_code;
                        output.last_update_status = body.last_update_status;
                        output.last_update_status_reason = body.last_update_status_reason;
                        output.last_update_status_reason_code = body.last_update_status_reason_code;
                        output.file_system_configs = body.file_system_configs;
                        output.package_type = body.package_type;
                        output.image_config_response = body.image_config_response;
                        output.signing_profile_version_arn = body.signing_profile_version_arn;
                        output.signing_job_arn = body.signing_job_arn;
                        output.architectures = body.architectures;
                        output.ephemeral_storage = body.ephemeral_storage;

                        serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn put_function_code_signing_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutFunctionCodeSigningConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_function_code_signing_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_function_code_signing_config(input: PutFunctionCodeSigningConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2020-06-30/functions/{FunctionName}/code-signing-config");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "lambda",
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
    http_request.add_header("x-amz-target", "PutFunctionCodeSigningConfig");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: PutFunctionCodeSigningConfigResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: PutFunctionCodeSigningConfigResponse = serde_json::from_slice(body).unwrap();
                        output.code_signing_config_arn = body.code_signing_config_arn;
                        output.function_name = body.function_name;

                        serde_json::to_vec(&Result::<PutFunctionCodeSigningConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PutFunctionCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PutFunctionCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn put_function_concurrency(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutFunctionConcurrencyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_function_concurrency(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_function_concurrency(input: PutFunctionConcurrencyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2017-10-31/functions/{FunctionName}/concurrency");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "lambda",
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
    http_request.add_header("x-amz-target", "PutFunctionConcurrency");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: Concurrency = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: Concurrency = serde_json::from_slice(body).unwrap();
                        output.reserved_concurrent_executions = body.reserved_concurrent_executions;

                        serde_json::to_vec(&Result::<Concurrency, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<Concurrency, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<Concurrency, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn put_function_event_invoke_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutFunctionEventInvokeConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_function_event_invoke_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_function_event_invoke_config(input: PutFunctionEventInvokeConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2019-09-25/functions/{FunctionName}/event-invoke-config");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "lambda",
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
    http_request.add_header("x-amz-target", "PutFunctionEventInvokeConfig");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: FunctionEventInvokeConfig = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: FunctionEventInvokeConfig = serde_json::from_slice(body).unwrap();
                        output.last_modified = body.last_modified;
                        output.function_arn = body.function_arn;
                        output.maximum_retry_attempts = body.maximum_retry_attempts;
                        output.maximum_event_age_in_seconds = body.maximum_event_age_in_seconds;
                        output.destination_config = body.destination_config;

                        serde_json::to_vec(&Result::<FunctionEventInvokeConfig, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<FunctionEventInvokeConfig, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<FunctionEventInvokeConfig, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn put_provisioned_concurrency_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutProvisionedConcurrencyConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_provisioned_concurrency_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_provisioned_concurrency_config(input: PutProvisionedConcurrencyConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2019-09-30/functions/{FunctionName}/provisioned-concurrency");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "lambda",
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
    http_request.add_header("x-amz-target", "PutProvisionedConcurrencyConfig");
    http_request.set_content_type(String::from("application/json"));

    http_request.add_param("Qualifier", &input.qualifier);

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: PutProvisionedConcurrencyConfigResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: PutProvisionedConcurrencyConfigResponse = serde_json::from_slice(body).unwrap();
                        output.requested_provisioned_concurrent_executions = body.requested_provisioned_concurrent_executions;
                        output.available_provisioned_concurrent_executions = body.available_provisioned_concurrent_executions;
                        output.allocated_provisioned_concurrent_executions = body.allocated_provisioned_concurrent_executions;
                        output.status = body.status;
                        output.status_reason = body.status_reason;
                        output.last_modified = body.last_modified;

                        serde_json::to_vec(&Result::<PutProvisionedConcurrencyConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PutProvisionedConcurrencyConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PutProvisionedConcurrencyConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn remove_layer_version_permission(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: RemoveLayerVersionPermissionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __remove_layer_version_permission(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __remove_layer_version_permission(input: RemoveLayerVersionPermissionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2018-10-31/layers/{LayerName}/versions/{VersionNumber}/policy/{StatementId}");
    path = match path.find("{LayerName}") {
        Some(_) => path.replace("{LayerName}", &input.layer_name.to_string()),
        None => path.to_string(),
    };
    path = match path.find("{VersionNumber}") {
        Some(_) => path.replace("{VersionNumber}", &input.version_number.to_string()),
        None => path.to_string(),
    };
    path = match path.find("{StatementId}") {
        Some(_) => path.replace("{StatementId}", &input.statement_id.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "RemoveLayerVersionPermission");
    http_request.set_content_type(String::from("application/json"));

    if let Some(revision_id) = input.revision_id {
        http_request.add_param("RevisionId", &revision_id);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
pub fn remove_permission(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: RemovePermissionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __remove_permission(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __remove_permission(input: RemovePermissionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/policy/{StatementId}");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };
    path = match path.find("{StatementId}") {
        Some(_) => path.replace("{StatementId}", &input.statement_id.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "RemovePermission");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };
    if let Some(revision_id) = input.revision_id {
        http_request.add_param("RevisionId", &revision_id);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
pub fn tag_resource(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: TagResourceRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __tag_resource(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __tag_resource(input: TagResourceRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2017-03-31/tags/{ARN}");
    path = match path.find("{ARN}") {
        Some(_) => path.replace("{ARN}", &input.resource.to_string()),
        None => path.to_string(),
    };

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
        "lambda",
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
    http_request.add_header("x-amz-target", "TagResource");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
    let mut path = String::from("/2017-03-31/tags/{ARN}");
    path = match path.find("{ARN}") {
        Some(_) => path.replace("{ARN}", &input.resource.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "lambda",
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
    http_request.add_header("x-amz-target", "UntagResource");
    http_request.set_content_type(String::from("application/json"));

    http_request.add_param("tagKeys", &serde_json::to_string(&input.tag_keys).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
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
pub fn update_alias(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateAliasRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __update_alias(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_alias(input: UpdateAliasRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/aliases/{Name}");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };
    path = match path.find("{Name}") {
        Some(_) => path.replace("{Name}", &input.name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "lambda",
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
    http_request.add_header("x-amz-target", "UpdateAlias");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: AliasConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: AliasConfiguration = serde_json::from_slice(body).unwrap();
                        output.alias_arn = body.alias_arn;
                        output.name = body.name;
                        output.function_version = body.function_version;
                        output.description = body.description;
                        output.routing_config = body.routing_config;
                        output.revision_id = body.revision_id;

                        serde_json::to_vec(&Result::<AliasConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<AliasConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<AliasConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_code_signing_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateCodeSigningConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __update_code_signing_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_code_signing_config(input: UpdateCodeSigningConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2020-04-22/code-signing-configs/{CodeSigningConfigArn}");
    path = match path.find("{CodeSigningConfigArn}") {
        Some(_) => path.replace("{CodeSigningConfigArn}", &input.code_signing_config_arn.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "lambda",
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
    http_request.add_header("x-amz-target", "UpdateCodeSigningConfig");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateCodeSigningConfigResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: UpdateCodeSigningConfigResponse = serde_json::from_slice(body).unwrap();
                        output.code_signing_config = body.code_signing_config;

                        serde_json::to_vec(&Result::<UpdateCodeSigningConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateCodeSigningConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_event_source_mapping(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateEventSourceMappingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __update_event_source_mapping(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_event_source_mapping(input: UpdateEventSourceMappingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/event-source-mappings/{UUID}");
    path = match path.find("{UUID}") {
        Some(_) => path.replace("{UUID}", &input.uuid.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "lambda",
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
    http_request.add_header("x-amz-target", "UpdateEventSourceMapping");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: EventSourceMappingConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: EventSourceMappingConfiguration = serde_json::from_slice(body).unwrap();
                        output.uuid = body.uuid;
                        output.starting_position = body.starting_position;
                        output.starting_position_timestamp = body.starting_position_timestamp;
                        output.batch_size = body.batch_size;
                        output.maximum_batching_window_in_seconds = body.maximum_batching_window_in_seconds;
                        output.parallelization_factor = body.parallelization_factor;
                        output.event_source_arn = body.event_source_arn;
                        output.filter_criteria = body.filter_criteria;
                        output.function_arn = body.function_arn;
                        output.last_modified = body.last_modified;
                        output.last_processing_result = body.last_processing_result;
                        output.state = body.state;
                        output.state_transition_reason = body.state_transition_reason;
                        output.destination_config = body.destination_config;
                        output.topics = body.topics;
                        output.queues = body.queues;
                        output.source_access_configurations = body.source_access_configurations;
                        output.self_managed_event_source = body.self_managed_event_source;
                        output.maximum_record_age_in_seconds = body.maximum_record_age_in_seconds;
                        output.bisect_batch_on_function_error = body.bisect_batch_on_function_error;
                        output.maximum_retry_attempts = body.maximum_retry_attempts;
                        output.tumbling_window_in_seconds = body.tumbling_window_in_seconds;
                        output.function_response_types = body.function_response_types;
                        output.amazon_managed_kafka_event_source_config = body.amazon_managed_kafka_event_source_config;
                        output.self_managed_kafka_event_source_config = body.self_managed_kafka_event_source_config;

                        serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<EventSourceMappingConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_function_code(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateFunctionCodeRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __update_function_code(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_function_code(input: UpdateFunctionCodeRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/code");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "lambda",
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
    http_request.add_header("x-amz-target", "UpdateFunctionCode");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: FunctionConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: FunctionConfiguration = serde_json::from_slice(body).unwrap();
                        output.function_name = body.function_name;
                        output.function_arn = body.function_arn;
                        output.runtime = body.runtime;
                        output.role = body.role;
                        output.handler = body.handler;
                        output.code_size = body.code_size;
                        output.description = body.description;
                        output.timeout = body.timeout;
                        output.memory_size = body.memory_size;
                        output.last_modified = body.last_modified;
                        output.code_sha256 = body.code_sha256;
                        output.version = body.version;
                        output.vpc_config = body.vpc_config;
                        output.dead_letter_config = body.dead_letter_config;
                        output.environment = body.environment;
                        output.kms_key_arn = body.kms_key_arn;
                        output.tracing_config = body.tracing_config;
                        output.master_arn = body.master_arn;
                        output.revision_id = body.revision_id;
                        output.layers = body.layers;
                        output.state = body.state;
                        output.state_reason = body.state_reason;
                        output.state_reason_code = body.state_reason_code;
                        output.last_update_status = body.last_update_status;
                        output.last_update_status_reason = body.last_update_status_reason;
                        output.last_update_status_reason_code = body.last_update_status_reason_code;
                        output.file_system_configs = body.file_system_configs;
                        output.package_type = body.package_type;
                        output.image_config_response = body.image_config_response;
                        output.signing_profile_version_arn = body.signing_profile_version_arn;
                        output.signing_job_arn = body.signing_job_arn;
                        output.architectures = body.architectures;
                        output.ephemeral_storage = body.ephemeral_storage;

                        serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_function_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateFunctionConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __update_function_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_function_configuration(input: UpdateFunctionConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2015-03-31/functions/{FunctionName}/configuration");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "lambda",
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
    http_request.add_header("x-amz-target", "UpdateFunctionConfiguration");
    http_request.set_content_type(String::from("application/json"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: FunctionConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: FunctionConfiguration = serde_json::from_slice(body).unwrap();
                        output.function_name = body.function_name;
                        output.function_arn = body.function_arn;
                        output.runtime = body.runtime;
                        output.role = body.role;
                        output.handler = body.handler;
                        output.code_size = body.code_size;
                        output.description = body.description;
                        output.timeout = body.timeout;
                        output.memory_size = body.memory_size;
                        output.last_modified = body.last_modified;
                        output.code_sha256 = body.code_sha256;
                        output.version = body.version;
                        output.vpc_config = body.vpc_config;
                        output.dead_letter_config = body.dead_letter_config;
                        output.environment = body.environment;
                        output.kms_key_arn = body.kms_key_arn;
                        output.tracing_config = body.tracing_config;
                        output.master_arn = body.master_arn;
                        output.revision_id = body.revision_id;
                        output.layers = body.layers;
                        output.state = body.state;
                        output.state_reason = body.state_reason;
                        output.state_reason_code = body.state_reason_code;
                        output.last_update_status = body.last_update_status;
                        output.last_update_status_reason = body.last_update_status_reason;
                        output.last_update_status_reason_code = body.last_update_status_reason_code;
                        output.file_system_configs = body.file_system_configs;
                        output.package_type = body.package_type;
                        output.image_config_response = body.image_config_response;
                        output.signing_profile_version_arn = body.signing_profile_version_arn;
                        output.signing_job_arn = body.signing_job_arn;
                        output.architectures = body.architectures;
                        output.ephemeral_storage = body.ephemeral_storage;

                        serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<FunctionConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_function_event_invoke_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateFunctionEventInvokeConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __update_function_event_invoke_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_function_event_invoke_config(input: UpdateFunctionEventInvokeConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2019-09-25/functions/{FunctionName}/event-invoke-config");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

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
        "lambda",
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
    http_request.add_header("x-amz-target", "UpdateFunctionEventInvokeConfig");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: FunctionEventInvokeConfig = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: FunctionEventInvokeConfig = serde_json::from_slice(body).unwrap();
                        output.last_modified = body.last_modified;
                        output.function_arn = body.function_arn;
                        output.maximum_retry_attempts = body.maximum_retry_attempts;
                        output.maximum_event_age_in_seconds = body.maximum_event_age_in_seconds;
                        output.destination_config = body.destination_config;

                        serde_json::to_vec(&Result::<FunctionEventInvokeConfig, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<FunctionEventInvokeConfig, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<FunctionEventInvokeConfig, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_function_url_config(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateFunctionUrlConfigRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __update_function_url_config(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_function_url_config(input: UpdateFunctionUrlConfigRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/2021-10-31/functions/{FunctionName}/url");
    path = match path.find("{FunctionName}") {
        Some(_) => path.replace("{FunctionName}", &input.function_name.to_string()),
        None => path.to_string(),
    };

    let mut path_params: String = Default::default();
    path = match path.find('?') {
        None => path.to_string(),
        Some(idx) => {
            path_params = path.clone()[(idx + 1)..path.len()].to_string();
            path.clone()[..idx].to_string()
        },
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "lambda",
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
    http_request.add_header("x-amz-target", "UpdateFunctionUrlConfig");
    http_request.set_content_type(String::from("application/json"));

    if let Some(qualifier) = input.qualifier {
        http_request.add_param("Qualifier", &qualifier);
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateFunctionUrlConfigResponse = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body: UpdateFunctionUrlConfigResponse = serde_json::from_slice(body).unwrap();
                        output.function_url = body.function_url;
                        output.function_arn = body.function_arn;
                        output.auth_type = body.auth_type;
                        output.cors = body.cors;
                        output.creation_time = body.creation_time;
                        output.last_modified_time = body.last_modified_time;

                        serde_json::to_vec(&Result::<UpdateFunctionUrlConfigResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateFunctionUrlConfigResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateFunctionUrlConfigResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}
