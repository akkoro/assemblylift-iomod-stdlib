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
        describe_global_table => describe_global_table,
        describe_global_table_settings => describe_global_table_settings,
        describe_limits => describe_limits,
        describe_table => describe_table,
        describe_table_replica_auto_scaling => describe_table_replica_auto_scaling,
        describe_time_to_live => describe_time_to_live,
        get_item => get_item,
        list_backups => list_backups,
        list_contributor_insights => list_contributor_insights,
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
        update_time_to_live => update_time_to_live
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

macro_rules! dynamodb_noinput {
    ($call:ident, $output:ty) => {
        fn $call(_input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
            Box::pin(async move {
                use rusoto_dynamodb::*;
                match DYNAMODB.$call().await {
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

dynamodb!(batch_get_item, BatchGetItemOutput);
dynamodb!(batch_write_item, BatchWriteItemOutput);
dynamodb!(create_backup, CreateBackupOutput);
dynamodb!(create_global_table, CreateGlobalTableOutput);
dynamodb!(create_table, CreateTableOutput);
dynamodb!(delete_backup, DeleteBackupOutput);
dynamodb!(delete_item, DeleteItemOutput);
dynamodb!(delete_table, DeleteTableOutput);
dynamodb!(describe_backup, DescribeBackupOutput);
dynamodb!(describe_continuous_backups, DescribeContinuousBackupsOutput);
dynamodb!(describe_contributor_insights, DescribeContributorInsightsOutput);
dynamodb_noinput!(describe_endpoints, DescribeEndpointsResponse);
dynamodb!(describe_global_table, DescribeGlobalTableOutput);
dynamodb!(describe_global_table_settings, DescribeGlobalTableSettingsOutput);
dynamodb_noinput!(describe_limits, DescribeLimitsOutput);
dynamodb!(describe_table, DescribeTableOutput);
dynamodb!(describe_table_replica_auto_scaling, DescribeTableReplicaAutoScalingOutput);
dynamodb!(describe_time_to_live, DescribeTimeToLiveOutput);
dynamodb!(get_item, GetItemOutput);
dynamodb!(list_backups, ListBackupsOutput);
dynamodb!(list_contributor_insights, ListContributorInsightsOutput);
dynamodb!(list_global_tables, ListGlobalTablesOutput);
dynamodb!(list_tables, ListTablesOutput);
dynamodb!(list_tags_of_resource, ListTagsOfResourceOutput);
dynamodb!(put_item, PutItemOutput);
dynamodb!(query, QueryOutput);
dynamodb!(restore_table_from_backup, RestoreTableFromBackupOutput);
dynamodb!(restore_table_to_point_in_time, RestoreTableToPointInTimeOutput);
dynamodb!(scan, ScanOutput);
dynamodb!(tag_resource, ());
dynamodb!(transact_get_items, TransactGetItemsOutput);
dynamodb!(transact_write_items, TransactWriteItemsOutput);
dynamodb!(untag_resource, ());
dynamodb!(update_continuous_backups, UpdateContinuousBackupsOutput);
dynamodb!(update_contributor_insights, UpdateContributorInsightsOutput);
dynamodb!(update_global_table, UpdateGlobalTableOutput);
dynamodb!(update_global_table_settings, UpdateGlobalTableSettingsOutput);
dynamodb!(update_item, UpdateItemOutput);
dynamodb!(update_table, UpdateTableOutput);
dynamodb!(update_table_replica_auto_scaling, UpdateTableReplicaAutoScalingOutput);
dynamodb!(update_time_to_live, UpdateTimeToLiveOutput);
