use assemblylift_core_iomod::iomod;
use assemblylift_core_iomod::iomod_capnp::*;
use capnp::capability::Promise;
use capnp::{Error, ErrorKind};
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::future::BoxFuture;
use futures::{AsyncReadExt, FutureExt};
use futures_util::TryFutureExt;
use once_cell::sync::Lazy;
use paste::paste;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDbClient, *};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use guest;

static DYNAMODB: Lazy<DynamoDbClient> = Lazy::new(|| {
    DynamoDbClient::new(
        std::env::var("AWS_REGION")
            .unwrap_or(String::from("us-east-1"))
            .parse()
            .unwrap(),
    )
});

#[tokio::main]
async fn main() {
    iomod!(akkoro.aws.dynamodb => {
        list_tables => list_tables,
        put_item => put_item,
        get_item => get_item,
        delete_item => delete_item,
        update_item => update_item
    });
}

macro_rules! dynamodb {
    ($call:ident, $output:ty) => {
        fn $call(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
            Box::pin(async move {
                use rusoto_dynamodb::*;
                let deserialized = serde_json::from_slice(input.as_slice()).unwrap();
                match DYNAMODB.$call(deserialized).await {
                    Ok(result) => {
                        serde_json::to_vec(&Result::<$output, guest::Error>::Ok(result)).unwrap()
                    }
                    Err(err) => {
                        serde_json::to_vec(&Result::<$output, guest::Error>::Err(guest::Error {
                            why: err.to_string(),
                        }))
                        .unwrap()
                    }
                }
            })
        }
    };
}

dynamodb!(list_tables, ListTablesOutput);
dynamodb!(put_item,    PutItemOutput);
dynamodb!(get_item,    GetItemOutput);
dynamodb!(delete_item, DeleteItemOutput);
dynamodb!(update_item, UpdateItemOutput);
