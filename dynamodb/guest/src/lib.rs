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

call!(batch_get_item, BatchGetItemInput => Result<BatchGetItemInput, Error>);
call!(batch_write_item, BatchWriteItemInput => Result<BatchWriteItemInput, Error>);
call!(create_backup, CreateBackupInput => Result<CreateBackupInput, Error>);
call!(create_global_table, CreateGlobalTableInput => Result<CreateGlobalTableInput, Error>);
call!(create_table, CreateTableInput => Result<CreateTableInput, Error>);
call!(delete_backup, DeleteBackupInput => Result<DeleteBackupInput, Error>);
call!(delete_item, DeleteItemInput => Result<DeleteItemInput, Error>);
call!(delete_table, DeleteTableInput => Result<DeleteTableInput, Error>);
call!(describe_backup, DescribeBackupInput => Result<DescribeBackupInput, Error>);
call!(describe_continuous_backups, DescribeContinuousBackupsInput => Result<DescribeContinuousBackupsInput, Error>);
call!(describe_contributor_insights, DescribeContributorInsightsInput => Result<DescribeContributorInsightsInput, Error>);
call!(describe_endpoints, DescribeEndpointsRequest => Result<DescribeEndpointsRequest, Error>);
call!(describe_global_table, DescribeGlobalTableInput => Result<DescribeGlobalTableInput, Error>);
call!(describe_global_table_settings, DescribeGlobalTableSettingsInput => Result<DescribeGlobalTableSettingsInput, Error>);
call!(describe_limits, DescribeLimitsInput => Result<DescribeLimitsInput, Error>);
call!(describe_table, DescribeTableInput => Result<DescribeTableInput, Error>);
call!(describe_table_replica_auto_scaling, DescribeTableReplicaAutoScalingInput => Result<DescribeTableReplicaAutoScalingInput, Error>);
call!(describe_time_to_live, DescribeTimeToLiveInput => Result<DescribeTimeToLiveInput, Error>);
call!(get_item, GetItemInput => Result<GetItemInput, Error>);
call!(list_backups, ListBackupsInput => Result<ListBackupsInput, Error>);
call!(list_contributor_insights, ListContributorInsightsInput => Result<ListContributorInsightsInput, Error>);
call!(list_global_tables, ListGlobalTablesInput => Result<ListGlobalTablesInput, Error>);
call!(list_tables, ListTablesInput => Result<ListTablesInput, Error>);
call!(list_tags_of_resource, ListTagsOfResourceInput => Result<ListTagsOfResourceInput, Error>);
call!(put_item, PutItemInput => Result<PutItemInput, Error>);
call!(query, QueryInput => Result<QueryInput, Error>);
call!(restore_table_from_backup, RestoreTableFromBackupInput => Result<RestoreTableFromBackupInput, Error>);
call!(restore_table_to_point_in_time, RestoreTableToPointInTimeInput => Result<RestoreTableToPointInTimeInput, Error>);
call!(scan, ScanInput => Result<ScanInput, Error>);
call!(tag_resource, TagResourceInput => Result<TagResourceInput, Error>);
call!(transact_get_items, TransactGetItemsInput => Result<TransactGetItemsInput, Error>);
call!(transact_write_items, TransactWriteItemsInput => Result<TransactWriteItemsInput, Error>);
call!(untag_resource, UntagResourceInput => Result<UntagResourceInput, Error>);
call!(update_continuous_backups, UpdateContinuousBackupsInput => Result<UpdateContinuousBackupsInput, Error>);
call!(update_contributor_insights, UpdateContributorInsightsInput => Result<UpdateContributorInsightsInput, Error>);
call!(update_global_table, UpdateGlobalTableInput => Result<UpdateGlobalTableInput, Error>);
call!(update_global_table_settings, UpdateGlobalTableSettingsInput => Result<UpdateGlobalTableSettingsInput, Error>);
call!(update_item, UpdateItemInput => Result<UpdateItemInput, Error>);
call!(update_table, UpdateTableInput => Result<UpdateTableInput, Error>);
call!(update_table_replica_auto_scaling, UpdateTableReplicaAutoScalingInput => Result<UpdateTableReplicaAutoScalingInput, Error>);
call!(update_time_to_live, UpdateTimeToLiveInput => Result<UpdateTimeToLiveInput, Error>);

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
