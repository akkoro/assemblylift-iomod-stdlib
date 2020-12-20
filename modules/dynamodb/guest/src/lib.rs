use std::fmt;

use assemblylift_core_iomod_guest::{call, export_iomod_guest};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};

use crate::structs::*;

export_iomod_guest!(akkoro, aws, dynamodb);

mod serialization;
pub mod structs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub why: String,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}
impl std::error::Error for Error {}

call!(batch_get_item, BatchGetItemInput => Result<BatchGetItemOutput, Error>);
call!(batch_write_item, BatchWriteItemInput => Result<BatchWriteItemOutput, Error>);
call!(create_backup, CreateBackupInput => Result<CreateBackupOutput, Error>);
call!(create_global_table, CreateGlobalTableInput => Result<CreateGlobalTableOutput, Error>);
call!(create_table, CreateTableInput => Result<CreateTableOutput, Error>);
call!(delete_backup, DeleteBackupInput => Result<DeleteBackupOutput, Error>);
call!(delete_item, DeleteItemInput => Result<DeleteItemOutput, Error>);
call!(delete_table, DeleteTableInput => Result<DeleteTableOutput, Error>);
call!(describe_backup, DescribeBackupInput => Result<DescribeBackupOutput, Error>);
call!(describe_continuous_backups, DescribeContinuousBackupsInput => Result<DescribeContinuousBackupsOutput, Error>);
call!(describe_contributor_insights, DescribeContributorInsightsInput => Result<DescribeContributorInsightsOutput, Error>);
call!(describe_endpoints, DescribeEndpointsRequest => Result<DescribeEndpointsResponse, Error>);
call!(describe_global_table, DescribeGlobalTableInput => Result<DescribeGlobalTableOutput, Error>);
call!(describe_global_table_settings, DescribeGlobalTableSettingsInput => Result<DescribeGlobalTableSettingsOutput, Error>);
call!(describe_limits, DescribeLimitsInput => Result<DescribeLimitsOutput, Error>);
call!(describe_table, DescribeTableInput => Result<DescribeTableOutput, Error>);
call!(describe_table_replica_auto_scaling, DescribeTableReplicaAutoScalingInput => Result<DescribeTableReplicaAutoScalingOutput, Error>);
call!(describe_time_to_live, DescribeTimeToLiveInput => Result<DescribeTimeToLiveOutput, Error>);
call!(get_item, GetItemInput => Result<GetItemOutput, Error>);
call!(list_backups, ListBackupsInput => Result<ListBackupsOutput, Error>);
call!(list_contributor_insights, ListContributorInsightsInput => Result<ListContributorInsightsOutput, Error>);
call!(list_global_tables, ListGlobalTablesInput => Result<ListGlobalTablesOutput, Error>);
call!(list_tables, ListTablesInput => Result<ListTablesOutput, Error>);
call!(list_tags_of_resource, ListTagsOfResourceInput => Result<ListTagsOfResourceOutput, Error>);
call!(put_item, PutItemInput => Result<PutItemOutput, Error>);
call!(query, QueryInput => Result<QueryOutput, Error>);
call!(restore_table_from_backup, RestoreTableFromBackupInput => Result<RestoreTableFromBackupOutput, Error>);
call!(restore_table_to_point_in_time, RestoreTableToPointInTimeInput => Result<RestoreTableToPointInTimeOutput, Error>);
call!(scan, ScanInput => Result<ScanOutput, Error>);
call!(tag_resource, TagResourceInput => Result<(), Error>);
call!(transact_get_items, TransactGetItemsInput => Result<TransactGetItemsOutput, Error>);
call!(transact_write_items, TransactWriteItemsInput => Result<TransactWriteItemsOutput, Error>);
call!(untag_resource, UntagResourceInput => Result<(), Error>);
call!(update_continuous_backups, UpdateContinuousBackupsInput => Result<UpdateContinuousBackupsOutput, Error>);
call!(update_contributor_insights, UpdateContributorInsightsInput => Result<UpdateContributorInsightsOutput, Error>);
call!(update_global_table, UpdateGlobalTableInput => Result<UpdateGlobalTableOutput, Error>);
call!(update_global_table_settings, UpdateGlobalTableSettingsInput => Result<UpdateGlobalTableSettingsOutput, Error>);
call!(update_item, UpdateItemInput => Result<UpdateItemOutput, Error>);
call!(update_table, UpdateTableInput => Result<UpdateTableOutput, Error>);
call!(update_table_replica_auto_scaling, UpdateTableReplicaAutoScalingInput => Result<UpdateTableReplicaAutoScalingOutput, Error>);
call!(update_time_to_live, UpdateTimeToLiveInput => Result<UpdateTimeToLiveOutput, Error>);

#[macro_export]
macro_rules! val {
    (B => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.b = Some($val);
        attr
    }};
    (S => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.s = Some($val.to_string());
        attr
    }};
    (N => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.n = Some($val.to_string());
        attr
    }};
}
