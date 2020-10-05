use std::collections::HashMap;

use assemblylift_core_iomod::iomod;
use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use paste::paste;
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use tokio;

static DYNAMODB: Lazy<DynamoDbClient> = Lazy::new(|| DynamoDbClient::new(Region::UsEast1));

#[tokio::main]
async fn main() {
    iomod!(akkoro.aws.dynamodb => {
        list_tables => aws_dynamodb_list_tables,
        put_item => aws_dynamodb_put_item,
        get_item => aws_dynamodb_get_item,
        delete_item => aws_dynamodb_delete_item,
        update_item => aws_dynamodb_update_item
    });
}

macro_rules! dynamodb {
    ($call:ident) => {
        paste! {
            fn [<aws_dynamodb_ $call>] (input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
                Box::pin(async move {
                    use rusoto_dynamodb::*;
                    let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
                    let result = DYNAMODB.$call(deserialized).await.unwrap();
                    serde_json::to_vec(&result).unwrap()
                })
            }
        }
    };
}

dynamodb!(list_tables);
dynamodb!(put_item);
dynamodb!(get_item);
dynamodb!(delete_item);
dynamodb!(update_item);
