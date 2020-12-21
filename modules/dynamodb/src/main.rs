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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("Statements", serde_json::to_string(&input.statements).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: BatchExecuteStatementOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("RequestItems", serde_json::to_string(&input.request_items).unwrap());
    if let Some(return_consumed_capacity) = input.return_consumed_capacity {
        body.insert("ReturnConsumedCapacity", serde_json::to_string(&return_consumed_capacity).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: BatchGetItemOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("RequestItems", serde_json::to_string(&input.request_items).unwrap());
    if let Some(return_consumed_capacity) = input.return_consumed_capacity {
        body.insert("ReturnConsumedCapacity", serde_json::to_string(&return_consumed_capacity).unwrap());
    };
    if let Some(return_item_collection_metrics) = input.return_item_collection_metrics {
        body.insert("ReturnItemCollectionMetrics", serde_json::to_string(&return_item_collection_metrics).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: BatchWriteItemOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    body.insert("BackupName", serde_json::to_string(&input.backup_name).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: CreateBackupOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("GlobalTableName", serde_json::to_string(&input.global_table_name).unwrap());
    body.insert("ReplicationGroup", serde_json::to_string(&input.replication_group).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: CreateGlobalTableOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("AttributeDefinitions", serde_json::to_string(&input.attribute_definitions).unwrap());
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    body.insert("KeySchema", serde_json::to_string(&input.key_schema).unwrap());
    if let Some(local_secondary_indexes) = input.local_secondary_indexes {
        body.insert("LocalSecondaryIndexes", serde_json::to_string(&local_secondary_indexes).unwrap());
    };
    if let Some(global_secondary_indexes) = input.global_secondary_indexes {
        body.insert("GlobalSecondaryIndexes", serde_json::to_string(&global_secondary_indexes).unwrap());
    };
    if let Some(billing_mode) = input.billing_mode {
        body.insert("BillingMode", serde_json::to_string(&billing_mode).unwrap());
    };
    if let Some(provisioned_throughput) = input.provisioned_throughput {
        body.insert("ProvisionedThroughput", serde_json::to_string(&provisioned_throughput).unwrap());
    };
    if let Some(stream_specification) = input.stream_specification {
        body.insert("StreamSpecification", serde_json::to_string(&stream_specification).unwrap());
    };
    if let Some(sse_specification) = input.sse_specification {
        body.insert("SSESpecification", serde_json::to_string(&sse_specification).unwrap());
    };
    if let Some(tags) = input.tags {
        body.insert("Tags", serde_json::to_string(&tags).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: CreateTableOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("BackupArn", serde_json::to_string(&input.backup_arn).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DeleteBackupOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    body.insert("Key", serde_json::to_string(&input.key).unwrap());
    if let Some(expected) = input.expected {
        body.insert("Expected", serde_json::to_string(&expected).unwrap());
    };
    if let Some(conditional_operator) = input.conditional_operator {
        body.insert("ConditionalOperator", serde_json::to_string(&conditional_operator).unwrap());
    };
    if let Some(return_values) = input.return_values {
        body.insert("ReturnValues", serde_json::to_string(&return_values).unwrap());
    };
    if let Some(return_consumed_capacity) = input.return_consumed_capacity {
        body.insert("ReturnConsumedCapacity", serde_json::to_string(&return_consumed_capacity).unwrap());
    };
    if let Some(return_item_collection_metrics) = input.return_item_collection_metrics {
        body.insert("ReturnItemCollectionMetrics", serde_json::to_string(&return_item_collection_metrics).unwrap());
    };
    if let Some(condition_expression) = input.condition_expression {
        body.insert("ConditionExpression", serde_json::to_string(&condition_expression).unwrap());
    };
    if let Some(expression_attribute_names) = input.expression_attribute_names {
        body.insert("ExpressionAttributeNames", serde_json::to_string(&expression_attribute_names).unwrap());
    };
    if let Some(expression_attribute_values) = input.expression_attribute_values {
        body.insert("ExpressionAttributeValues", serde_json::to_string(&expression_attribute_values).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DeleteItemOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DeleteTableOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("BackupArn", serde_json::to_string(&input.backup_arn).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeBackupOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeContinuousBackupsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if let Some(index_name) = input.index_name {
        body.insert("IndexName", serde_json::to_string(&index_name).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeContributorInsightsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeEndpointsResponse = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("ExportArn", serde_json::to_string(&input.export_arn).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeExportOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("GlobalTableName", serde_json::to_string(&input.global_table_name).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeGlobalTableOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("GlobalTableName", serde_json::to_string(&input.global_table_name).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeGlobalTableSettingsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeKinesisStreamingDestinationOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeLimitsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeTableOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeTableReplicaAutoScalingOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: DescribeTimeToLiveOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    body.insert("StreamArn", serde_json::to_string(&input.stream_arn).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: KinesisStreamingDestinationOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    body.insert("StreamArn", serde_json::to_string(&input.stream_arn).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: KinesisStreamingDestinationOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("Statement", serde_json::to_string(&input.statement).unwrap());
    if let Some(parameters) = input.parameters {
        body.insert("Parameters", serde_json::to_string(&parameters).unwrap());
    };
    if let Some(consistent_read) = input.consistent_read {
        body.insert("ConsistentRead", serde_json::to_string(&consistent_read).unwrap());
    };
    if let Some(next_token) = input.next_token {
        body.insert("NextToken", serde_json::to_string(&next_token).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: ExecuteStatementOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TransactStatements", serde_json::to_string(&input.transact_statements).unwrap());
    if let Some(client_request_token) = input.client_request_token {
        body.insert("ClientRequestToken", serde_json::to_string(&client_request_token).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: ExecuteTransactionOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableArn", serde_json::to_string(&input.table_arn).unwrap());
    if let Some(export_time) = input.export_time {
        body.insert("ExportTime", serde_json::to_string(&export_time).unwrap());
    };
    if let Some(client_token) = input.client_token {
        body.insert("ClientToken", serde_json::to_string(&client_token).unwrap());
    };
    body.insert("S3Bucket", serde_json::to_string(&input.s3_bucket).unwrap());
    if let Some(s3_bucket_owner) = input.s3_bucket_owner {
        body.insert("S3BucketOwner", serde_json::to_string(&s3_bucket_owner).unwrap());
    };
    if let Some(s3_prefix) = input.s3_prefix {
        body.insert("S3Prefix", serde_json::to_string(&s3_prefix).unwrap());
    };
    if let Some(s3_sse_algorithm) = input.s3_sse_algorithm {
        body.insert("S3SseAlgorithm", serde_json::to_string(&s3_sse_algorithm).unwrap());
    };
    if let Some(s3_sse_kms_key_id) = input.s3_sse_kms_key_id {
        body.insert("S3SseKmsKeyId", serde_json::to_string(&s3_sse_kms_key_id).unwrap());
    };
    if let Some(export_format) = input.export_format {
        body.insert("ExportFormat", serde_json::to_string(&export_format).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: ExportTableToPointInTimeOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    body.insert("Key", serde_json::to_string(&input.key).unwrap());
    if let Some(attributes_to_get) = input.attributes_to_get {
        body.insert("AttributesToGet", serde_json::to_string(&attributes_to_get).unwrap());
    };
    if let Some(consistent_read) = input.consistent_read {
        body.insert("ConsistentRead", serde_json::to_string(&consistent_read).unwrap());
    };
    if let Some(return_consumed_capacity) = input.return_consumed_capacity {
        body.insert("ReturnConsumedCapacity", serde_json::to_string(&return_consumed_capacity).unwrap());
    };
    if let Some(projection_expression) = input.projection_expression {
        body.insert("ProjectionExpression", serde_json::to_string(&projection_expression).unwrap());
    };
    if let Some(expression_attribute_names) = input.expression_attribute_names {
        body.insert("ExpressionAttributeNames", serde_json::to_string(&expression_attribute_names).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: GetItemOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(table_name) = input.table_name {
        body.insert("TableName", serde_json::to_string(&table_name).unwrap());
    };
    if let Some(limit) = input.limit {
        body.insert("Limit", serde_json::to_string(&limit).unwrap());
    };
    if let Some(time_range_lower_bound) = input.time_range_lower_bound {
        body.insert("TimeRangeLowerBound", serde_json::to_string(&time_range_lower_bound).unwrap());
    };
    if let Some(time_range_upper_bound) = input.time_range_upper_bound {
        body.insert("TimeRangeUpperBound", serde_json::to_string(&time_range_upper_bound).unwrap());
    };
    if let Some(exclusive_start_backup_arn) = input.exclusive_start_backup_arn {
        body.insert("ExclusiveStartBackupArn", serde_json::to_string(&exclusive_start_backup_arn).unwrap());
    };
    if let Some(backup_type) = input.backup_type {
        body.insert("BackupType", serde_json::to_string(&backup_type).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: ListBackupsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(table_name) = input.table_name {
        body.insert("TableName", serde_json::to_string(&table_name).unwrap());
    };
    if let Some(next_token) = input.next_token {
        body.insert("NextToken", serde_json::to_string(&next_token).unwrap());
    };
    if let Some(max_results) = input.max_results {
        body.insert("MaxResults", serde_json::to_string(&max_results).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: ListContributorInsightsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(table_arn) = input.table_arn {
        body.insert("TableArn", serde_json::to_string(&table_arn).unwrap());
    };
    if let Some(max_results) = input.max_results {
        body.insert("MaxResults", serde_json::to_string(&max_results).unwrap());
    };
    if let Some(next_token) = input.next_token {
        body.insert("NextToken", serde_json::to_string(&next_token).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: ListExportsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(exclusive_start_global_table_name) = input.exclusive_start_global_table_name {
        body.insert("ExclusiveStartGlobalTableName", serde_json::to_string(&exclusive_start_global_table_name).unwrap());
    };
    if let Some(limit) = input.limit {
        body.insert("Limit", serde_json::to_string(&limit).unwrap());
    };
    if let Some(region_name) = input.region_name {
        body.insert("RegionName", serde_json::to_string(&region_name).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: ListGlobalTablesOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(exclusive_start_table_name) = input.exclusive_start_table_name {
        body.insert("ExclusiveStartTableName", serde_json::to_string(&exclusive_start_table_name).unwrap());
    };
    if let Some(limit) = input.limit {
        body.insert("Limit", serde_json::to_string(&limit).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: ListTablesOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("ResourceArn", serde_json::to_string(&input.resource_arn).unwrap());
    if let Some(next_token) = input.next_token {
        body.insert("NextToken", serde_json::to_string(&next_token).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: ListTagsOfResourceOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    body.insert("Item", serde_json::to_string(&input.item).unwrap());
    if let Some(expected) = input.expected {
        body.insert("Expected", serde_json::to_string(&expected).unwrap());
    };
    if let Some(return_values) = input.return_values {
        body.insert("ReturnValues", serde_json::to_string(&return_values).unwrap());
    };
    if let Some(return_consumed_capacity) = input.return_consumed_capacity {
        body.insert("ReturnConsumedCapacity", serde_json::to_string(&return_consumed_capacity).unwrap());
    };
    if let Some(return_item_collection_metrics) = input.return_item_collection_metrics {
        body.insert("ReturnItemCollectionMetrics", serde_json::to_string(&return_item_collection_metrics).unwrap());
    };
    if let Some(conditional_operator) = input.conditional_operator {
        body.insert("ConditionalOperator", serde_json::to_string(&conditional_operator).unwrap());
    };
    if let Some(condition_expression) = input.condition_expression {
        body.insert("ConditionExpression", serde_json::to_string(&condition_expression).unwrap());
    };
    if let Some(expression_attribute_names) = input.expression_attribute_names {
        body.insert("ExpressionAttributeNames", serde_json::to_string(&expression_attribute_names).unwrap());
    };
    if let Some(expression_attribute_values) = input.expression_attribute_values {
        body.insert("ExpressionAttributeValues", serde_json::to_string(&expression_attribute_values).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: PutItemOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if let Some(index_name) = input.index_name {
        body.insert("IndexName", serde_json::to_string(&index_name).unwrap());
    };
    if let Some(select) = input.select {
        body.insert("Select", serde_json::to_string(&select).unwrap());
    };
    if let Some(attributes_to_get) = input.attributes_to_get {
        body.insert("AttributesToGet", serde_json::to_string(&attributes_to_get).unwrap());
    };
    if let Some(limit) = input.limit {
        body.insert("Limit", serde_json::to_string(&limit).unwrap());
    };
    if let Some(consistent_read) = input.consistent_read {
        body.insert("ConsistentRead", serde_json::to_string(&consistent_read).unwrap());
    };
    if let Some(key_conditions) = input.key_conditions {
        body.insert("KeyConditions", serde_json::to_string(&key_conditions).unwrap());
    };
    if let Some(query_filter) = input.query_filter {
        body.insert("QueryFilter", serde_json::to_string(&query_filter).unwrap());
    };
    if let Some(conditional_operator) = input.conditional_operator {
        body.insert("ConditionalOperator", serde_json::to_string(&conditional_operator).unwrap());
    };
    if let Some(scan_index_forward) = input.scan_index_forward {
        body.insert("ScanIndexForward", serde_json::to_string(&scan_index_forward).unwrap());
    };
    if let Some(exclusive_start_key) = input.exclusive_start_key {
        body.insert("ExclusiveStartKey", serde_json::to_string(&exclusive_start_key).unwrap());
    };
    if let Some(return_consumed_capacity) = input.return_consumed_capacity {
        body.insert("ReturnConsumedCapacity", serde_json::to_string(&return_consumed_capacity).unwrap());
    };
    if let Some(projection_expression) = input.projection_expression {
        body.insert("ProjectionExpression", serde_json::to_string(&projection_expression).unwrap());
    };
    if let Some(filter_expression) = input.filter_expression {
        body.insert("FilterExpression", serde_json::to_string(&filter_expression).unwrap());
    };
    if let Some(key_condition_expression) = input.key_condition_expression {
        body.insert("KeyConditionExpression", serde_json::to_string(&key_condition_expression).unwrap());
    };
    if let Some(expression_attribute_names) = input.expression_attribute_names {
        body.insert("ExpressionAttributeNames", serde_json::to_string(&expression_attribute_names).unwrap());
    };
    if let Some(expression_attribute_values) = input.expression_attribute_values {
        body.insert("ExpressionAttributeValues", serde_json::to_string(&expression_attribute_values).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: QueryOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TargetTableName", serde_json::to_string(&input.target_table_name).unwrap());
    body.insert("BackupArn", serde_json::to_string(&input.backup_arn).unwrap());
    if let Some(billing_mode_override) = input.billing_mode_override {
        body.insert("BillingModeOverride", serde_json::to_string(&billing_mode_override).unwrap());
    };
    if let Some(global_secondary_index_override) = input.global_secondary_index_override {
        body.insert("GlobalSecondaryIndexOverride", serde_json::to_string(&global_secondary_index_override).unwrap());
    };
    if let Some(local_secondary_index_override) = input.local_secondary_index_override {
        body.insert("LocalSecondaryIndexOverride", serde_json::to_string(&local_secondary_index_override).unwrap());
    };
    if let Some(provisioned_throughput_override) = input.provisioned_throughput_override {
        body.insert("ProvisionedThroughputOverride", serde_json::to_string(&provisioned_throughput_override).unwrap());
    };
    if let Some(sse_specification_override) = input.sse_specification_override {
        body.insert("SSESpecificationOverride", serde_json::to_string(&sse_specification_override).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: RestoreTableFromBackupOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(source_table_arn) = input.source_table_arn {
        body.insert("SourceTableArn", serde_json::to_string(&source_table_arn).unwrap());
    };
    if let Some(source_table_name) = input.source_table_name {
        body.insert("SourceTableName", serde_json::to_string(&source_table_name).unwrap());
    };
    body.insert("TargetTableName", serde_json::to_string(&input.target_table_name).unwrap());
    if let Some(use_latest_restorable_time) = input.use_latest_restorable_time {
        body.insert("UseLatestRestorableTime", serde_json::to_string(&use_latest_restorable_time).unwrap());
    };
    if let Some(restore_date_time) = input.restore_date_time {
        body.insert("RestoreDateTime", serde_json::to_string(&restore_date_time).unwrap());
    };
    if let Some(billing_mode_override) = input.billing_mode_override {
        body.insert("BillingModeOverride", serde_json::to_string(&billing_mode_override).unwrap());
    };
    if let Some(global_secondary_index_override) = input.global_secondary_index_override {
        body.insert("GlobalSecondaryIndexOverride", serde_json::to_string(&global_secondary_index_override).unwrap());
    };
    if let Some(local_secondary_index_override) = input.local_secondary_index_override {
        body.insert("LocalSecondaryIndexOverride", serde_json::to_string(&local_secondary_index_override).unwrap());
    };
    if let Some(provisioned_throughput_override) = input.provisioned_throughput_override {
        body.insert("ProvisionedThroughputOverride", serde_json::to_string(&provisioned_throughput_override).unwrap());
    };
    if let Some(sse_specification_override) = input.sse_specification_override {
        body.insert("SSESpecificationOverride", serde_json::to_string(&sse_specification_override).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: RestoreTableToPointInTimeOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if let Some(index_name) = input.index_name {
        body.insert("IndexName", serde_json::to_string(&index_name).unwrap());
    };
    if let Some(attributes_to_get) = input.attributes_to_get {
        body.insert("AttributesToGet", serde_json::to_string(&attributes_to_get).unwrap());
    };
    if let Some(limit) = input.limit {
        body.insert("Limit", serde_json::to_string(&limit).unwrap());
    };
    if let Some(select) = input.select {
        body.insert("Select", serde_json::to_string(&select).unwrap());
    };
    if let Some(scan_filter) = input.scan_filter {
        body.insert("ScanFilter", serde_json::to_string(&scan_filter).unwrap());
    };
    if let Some(conditional_operator) = input.conditional_operator {
        body.insert("ConditionalOperator", serde_json::to_string(&conditional_operator).unwrap());
    };
    if let Some(exclusive_start_key) = input.exclusive_start_key {
        body.insert("ExclusiveStartKey", serde_json::to_string(&exclusive_start_key).unwrap());
    };
    if let Some(return_consumed_capacity) = input.return_consumed_capacity {
        body.insert("ReturnConsumedCapacity", serde_json::to_string(&return_consumed_capacity).unwrap());
    };
    if let Some(total_segments) = input.total_segments {
        body.insert("TotalSegments", serde_json::to_string(&total_segments).unwrap());
    };
    if let Some(segment) = input.segment {
        body.insert("Segment", serde_json::to_string(&segment).unwrap());
    };
    if let Some(projection_expression) = input.projection_expression {
        body.insert("ProjectionExpression", serde_json::to_string(&projection_expression).unwrap());
    };
    if let Some(filter_expression) = input.filter_expression {
        body.insert("FilterExpression", serde_json::to_string(&filter_expression).unwrap());
    };
    if let Some(expression_attribute_names) = input.expression_attribute_names {
        body.insert("ExpressionAttributeNames", serde_json::to_string(&expression_attribute_names).unwrap());
    };
    if let Some(expression_attribute_values) = input.expression_attribute_values {
        body.insert("ExpressionAttributeValues", serde_json::to_string(&expression_attribute_values).unwrap());
    };
    if let Some(consistent_read) = input.consistent_read {
        body.insert("ConsistentRead", serde_json::to_string(&consistent_read).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: ScanOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("ResourceArn", serde_json::to_string(&input.resource_arn).unwrap());
    body.insert("Tags", serde_json::to_string(&input.tags).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: () = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TransactItems", serde_json::to_string(&input.transact_items).unwrap());
    if let Some(return_consumed_capacity) = input.return_consumed_capacity {
        body.insert("ReturnConsumedCapacity", serde_json::to_string(&return_consumed_capacity).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: TransactGetItemsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TransactItems", serde_json::to_string(&input.transact_items).unwrap());
    if let Some(return_consumed_capacity) = input.return_consumed_capacity {
        body.insert("ReturnConsumedCapacity", serde_json::to_string(&return_consumed_capacity).unwrap());
    };
    if let Some(return_item_collection_metrics) = input.return_item_collection_metrics {
        body.insert("ReturnItemCollectionMetrics", serde_json::to_string(&return_item_collection_metrics).unwrap());
    };
    if let Some(client_request_token) = input.client_request_token {
        body.insert("ClientRequestToken", serde_json::to_string(&client_request_token).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: TransactWriteItemsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("ResourceArn", serde_json::to_string(&input.resource_arn).unwrap());
    body.insert("TagKeys", serde_json::to_string(&input.tag_keys).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: () = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    body.insert("PointInTimeRecoverySpecification", serde_json::to_string(&input.point_in_time_recovery_specification).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: UpdateContinuousBackupsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if let Some(index_name) = input.index_name {
        body.insert("IndexName", serde_json::to_string(&index_name).unwrap());
    };
    body.insert("ContributorInsightsAction", serde_json::to_string(&input.contributor_insights_action).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: UpdateContributorInsightsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("GlobalTableName", serde_json::to_string(&input.global_table_name).unwrap());
    body.insert("ReplicaUpdates", serde_json::to_string(&input.replica_updates).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: UpdateGlobalTableOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("GlobalTableName", serde_json::to_string(&input.global_table_name).unwrap());
    if let Some(global_table_billing_mode) = input.global_table_billing_mode {
        body.insert("GlobalTableBillingMode", serde_json::to_string(&global_table_billing_mode).unwrap());
    };
    if let Some(global_table_provisioned_write_capacity_units) = input.global_table_provisioned_write_capacity_units {
        body.insert("GlobalTableProvisionedWriteCapacityUnits", serde_json::to_string(&global_table_provisioned_write_capacity_units).unwrap());
    };
    if let Some(global_table_provisioned_write_capacity_auto_scaling_settings_update) = input.global_table_provisioned_write_capacity_auto_scaling_settings_update {
        body.insert("GlobalTableProvisionedWriteCapacityAutoScalingSettingsUpdate", serde_json::to_string(&global_table_provisioned_write_capacity_auto_scaling_settings_update).unwrap());
    };
    if let Some(global_table_global_secondary_index_settings_update) = input.global_table_global_secondary_index_settings_update {
        body.insert("GlobalTableGlobalSecondaryIndexSettingsUpdate", serde_json::to_string(&global_table_global_secondary_index_settings_update).unwrap());
    };
    if let Some(replica_settings_update) = input.replica_settings_update {
        body.insert("ReplicaSettingsUpdate", serde_json::to_string(&replica_settings_update).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: UpdateGlobalTableSettingsOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    body.insert("Key", serde_json::to_string(&input.key).unwrap());
    if let Some(attribute_updates) = input.attribute_updates {
        body.insert("AttributeUpdates", serde_json::to_string(&attribute_updates).unwrap());
    };
    if let Some(expected) = input.expected {
        body.insert("Expected", serde_json::to_string(&expected).unwrap());
    };
    if let Some(conditional_operator) = input.conditional_operator {
        body.insert("ConditionalOperator", serde_json::to_string(&conditional_operator).unwrap());
    };
    if let Some(return_values) = input.return_values {
        body.insert("ReturnValues", serde_json::to_string(&return_values).unwrap());
    };
    if let Some(return_consumed_capacity) = input.return_consumed_capacity {
        body.insert("ReturnConsumedCapacity", serde_json::to_string(&return_consumed_capacity).unwrap());
    };
    if let Some(return_item_collection_metrics) = input.return_item_collection_metrics {
        body.insert("ReturnItemCollectionMetrics", serde_json::to_string(&return_item_collection_metrics).unwrap());
    };
    if let Some(update_expression) = input.update_expression {
        body.insert("UpdateExpression", serde_json::to_string(&update_expression).unwrap());
    };
    if let Some(condition_expression) = input.condition_expression {
        body.insert("ConditionExpression", serde_json::to_string(&condition_expression).unwrap());
    };
    if let Some(expression_attribute_names) = input.expression_attribute_names {
        body.insert("ExpressionAttributeNames", serde_json::to_string(&expression_attribute_names).unwrap());
    };
    if let Some(expression_attribute_values) = input.expression_attribute_values {
        body.insert("ExpressionAttributeValues", serde_json::to_string(&expression_attribute_values).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: UpdateItemOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(attribute_definitions) = input.attribute_definitions {
        body.insert("AttributeDefinitions", serde_json::to_string(&attribute_definitions).unwrap());
    };
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if let Some(billing_mode) = input.billing_mode {
        body.insert("BillingMode", serde_json::to_string(&billing_mode).unwrap());
    };
    if let Some(provisioned_throughput) = input.provisioned_throughput {
        body.insert("ProvisionedThroughput", serde_json::to_string(&provisioned_throughput).unwrap());
    };
    if let Some(global_secondary_index_updates) = input.global_secondary_index_updates {
        body.insert("GlobalSecondaryIndexUpdates", serde_json::to_string(&global_secondary_index_updates).unwrap());
    };
    if let Some(stream_specification) = input.stream_specification {
        body.insert("StreamSpecification", serde_json::to_string(&stream_specification).unwrap());
    };
    if let Some(sse_specification) = input.sse_specification {
        body.insert("SSESpecification", serde_json::to_string(&sse_specification).unwrap());
    };
    if let Some(replica_updates) = input.replica_updates {
        body.insert("ReplicaUpdates", serde_json::to_string(&replica_updates).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: UpdateTableOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(global_secondary_index_updates) = input.global_secondary_index_updates {
        body.insert("GlobalSecondaryIndexUpdates", serde_json::to_string(&global_secondary_index_updates).unwrap());
    };
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    if let Some(provisioned_write_capacity_auto_scaling_update) = input.provisioned_write_capacity_auto_scaling_update {
        body.insert("ProvisionedWriteCapacityAutoScalingUpdate", serde_json::to_string(&provisioned_write_capacity_auto_scaling_update).unwrap());
    };
    if let Some(replica_updates) = input.replica_updates {
        body.insert("ReplicaUpdates", serde_json::to_string(&replica_updates).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: UpdateTableReplicaAutoScalingOutput = serde_json::from_slice(body).unwrap();
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

    let mut http_request = SignedRequest::new(
        "POST",
        "dynamodb",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("TableName", serde_json::to_string(&input.table_name).unwrap());
    body.insert("TimeToLiveSpecification", serde_json::to_string(&input.time_to_live_specification).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_json::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let output: UpdateTimeToLiveOutput = serde_json::from_slice(body).unwrap();
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

