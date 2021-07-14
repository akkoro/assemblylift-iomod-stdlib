// This file is generated!
// See https://github.com/akkoro/asml-aws-codegen

mod client;

use std::str::FromStr;

use assemblylift_core_iomod::iomod;
use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use hyper::StatusCode;
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
    iomod!(akkoro.aws.dynamodb => {
      batch_execute_statement => batch_execute_statement,
      batch_get_item => batch_get_item,
      batch_write_item => batch_write_item,
      create_backup => create_backup,
      create_global_table => create_global_table,
      create_table => create_table,
      delete_backup => delete_backup,
      delete_item => delete_item,
      delete_table => delete_table,
      describe_backup => describe_backup,
      describe_continuous_backups => describe_continuous_backups,
      describe_contributor_insights => describe_contributor_insights,
      describe_endpoints => describe_endpoints,
      describe_export => describe_export,
      describe_global_table => describe_global_table,
      describe_global_table_settings => describe_global_table_settings,
      describe_kinesis_streaming_destination => describe_kinesis_streaming_destination,
      describe_limits => describe_limits,
      describe_table => describe_table,
      describe_table_replica_auto_scaling => describe_table_replica_auto_scaling,
      describe_time_to_live => describe_time_to_live,
      disable_kinesis_streaming_destination => disable_kinesis_streaming_destination,
      enable_kinesis_streaming_destination => enable_kinesis_streaming_destination,
      execute_statement => execute_statement,
      execute_transaction => execute_transaction,
      export_table_to_point_in_time => export_table_to_point_in_time,
      get_item => get_item,
      list_backups => list_backups,
      list_contributor_insights => list_contributor_insights,
      list_exports => list_exports,
      list_global_tables => list_global_tables,
      list_tables => list_tables,
      list_tags_of_resource => list_tags_of_resource,
      put_item => put_item,
      query => query,
      restore_table_from_backup => restore_table_from_backup,
      restore_table_to_point_in_time => restore_table_to_point_in_time,
      scan => scan,
      tag_resource => tag_resource,
      transact_get_items => transact_get_items,
      transact_write_items => transact_write_items,
      untag_resource => untag_resource,
      update_continuous_backups => update_continuous_backups,
      update_contributor_insights => update_contributor_insights,
      update_global_table => update_global_table,
      update_global_table_settings => update_global_table_settings,
      update_item => update_item,
      update_table => update_table,
      update_table_replica_auto_scaling => update_table_replica_auto_scaling,
      update_time_to_live => update_time_to_live,
    });
}

#[allow(dead_code)]
pub fn batch_execute_statement(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: BatchExecuteStatementInput = serde_json::from_slice(input.as_slice()).unwrap();
    __batch_execute_statement(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __batch_execute_statement(input: BatchExecuteStatementInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.BatchExecuteStatement");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: BatchExecuteStatementOutput = Default::default();

                        let body = response.into_body();
                        let body: BatchExecuteStatementOutput = serde_json::from_slice(&*body).unwrap();
                        output.responses = body.responses;

                        serde_json::to_vec(&Result::<BatchExecuteStatementOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<BatchExecuteStatementOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<BatchExecuteStatementOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn batch_get_item(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: BatchGetItemInput = serde_json::from_slice(input.as_slice()).unwrap();
    __batch_get_item(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __batch_get_item(input: BatchGetItemInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.BatchGetItem");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: BatchGetItemOutput = Default::default();

                        let body = response.into_body();
                        let body: BatchGetItemOutput = serde_json::from_slice(&*body).unwrap();
                        output.responses = body.responses;
                        output.unprocessed_keys = body.unprocessed_keys;
                        output.consumed_capacity = body.consumed_capacity;

                        serde_json::to_vec(&Result::<BatchGetItemOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<BatchGetItemOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<BatchGetItemOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn batch_write_item(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: BatchWriteItemInput = serde_json::from_slice(input.as_slice()).unwrap();
    __batch_write_item(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __batch_write_item(input: BatchWriteItemInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.BatchWriteItem");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: BatchWriteItemOutput = Default::default();

                        let body = response.into_body();
                        let body: BatchWriteItemOutput = serde_json::from_slice(&*body).unwrap();
                        output.unprocessed_items = body.unprocessed_items;
                        output.item_collection_metrics = body.item_collection_metrics;
                        output.consumed_capacity = body.consumed_capacity;

                        serde_json::to_vec(&Result::<BatchWriteItemOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<BatchWriteItemOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<BatchWriteItemOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn create_backup(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateBackupInput = serde_json::from_slice(input.as_slice()).unwrap();
    __create_backup(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __create_backup(input: CreateBackupInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.CreateBackup");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: CreateBackupOutput = Default::default();

                        let body = response.into_body();
                        let body: CreateBackupOutput = serde_json::from_slice(&*body).unwrap();
                        output.backup_details = body.backup_details;

                        serde_json::to_vec(&Result::<CreateBackupOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<CreateBackupOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<CreateBackupOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn create_global_table(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateGlobalTableInput = serde_json::from_slice(input.as_slice()).unwrap();
    __create_global_table(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __create_global_table(input: CreateGlobalTableInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.CreateGlobalTable");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: CreateGlobalTableOutput = Default::default();

                        let body = response.into_body();
                        let body: CreateGlobalTableOutput = serde_json::from_slice(&*body).unwrap();
                        output.global_table_description = body.global_table_description;

                        serde_json::to_vec(&Result::<CreateGlobalTableOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<CreateGlobalTableOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<CreateGlobalTableOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn create_table(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateTableInput = serde_json::from_slice(input.as_slice()).unwrap();
    __create_table(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __create_table(input: CreateTableInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.CreateTable");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: CreateTableOutput = Default::default();

                        let body = response.into_body();
                        let body: CreateTableOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_description = body.table_description;

                        serde_json::to_vec(&Result::<CreateTableOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<CreateTableOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<CreateTableOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn delete_backup(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBackupInput = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_backup(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_backup(input: DeleteBackupInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DeleteBackup");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DeleteBackupOutput = Default::default();

                        let body = response.into_body();
                        let body: DeleteBackupOutput = serde_json::from_slice(&*body).unwrap();
                        output.backup_description = body.backup_description;

                        serde_json::to_vec(&Result::<DeleteBackupOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DeleteBackupOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DeleteBackupOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn delete_item(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteItemInput = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_item(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_item(input: DeleteItemInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DeleteItem");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DeleteItemOutput = Default::default();

                        let body = response.into_body();
                        let body: DeleteItemOutput = serde_json::from_slice(&*body).unwrap();
                        output.attributes = body.attributes;
                        output.consumed_capacity = body.consumed_capacity;
                        output.item_collection_metrics = body.item_collection_metrics;

                        serde_json::to_vec(&Result::<DeleteItemOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DeleteItemOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DeleteItemOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn delete_table(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteTableInput = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_table(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_table(input: DeleteTableInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DeleteTable");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DeleteTableOutput = Default::default();

                        let body = response.into_body();
                        let body: DeleteTableOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_description = body.table_description;

                        serde_json::to_vec(&Result::<DeleteTableOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DeleteTableOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DeleteTableOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_backup(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeBackupInput = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_backup(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_backup(input: DescribeBackupInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeBackup");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeBackupOutput = Default::default();

                        let body = response.into_body();
                        let body: DescribeBackupOutput = serde_json::from_slice(&*body).unwrap();
                        output.backup_description = body.backup_description;

                        serde_json::to_vec(&Result::<DescribeBackupOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeBackupOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeBackupOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_continuous_backups(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeContinuousBackupsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_continuous_backups(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_continuous_backups(input: DescribeContinuousBackupsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeContinuousBackups");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeContinuousBackupsOutput = Default::default();

                        let body = response.into_body();
                        let body: DescribeContinuousBackupsOutput = serde_json::from_slice(&*body).unwrap();
                        output.continuous_backups_description = body.continuous_backups_description;

                        serde_json::to_vec(&Result::<DescribeContinuousBackupsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeContinuousBackupsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeContinuousBackupsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_contributor_insights(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeContributorInsightsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_contributor_insights(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_contributor_insights(input: DescribeContributorInsightsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeContributorInsights");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeContributorInsightsOutput = Default::default();

                        let body = response.into_body();
                        let body: DescribeContributorInsightsOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_name = body.table_name;
                        output.index_name = body.index_name;
                        output.contributor_insights_rule_list = body.contributor_insights_rule_list;
                        output.contributor_insights_status = body.contributor_insights_status;
                        output.last_update_date_time = body.last_update_date_time;
                        output.failure_exception = body.failure_exception;

                        serde_json::to_vec(&Result::<DescribeContributorInsightsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeContributorInsightsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeContributorInsightsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_endpoints(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeEndpointsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_endpoints(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_endpoints(input: DescribeEndpointsRequest) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeEndpoints");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeEndpointsResponse = Default::default();

                        let body = response.into_body();
                        let body: DescribeEndpointsResponse = serde_json::from_slice(&*body).unwrap();
                        output.endpoints = body.endpoints;

                        serde_json::to_vec(&Result::<DescribeEndpointsResponse, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeEndpointsResponse, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeEndpointsResponse, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_export(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeExportInput = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_export(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_export(input: DescribeExportInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeExport");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeExportOutput = Default::default();

                        let body = response.into_body();
                        let body: DescribeExportOutput = serde_json::from_slice(&*body).unwrap();
                        output.export_description = body.export_description;

                        serde_json::to_vec(&Result::<DescribeExportOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeExportOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeExportOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_global_table(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeGlobalTableInput = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_global_table(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_global_table(input: DescribeGlobalTableInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeGlobalTable");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeGlobalTableOutput = Default::default();

                        let body = response.into_body();
                        let body: DescribeGlobalTableOutput = serde_json::from_slice(&*body).unwrap();
                        output.global_table_description = body.global_table_description;

                        serde_json::to_vec(&Result::<DescribeGlobalTableOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeGlobalTableOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeGlobalTableOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_global_table_settings(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeGlobalTableSettingsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_global_table_settings(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_global_table_settings(input: DescribeGlobalTableSettingsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeGlobalTableSettings");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeGlobalTableSettingsOutput = Default::default();

                        let body = response.into_body();
                        let body: DescribeGlobalTableSettingsOutput = serde_json::from_slice(&*body).unwrap();
                        output.global_table_name = body.global_table_name;
                        output.replica_settings = body.replica_settings;

                        serde_json::to_vec(&Result::<DescribeGlobalTableSettingsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeGlobalTableSettingsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeGlobalTableSettingsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_kinesis_streaming_destination(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeKinesisStreamingDestinationInput = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_kinesis_streaming_destination(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_kinesis_streaming_destination(input: DescribeKinesisStreamingDestinationInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeKinesisStreamingDestination");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeKinesisStreamingDestinationOutput = Default::default();

                        let body = response.into_body();
                        let body: DescribeKinesisStreamingDestinationOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_name = body.table_name;
                        output.kinesis_data_stream_destinations = body.kinesis_data_stream_destinations;

                        serde_json::to_vec(&Result::<DescribeKinesisStreamingDestinationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeKinesisStreamingDestinationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeKinesisStreamingDestinationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_limits(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeLimitsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_limits(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_limits(input: DescribeLimitsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeLimits");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeLimitsOutput = Default::default();

                        let body = response.into_body();
                        let body: DescribeLimitsOutput = serde_json::from_slice(&*body).unwrap();
                        output.account_max_read_capacity_units = body.account_max_read_capacity_units;
                        output.account_max_write_capacity_units = body.account_max_write_capacity_units;
                        output.table_max_read_capacity_units = body.table_max_read_capacity_units;
                        output.table_max_write_capacity_units = body.table_max_write_capacity_units;

                        serde_json::to_vec(&Result::<DescribeLimitsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeLimitsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeLimitsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_table(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeTableInput = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_table(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_table(input: DescribeTableInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeTable");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeTableOutput = Default::default();

                        let body = response.into_body();
                        let body: DescribeTableOutput = serde_json::from_slice(&*body).unwrap();
                        output.table = body.table;

                        serde_json::to_vec(&Result::<DescribeTableOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeTableOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeTableOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_table_replica_auto_scaling(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeTableReplicaAutoScalingInput = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_table_replica_auto_scaling(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_table_replica_auto_scaling(input: DescribeTableReplicaAutoScalingInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeTableReplicaAutoScaling");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeTableReplicaAutoScalingOutput = Default::default();

                        let body = response.into_body();
                        let body: DescribeTableReplicaAutoScalingOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_auto_scaling_description = body.table_auto_scaling_description;

                        serde_json::to_vec(&Result::<DescribeTableReplicaAutoScalingOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeTableReplicaAutoScalingOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeTableReplicaAutoScalingOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn describe_time_to_live(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DescribeTimeToLiveInput = serde_json::from_slice(input.as_slice()).unwrap();
    __describe_time_to_live(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __describe_time_to_live(input: DescribeTimeToLiveInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DescribeTimeToLive");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: DescribeTimeToLiveOutput = Default::default();

                        let body = response.into_body();
                        let body: DescribeTimeToLiveOutput = serde_json::from_slice(&*body).unwrap();
                        output.time_to_live_description = body.time_to_live_description;

                        serde_json::to_vec(&Result::<DescribeTimeToLiveOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DescribeTimeToLiveOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DescribeTimeToLiveOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn disable_kinesis_streaming_destination(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: KinesisStreamingDestinationInput = serde_json::from_slice(input.as_slice()).unwrap();
    __disable_kinesis_streaming_destination(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __disable_kinesis_streaming_destination(input: KinesisStreamingDestinationInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.DisableKinesisStreamingDestination");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: KinesisStreamingDestinationOutput = Default::default();

                        let body = response.into_body();
                        let body: KinesisStreamingDestinationOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_name = body.table_name;
                        output.stream_arn = body.stream_arn;
                        output.destination_status = body.destination_status;

                        serde_json::to_vec(&Result::<KinesisStreamingDestinationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<KinesisStreamingDestinationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<KinesisStreamingDestinationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn enable_kinesis_streaming_destination(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: KinesisStreamingDestinationInput = serde_json::from_slice(input.as_slice()).unwrap();
    __enable_kinesis_streaming_destination(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __enable_kinesis_streaming_destination(input: KinesisStreamingDestinationInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.EnableKinesisStreamingDestination");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: KinesisStreamingDestinationOutput = Default::default();

                        let body = response.into_body();
                        let body: KinesisStreamingDestinationOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_name = body.table_name;
                        output.stream_arn = body.stream_arn;
                        output.destination_status = body.destination_status;

                        serde_json::to_vec(&Result::<KinesisStreamingDestinationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<KinesisStreamingDestinationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<KinesisStreamingDestinationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn execute_statement(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ExecuteStatementInput = serde_json::from_slice(input.as_slice()).unwrap();
    __execute_statement(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __execute_statement(input: ExecuteStatementInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.ExecuteStatement");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ExecuteStatementOutput = Default::default();

                        let body = response.into_body();
                        let body: ExecuteStatementOutput = serde_json::from_slice(&*body).unwrap();
                        output.items = body.items;
                        output.next_token = body.next_token;

                        serde_json::to_vec(&Result::<ExecuteStatementOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ExecuteStatementOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ExecuteStatementOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn execute_transaction(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ExecuteTransactionInput = serde_json::from_slice(input.as_slice()).unwrap();
    __execute_transaction(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __execute_transaction(input: ExecuteTransactionInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.ExecuteTransaction");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ExecuteTransactionOutput = Default::default();

                        let body = response.into_body();
                        let body: ExecuteTransactionOutput = serde_json::from_slice(&*body).unwrap();
                        output.responses = body.responses;

                        serde_json::to_vec(&Result::<ExecuteTransactionOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ExecuteTransactionOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ExecuteTransactionOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn export_table_to_point_in_time(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ExportTableToPointInTimeInput = serde_json::from_slice(input.as_slice()).unwrap();
    __export_table_to_point_in_time(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __export_table_to_point_in_time(input: ExportTableToPointInTimeInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.ExportTableToPointInTime");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ExportTableToPointInTimeOutput = Default::default();

                        let body = response.into_body();
                        let body: ExportTableToPointInTimeOutput = serde_json::from_slice(&*body).unwrap();
                        output.export_description = body.export_description;

                        serde_json::to_vec(&Result::<ExportTableToPointInTimeOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ExportTableToPointInTimeOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ExportTableToPointInTimeOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn get_item(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetItemInput = serde_json::from_slice(input.as_slice()).unwrap();
    __get_item(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_item(input: GetItemInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.GetItem");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: GetItemOutput = Default::default();

                        let body = response.into_body();
                        let body: GetItemOutput = serde_json::from_slice(&*body).unwrap();
                        output.item = body.item;
                        output.consumed_capacity = body.consumed_capacity;

                        serde_json::to_vec(&Result::<GetItemOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetItemOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetItemOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_backups(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBackupsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __list_backups(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_backups(input: ListBackupsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.ListBackups");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListBackupsOutput = Default::default();

                        let body = response.into_body();
                        let body: ListBackupsOutput = serde_json::from_slice(&*body).unwrap();
                        output.backup_summaries = body.backup_summaries;
                        output.last_evaluated_backup_arn = body.last_evaluated_backup_arn;

                        serde_json::to_vec(&Result::<ListBackupsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListBackupsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListBackupsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_contributor_insights(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListContributorInsightsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __list_contributor_insights(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_contributor_insights(input: ListContributorInsightsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.ListContributorInsights");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListContributorInsightsOutput = Default::default();

                        let body = response.into_body();
                        let body: ListContributorInsightsOutput = serde_json::from_slice(&*body).unwrap();
                        output.contributor_insights_summaries = body.contributor_insights_summaries;
                        output.next_token = body.next_token;

                        serde_json::to_vec(&Result::<ListContributorInsightsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListContributorInsightsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListContributorInsightsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_exports(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListExportsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __list_exports(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_exports(input: ListExportsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.ListExports");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListExportsOutput = Default::default();

                        let body = response.into_body();
                        let body: ListExportsOutput = serde_json::from_slice(&*body).unwrap();
                        output.export_summaries = body.export_summaries;
                        output.next_token = body.next_token;

                        serde_json::to_vec(&Result::<ListExportsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListExportsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListExportsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_global_tables(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListGlobalTablesInput = serde_json::from_slice(input.as_slice()).unwrap();
    __list_global_tables(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_global_tables(input: ListGlobalTablesInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.ListGlobalTables");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListGlobalTablesOutput = Default::default();

                        let body = response.into_body();
                        let body: ListGlobalTablesOutput = serde_json::from_slice(&*body).unwrap();
                        output.global_tables = body.global_tables;
                        output.last_evaluated_global_table_name = body.last_evaluated_global_table_name;

                        serde_json::to_vec(&Result::<ListGlobalTablesOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListGlobalTablesOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListGlobalTablesOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_tables(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListTablesInput = serde_json::from_slice(input.as_slice()).unwrap();
    __list_tables(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_tables(input: ListTablesInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.ListTables");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListTablesOutput = Default::default();

                        let body = response.into_body();
                        let body: ListTablesOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_names = body.table_names;
                        output.last_evaluated_table_name = body.last_evaluated_table_name;

                        serde_json::to_vec(&Result::<ListTablesOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListTablesOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListTablesOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn list_tags_of_resource(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListTagsOfResourceInput = serde_json::from_slice(input.as_slice()).unwrap();
    __list_tags_of_resource(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_tags_of_resource(input: ListTagsOfResourceInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.ListTagsOfResource");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ListTagsOfResourceOutput = Default::default();

                        let body = response.into_body();
                        let body: ListTagsOfResourceOutput = serde_json::from_slice(&*body).unwrap();
                        output.tags = body.tags;
                        output.next_token = body.next_token;

                        serde_json::to_vec(&Result::<ListTagsOfResourceOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListTagsOfResourceOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListTagsOfResourceOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn put_item(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutItemInput = serde_json::from_slice(input.as_slice()).unwrap();
    __put_item(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_item(input: PutItemInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.PutItem");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: PutItemOutput = Default::default();

                        let body = response.into_body();
                        let body: PutItemOutput = serde_json::from_slice(&*body).unwrap();
                        output.attributes = body.attributes;
                        output.consumed_capacity = body.consumed_capacity;
                        output.item_collection_metrics = body.item_collection_metrics;

                        serde_json::to_vec(&Result::<PutItemOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PutItemOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PutItemOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn query(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: QueryInput = serde_json::from_slice(input.as_slice()).unwrap();
    __query(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __query(input: QueryInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.Query");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: QueryOutput = Default::default();

                        let body = response.into_body();
                        let body: QueryOutput = serde_json::from_slice(&*body).unwrap();
                        output.items = body.items;
                        output.count = body.count;
                        output.scanned_count = body.scanned_count;
                        output.last_evaluated_key = body.last_evaluated_key;
                        output.consumed_capacity = body.consumed_capacity;

                        serde_json::to_vec(&Result::<QueryOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<QueryOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<QueryOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn restore_table_from_backup(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: RestoreTableFromBackupInput = serde_json::from_slice(input.as_slice()).unwrap();
    __restore_table_from_backup(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __restore_table_from_backup(input: RestoreTableFromBackupInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.RestoreTableFromBackup");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: RestoreTableFromBackupOutput = Default::default();

                        let body = response.into_body();
                        let body: RestoreTableFromBackupOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_description = body.table_description;

                        serde_json::to_vec(&Result::<RestoreTableFromBackupOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<RestoreTableFromBackupOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<RestoreTableFromBackupOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn restore_table_to_point_in_time(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: RestoreTableToPointInTimeInput = serde_json::from_slice(input.as_slice()).unwrap();
    __restore_table_to_point_in_time(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __restore_table_to_point_in_time(input: RestoreTableToPointInTimeInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.RestoreTableToPointInTime");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: RestoreTableToPointInTimeOutput = Default::default();

                        let body = response.into_body();
                        let body: RestoreTableToPointInTimeOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_description = body.table_description;

                        serde_json::to_vec(&Result::<RestoreTableToPointInTimeOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<RestoreTableToPointInTimeOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<RestoreTableToPointInTimeOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn scan(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ScanInput = serde_json::from_slice(input.as_slice()).unwrap();
    __scan(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __scan(input: ScanInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.Scan");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: ScanOutput = Default::default();

                        let body = response.into_body();
                        let body: ScanOutput = serde_json::from_slice(&*body).unwrap();
                        output.items = body.items;
                        output.count = body.count;
                        output.scanned_count = body.scanned_count;
                        output.last_evaluated_key = body.last_evaluated_key;
                        output.consumed_capacity = body.consumed_capacity;

                        serde_json::to_vec(&Result::<ScanOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ScanOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ScanOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn tag_resource(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: TagResourceInput = serde_json::from_slice(input.as_slice()).unwrap();
    __tag_resource(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __tag_resource(input: TagResourceInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.TagResource");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


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
pub fn transact_get_items(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: TransactGetItemsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __transact_get_items(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __transact_get_items(input: TransactGetItemsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.TransactGetItems");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: TransactGetItemsOutput = Default::default();

                        let body = response.into_body();
                        let body: TransactGetItemsOutput = serde_json::from_slice(&*body).unwrap();
                        output.consumed_capacity = body.consumed_capacity;
                        output.responses = body.responses;

                        serde_json::to_vec(&Result::<TransactGetItemsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<TransactGetItemsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<TransactGetItemsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn transact_write_items(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: TransactWriteItemsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __transact_write_items(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __transact_write_items(input: TransactWriteItemsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.TransactWriteItems");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: TransactWriteItemsOutput = Default::default();

                        let body = response.into_body();
                        let body: TransactWriteItemsOutput = serde_json::from_slice(&*body).unwrap();
                        output.consumed_capacity = body.consumed_capacity;
                        output.item_collection_metrics = body.item_collection_metrics;

                        serde_json::to_vec(&Result::<TransactWriteItemsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<TransactWriteItemsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<TransactWriteItemsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn untag_resource(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UntagResourceInput = serde_json::from_slice(input.as_slice()).unwrap();
    __untag_resource(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __untag_resource(input: UntagResourceInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.UntagResource");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


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
pub fn update_continuous_backups(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateContinuousBackupsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __update_continuous_backups(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_continuous_backups(input: UpdateContinuousBackupsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.UpdateContinuousBackups");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateContinuousBackupsOutput = Default::default();

                        let body = response.into_body();
                        let body: UpdateContinuousBackupsOutput = serde_json::from_slice(&*body).unwrap();
                        output.continuous_backups_description = body.continuous_backups_description;

                        serde_json::to_vec(&Result::<UpdateContinuousBackupsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateContinuousBackupsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateContinuousBackupsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_contributor_insights(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateContributorInsightsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __update_contributor_insights(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_contributor_insights(input: UpdateContributorInsightsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.UpdateContributorInsights");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateContributorInsightsOutput = Default::default();

                        let body = response.into_body();
                        let body: UpdateContributorInsightsOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_name = body.table_name;
                        output.index_name = body.index_name;
                        output.contributor_insights_status = body.contributor_insights_status;

                        serde_json::to_vec(&Result::<UpdateContributorInsightsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateContributorInsightsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateContributorInsightsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_global_table(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateGlobalTableInput = serde_json::from_slice(input.as_slice()).unwrap();
    __update_global_table(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_global_table(input: UpdateGlobalTableInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.UpdateGlobalTable");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateGlobalTableOutput = Default::default();

                        let body = response.into_body();
                        let body: UpdateGlobalTableOutput = serde_json::from_slice(&*body).unwrap();
                        output.global_table_description = body.global_table_description;

                        serde_json::to_vec(&Result::<UpdateGlobalTableOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateGlobalTableOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateGlobalTableOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_global_table_settings(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateGlobalTableSettingsInput = serde_json::from_slice(input.as_slice()).unwrap();
    __update_global_table_settings(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_global_table_settings(input: UpdateGlobalTableSettingsInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.UpdateGlobalTableSettings");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateGlobalTableSettingsOutput = Default::default();

                        let body = response.into_body();
                        let body: UpdateGlobalTableSettingsOutput = serde_json::from_slice(&*body).unwrap();
                        output.global_table_name = body.global_table_name;
                        output.replica_settings = body.replica_settings;

                        serde_json::to_vec(&Result::<UpdateGlobalTableSettingsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateGlobalTableSettingsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateGlobalTableSettingsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_item(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateItemInput = serde_json::from_slice(input.as_slice()).unwrap();
    __update_item(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_item(input: UpdateItemInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.UpdateItem");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateItemOutput = Default::default();

                        let body = response.into_body();
                        let body: UpdateItemOutput = serde_json::from_slice(&*body).unwrap();
                        output.attributes = body.attributes;
                        output.consumed_capacity = body.consumed_capacity;
                        output.item_collection_metrics = body.item_collection_metrics;

                        serde_json::to_vec(&Result::<UpdateItemOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateItemOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateItemOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_table(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateTableInput = serde_json::from_slice(input.as_slice()).unwrap();
    __update_table(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_table(input: UpdateTableInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.UpdateTable");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateTableOutput = Default::default();

                        let body = response.into_body();
                        let body: UpdateTableOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_description = body.table_description;

                        serde_json::to_vec(&Result::<UpdateTableOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateTableOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateTableOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_table_replica_auto_scaling(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateTableReplicaAutoScalingInput = serde_json::from_slice(input.as_slice()).unwrap();
    __update_table_replica_auto_scaling(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_table_replica_auto_scaling(input: UpdateTableReplicaAutoScalingInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.UpdateTableReplicaAutoScaling");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateTableReplicaAutoScalingOutput = Default::default();

                        let body = response.into_body();
                        let body: UpdateTableReplicaAutoScalingOutput = serde_json::from_slice(&*body).unwrap();
                        output.table_auto_scaling_description = body.table_auto_scaling_description;

                        serde_json::to_vec(&Result::<UpdateTableReplicaAutoScalingOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateTableReplicaAutoScalingOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateTableReplicaAutoScalingOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

#[allow(dead_code)]
pub fn update_time_to_live(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UpdateTimeToLiveInput = serde_json::from_slice(input.as_slice()).unwrap();
    __update_time_to_live(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __update_time_to_live(input: UpdateTimeToLiveInput) -> BoxFuture<'static, Vec<u8>> {
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
        "dynamodb",
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
    http_request.add_header("x-amz-target", "DynamoDB_20120810.UpdateTimeToLive");
    http_request.set_content_type(String::from("application/x-amz-json-1.0"));


    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK|StatusCode::CREATED|StatusCode::ACCEPTED => {
                        let mut output: UpdateTimeToLiveOutput = Default::default();

                        let body = response.into_body();
                        let body: UpdateTimeToLiveOutput = serde_json::from_slice(&*body).unwrap();
                        output.time_to_live_specification = body.time_to_live_specification;

                        serde_json::to_vec(&Result::<UpdateTimeToLiveOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UpdateTimeToLiveOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                            .unwrap()
                    }
                }
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UpdateTimeToLiveOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}
