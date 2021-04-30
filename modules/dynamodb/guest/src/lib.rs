use std::fmt;
use std::fmt::Formatter;

use assemblylift_core_iomod_guest::{call, export_iomod_guest};
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

call!(batch_execute_statement, BatchExecuteStatementInput => Result<BatchExecuteStatementOutput, Error>);
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
call!(describe_export, DescribeExportInput => Result<DescribeExportOutput, Error>);
call!(describe_global_table, DescribeGlobalTableInput => Result<DescribeGlobalTableOutput, Error>);
call!(describe_global_table_settings, DescribeGlobalTableSettingsInput => Result<DescribeGlobalTableSettingsOutput, Error>);
call!(describe_kinesis_streaming_destination, DescribeKinesisStreamingDestinationInput => Result<DescribeKinesisStreamingDestinationOutput, Error>);
call!(describe_limits, DescribeLimitsInput => Result<DescribeLimitsOutput, Error>);
call!(describe_table, DescribeTableInput => Result<DescribeTableOutput, Error>);
call!(describe_table_replica_auto_scaling, DescribeTableReplicaAutoScalingInput => Result<DescribeTableReplicaAutoScalingOutput, Error>);
call!(describe_time_to_live, DescribeTimeToLiveInput => Result<DescribeTimeToLiveOutput, Error>);
call!(disable_kinesis_streaming_destination, KinesisStreamingDestinationInput => Result<KinesisStreamingDestinationOutput, Error>);
call!(enable_kinesis_streaming_destination, KinesisStreamingDestinationInput => Result<KinesisStreamingDestinationOutput, Error>);
call!(execute_statement, ExecuteStatementInput => Result<ExecuteStatementOutput, Error>);
call!(execute_transaction, ExecuteTransactionInput => Result<ExecuteTransactionOutput, Error>);
call!(export_table_to_point_in_time, ExportTableToPointInTimeInput => Result<ExportTableToPointInTimeOutput, Error>);
call!(get_item, GetItemInput => Result<GetItemOutput, Error>);
call!(list_backups, ListBackupsInput => Result<ListBackupsOutput, Error>);
call!(list_contributor_insights, ListContributorInsightsInput => Result<ListContributorInsightsOutput, Error>);
call!(list_exports, ListExportsInput => Result<ListExportsOutput, Error>);
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
