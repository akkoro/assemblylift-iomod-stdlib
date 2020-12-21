// This file is generated!
// See https://github.com/akkoro/asml-aws-codegen

use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub type ArchivalReason = String;
pub type AttributeAction = String;
pub type AttributeDefinitions = Vec<AttributeDefinition>;
pub type AttributeMap = HashMap<AttributeName, AttributeValue>;
pub type AttributeName = String;
pub type AttributeNameList = Vec<AttributeName>;
pub type AttributeUpdates = HashMap<AttributeName, AttributeValueUpdate>;
pub type AttributeValueList = Vec<AttributeValue>;
pub type AutoScalingPolicyDescriptionList = Vec<AutoScalingPolicyDescription>;
pub type AutoScalingPolicyName = String;
pub type AutoScalingRoleArn = String;
pub type Backfilling = bool;
pub type BackupArn = String;
pub type BackupCreationDateTime = String;
pub type BackupName = String;
pub type BackupSizeBytes = u64;
pub type BackupStatus = String;
pub type BackupSummaries = Vec<BackupSummary>;
pub type BackupType = String;
pub type BackupTypeFilter = String;
pub type BackupsInputLimit = i64;
pub type BatchGetRequestMap = HashMap<TableName, KeysAndAttributes>;
pub type BatchGetResponseMap = HashMap<TableName, ItemList>;
pub type BatchStatementErrorCodeEnum = String;
pub type BatchWriteItemRequestMap = HashMap<TableName, WriteRequests>;
pub type BilledSizeBytes = u64;
pub type BillingMode = String;
pub type BinaryAttributeValue = Vec<u8>;
pub type BinarySetAttributeValue = Vec<BinaryAttributeValue>;
pub type BooleanAttributeValue = bool;
pub type BooleanObject = bool;
pub type CancellationReasonList = Vec<CancellationReason>;
pub type ClientRequestToken = String;
pub type ClientToken = String;
pub type Code = String;
pub type ComparisonOperator = String;
pub type ConditionExpression = String;
pub type ConditionalOperator = String;
pub type ConsistentRead = bool;
pub type ConsumedCapacityMultiple = Vec<ConsumedCapacity>;
pub type ConsumedCapacityUnits = f64;
pub type ContinuousBackupsStatus = String;
pub type ContributorInsightsAction = String;
pub type ContributorInsightsRule = String;
pub type ContributorInsightsRuleList = Vec<ContributorInsightsRule>;
pub type ContributorInsightsStatus = String;
pub type ContributorInsightsSummaries = Vec<ContributorInsightsSummary>;
pub type Date = String;
pub type DestinationStatus = String;
pub type Double = f64;
pub type Endpoints = Vec<Endpoint>;
pub type ErrorMessage = String;
pub type ExceptionDescription = String;
pub type ExceptionName = String;
pub type ExpectedAttributeMap = HashMap<AttributeName, ExpectedAttributeValue>;
pub type ExportArn = String;
pub type ExportEndTime = String;
pub type ExportFormat = String;
pub type ExportManifest = String;
pub type ExportNextToken = String;
pub type ExportStartTime = String;
pub type ExportStatus = String;
pub type ExportSummaries = Vec<ExportSummary>;
pub type ExportTime = String;
pub type ExpressionAttributeNameMap = HashMap<ExpressionAttributeNameVariable, AttributeName>;
pub type ExpressionAttributeNameVariable = String;
pub type ExpressionAttributeValueMap = HashMap<ExpressionAttributeValueVariable, AttributeValue>;
pub type ExpressionAttributeValueVariable = String;
pub type FailureCode = String;
pub type FailureMessage = String;
pub type FilterConditionMap = HashMap<AttributeName, Condition>;
pub type GlobalSecondaryIndexAutoScalingUpdateList = Vec<GlobalSecondaryIndexAutoScalingUpdate>;
pub type GlobalSecondaryIndexDescriptionList = Vec<GlobalSecondaryIndexDescription>;
pub type GlobalSecondaryIndexList = Vec<GlobalSecondaryIndex>;
pub type GlobalSecondaryIndexUpdateList = Vec<GlobalSecondaryIndexUpdate>;
pub type GlobalSecondaryIndexes = Vec<GlobalSecondaryIndexInfo>;
pub type GlobalTableArnString = String;
pub type GlobalTableGlobalSecondaryIndexSettingsUpdateList = Vec<GlobalTableGlobalSecondaryIndexSettingsUpdate>;
pub type GlobalTableList = Vec<GlobalTable>;
pub type GlobalTableStatus = String;
pub type IndexName = String;
pub type IndexStatus = String;
pub type Integer = i64;
pub type IntegerObject = i64;
pub type ItemCollectionKeyAttributeMap = HashMap<AttributeName, AttributeValue>;
pub type ItemCollectionMetricsMultiple = Vec<ItemCollectionMetrics>;
pub type ItemCollectionMetricsPerTable = HashMap<TableName, ItemCollectionMetricsMultiple>;
pub type ItemCollectionSizeEstimateBound = f64;
pub type ItemCollectionSizeEstimateRange = Vec<ItemCollectionSizeEstimateBound>;
pub type ItemCount = u64;
pub type ItemList = Vec<AttributeMap>;
pub type ItemResponseList = Vec<ItemResponse>;
pub type KMSMasterKeyArn = String;
pub type KMSMasterKeyId = String;
pub type Key = HashMap<AttributeName, AttributeValue>;
pub type KeyConditions = HashMap<AttributeName, Condition>;
pub type KeyExpression = String;
pub type KeyList = Vec<Key>;
pub type KeySchema = Vec<KeySchemaElement>;
pub type KeySchemaAttributeName = String;
pub type KeyType = String;
pub type KinesisDataStreamDestinations = Vec<KinesisDataStreamDestination>;
pub type LastUpdateDateTime = String;
pub type ListAttributeValue = Vec<AttributeValue>;
pub type ListContributorInsightsLimit = i64;
pub type ListExportsMaxLimit = i64;
pub type ListTablesInputLimit = i64;
pub type LocalSecondaryIndexDescriptionList = Vec<LocalSecondaryIndexDescription>;
pub type LocalSecondaryIndexList = Vec<LocalSecondaryIndex>;
pub type LocalSecondaryIndexes = Vec<LocalSecondaryIndexInfo>;
pub type Long = u64;
pub type MapAttributeValue = HashMap<AttributeName, AttributeValue>;
pub type NextTokenString = String;
pub type NonKeyAttributeName = String;
pub type NonKeyAttributeNameList = Vec<NonKeyAttributeName>;
pub type NonNegativeLongObject = u64;
pub type NullAttributeValue = bool;
pub type NumberAttributeValue = String;
pub type NumberSetAttributeValue = Vec<NumberAttributeValue>;
pub type ParameterizedStatements = Vec<ParameterizedStatement>;
pub type PartiQLBatchRequest = Vec<BatchStatementRequest>;
pub type PartiQLBatchResponse = Vec<BatchStatementResponse>;
pub type PartiQLNextToken = String;
pub type PartiQLStatement = String;
pub type PointInTimeRecoveryStatus = String;
pub type PositiveIntegerObject = i64;
pub type PositiveLongObject = u64;
pub type PreparedStatementParameters = Vec<AttributeValue>;
pub type ProjectionExpression = String;
pub type ProjectionType = String;
pub type PutItemInputAttributeMap = HashMap<AttributeName, AttributeValue>;
pub type RegionName = String;
pub type ReplicaAutoScalingDescriptionList = Vec<ReplicaAutoScalingDescription>;
pub type ReplicaAutoScalingUpdateList = Vec<ReplicaAutoScalingUpdate>;
pub type ReplicaDescriptionList = Vec<ReplicaDescription>;
pub type ReplicaGlobalSecondaryIndexAutoScalingDescriptionList = Vec<ReplicaGlobalSecondaryIndexAutoScalingDescription>;
pub type ReplicaGlobalSecondaryIndexAutoScalingUpdateList = Vec<ReplicaGlobalSecondaryIndexAutoScalingUpdate>;
pub type ReplicaGlobalSecondaryIndexDescriptionList = Vec<ReplicaGlobalSecondaryIndexDescription>;
pub type ReplicaGlobalSecondaryIndexList = Vec<ReplicaGlobalSecondaryIndex>;
pub type ReplicaGlobalSecondaryIndexSettingsDescriptionList = Vec<ReplicaGlobalSecondaryIndexSettingsDescription>;
pub type ReplicaGlobalSecondaryIndexSettingsUpdateList = Vec<ReplicaGlobalSecondaryIndexSettingsUpdate>;
pub type ReplicaList = Vec<Replica>;
pub type ReplicaSettingsDescriptionList = Vec<ReplicaSettingsDescription>;
pub type ReplicaSettingsUpdateList = Vec<ReplicaSettingsUpdate>;
pub type ReplicaStatus = String;
pub type ReplicaStatusDescription = String;
pub type ReplicaStatusPercentProgress = String;
pub type ReplicaUpdateList = Vec<ReplicaUpdate>;
pub type ReplicationGroupUpdateList = Vec<ReplicationGroupUpdate>;
pub type ResourceArnString = String;
pub type RestoreInProgress = bool;
pub type ReturnConsumedCapacity = String;
pub type ReturnItemCollectionMetrics = String;
pub type ReturnValue = String;
pub type ReturnValuesOnConditionCheckFailure = String;
pub type S3Bucket = String;
pub type S3BucketOwner = String;
pub type S3Prefix = String;
pub type S3SseAlgorithm = String;
pub type S3SseKmsKeyId = String;
pub type SSEEnabled = bool;
pub type SSEStatus = String;
pub type SSEType = String;
pub type ScalarAttributeType = String;
pub type ScanSegment = i64;
pub type ScanTotalSegments = i64;
pub type SecondaryIndexesCapacityMap = HashMap<IndexName, Capacity>;
pub type Select = String;
pub type StreamArn = String;
pub type StreamEnabled = bool;
pub type StreamViewType = String;
pub type StringAttributeValue = String;
pub type StringSetAttributeValue = Vec<StringAttributeValue>;
pub type TableArn = String;
pub type TableCreationDateTime = String;
pub type TableId = String;
pub type TableName = String;
pub type TableNameList = Vec<TableName>;
pub type TableStatus = String;
pub type TagKeyList = Vec<TagKeyString>;
pub type TagKeyString = String;
pub type TagList = Vec<Tag>;
pub type TagValueString = String;
pub type TimeRangeLowerBound = String;
pub type TimeRangeUpperBound = String;
pub type TimeToLiveAttributeName = String;
pub type TimeToLiveEnabled = bool;
pub type TimeToLiveStatus = String;
pub type TransactGetItemList = Vec<TransactGetItem>;
pub type TransactWriteItemList = Vec<TransactWriteItem>;
pub type UpdateExpression = String;
pub type WriteRequests = Vec<WriteRequest>;
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ArchivalSummary {
    #[serde(rename = "ArchivalDateTime")]
    pub archival_date_time: Option<Date>,
    #[serde(rename = "ArchivalReason")]
    pub archival_reason: Option<ArchivalReason>,
    #[serde(rename = "ArchivalBackupArn")]
    pub archival_backup_arn: Option<BackupArn>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AttributeDefinition {
    #[serde(rename = "AttributeName")]
    pub attribute_name: KeySchemaAttributeName,
    #[serde(rename = "AttributeType")]
    pub attribute_type: ScalarAttributeType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AttributeValue {
    #[serde(rename = "S")]
    pub s: Option<StringAttributeValue>,
    #[serde(rename = "N")]
    pub n: Option<NumberAttributeValue>,
    #[serde(rename = "B")]
    pub b: Option<BinaryAttributeValue>,
    #[serde(rename = "SS")]
    pub ss: Option<StringSetAttributeValue>,
    #[serde(rename = "NS")]
    pub ns: Option<NumberSetAttributeValue>,
    #[serde(rename = "BS")]
    pub bs: Option<BinarySetAttributeValue>,
    #[serde(rename = "M")]
    pub m: Option<MapAttributeValue>,
    #[serde(rename = "L")]
    pub l: Option<ListAttributeValue>,
    #[serde(rename = "NULL")]
    pub null: Option<NullAttributeValue>,
    #[serde(rename = "BOOL")]
    pub bool: Option<BooleanAttributeValue>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AttributeValueUpdate {
    #[serde(rename = "Value")]
    pub value: Option<AttributeValue>,
    #[serde(rename = "Action")]
    pub action: Option<AttributeAction>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingPolicyDescription {
    #[serde(rename = "PolicyName")]
    pub policy_name: Option<AutoScalingPolicyName>,
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    pub target_tracking_scaling_policy_configuration: Option<AutoScalingTargetTrackingScalingPolicyConfigurationDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingPolicyUpdate {
    #[serde(rename = "PolicyName")]
    pub policy_name: Option<AutoScalingPolicyName>,
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    pub target_tracking_scaling_policy_configuration: AutoScalingTargetTrackingScalingPolicyConfigurationUpdate,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingSettingsDescription {
    #[serde(rename = "MinimumUnits")]
    pub minimum_units: Option<PositiveLongObject>,
    #[serde(rename = "MaximumUnits")]
    pub maximum_units: Option<PositiveLongObject>,
    #[serde(rename = "AutoScalingDisabled")]
    pub auto_scaling_disabled: Option<BooleanObject>,
    #[serde(rename = "AutoScalingRoleArn")]
    pub auto_scaling_role_arn: Option<String>,
    #[serde(rename = "ScalingPolicies")]
    pub scaling_policies: Option<AutoScalingPolicyDescriptionList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingSettingsUpdate {
    #[serde(rename = "MinimumUnits")]
    pub minimum_units: Option<PositiveLongObject>,
    #[serde(rename = "MaximumUnits")]
    pub maximum_units: Option<PositiveLongObject>,
    #[serde(rename = "AutoScalingDisabled")]
    pub auto_scaling_disabled: Option<BooleanObject>,
    #[serde(rename = "AutoScalingRoleArn")]
    pub auto_scaling_role_arn: Option<AutoScalingRoleArn>,
    #[serde(rename = "ScalingPolicyUpdate")]
    pub scaling_policy_update: Option<AutoScalingPolicyUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingTargetTrackingScalingPolicyConfigurationDescription {
    #[serde(rename = "DisableScaleIn")]
    pub disable_scale_in: Option<BooleanObject>,
    #[serde(rename = "ScaleInCooldown")]
    pub scale_in_cooldown: Option<IntegerObject>,
    #[serde(rename = "ScaleOutCooldown")]
    pub scale_out_cooldown: Option<IntegerObject>,
    #[serde(rename = "TargetValue")]
    pub target_value: Double,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingTargetTrackingScalingPolicyConfigurationUpdate {
    #[serde(rename = "DisableScaleIn")]
    pub disable_scale_in: Option<BooleanObject>,
    #[serde(rename = "ScaleInCooldown")]
    pub scale_in_cooldown: Option<IntegerObject>,
    #[serde(rename = "ScaleOutCooldown")]
    pub scale_out_cooldown: Option<IntegerObject>,
    #[serde(rename = "TargetValue")]
    pub target_value: Double,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackupDescription {
    #[serde(rename = "BackupDetails")]
    pub backup_details: Option<BackupDetails>,
    #[serde(rename = "SourceTableDetails")]
    pub source_table_details: Option<SourceTableDetails>,
    #[serde(rename = "SourceTableFeatureDetails")]
    pub source_table_feature_details: Option<SourceTableFeatureDetails>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackupDetails {
    #[serde(rename = "BackupArn")]
    pub backup_arn: BackupArn,
    #[serde(rename = "BackupName")]
    pub backup_name: BackupName,
    #[serde(rename = "BackupSizeBytes")]
    pub backup_size_bytes: Option<BackupSizeBytes>,
    #[serde(rename = "BackupStatus")]
    pub backup_status: BackupStatus,
    #[serde(rename = "BackupType")]
    pub backup_type: BackupType,
    #[serde(rename = "BackupCreationDateTime")]
    pub backup_creation_date_time: BackupCreationDateTime,
    #[serde(rename = "BackupExpiryDateTime")]
    pub backup_expiry_date_time: Option<Date>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackupInUseException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackupNotFoundException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackupSummary {
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "TableId")]
    pub table_id: Option<TableId>,
    #[serde(rename = "TableArn")]
    pub table_arn: Option<TableArn>,
    #[serde(rename = "BackupArn")]
    pub backup_arn: Option<BackupArn>,
    #[serde(rename = "BackupName")]
    pub backup_name: Option<BackupName>,
    #[serde(rename = "BackupCreationDateTime")]
    pub backup_creation_date_time: Option<BackupCreationDateTime>,
    #[serde(rename = "BackupExpiryDateTime")]
    pub backup_expiry_date_time: Option<Date>,
    #[serde(rename = "BackupStatus")]
    pub backup_status: Option<BackupStatus>,
    #[serde(rename = "BackupType")]
    pub backup_type: Option<BackupType>,
    #[serde(rename = "BackupSizeBytes")]
    pub backup_size_bytes: Option<BackupSizeBytes>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchExecuteStatementInput {
    #[serde(rename = "Statements")]
    pub statements: PartiQLBatchRequest,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchExecuteStatementOutput {
    #[serde(rename = "Responses")]
    pub responses: Option<PartiQLBatchResponse>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchGetItemInput {
    #[serde(rename = "RequestItems")]
    pub request_items: BatchGetRequestMap,
    #[serde(rename = "ReturnConsumedCapacity")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchGetItemOutput {
    #[serde(rename = "Responses")]
    pub responses: Option<BatchGetResponseMap>,
    #[serde(rename = "UnprocessedKeys")]
    pub unprocessed_keys: Option<BatchGetRequestMap>,
    #[serde(rename = "ConsumedCapacity")]
    pub consumed_capacity: Option<ConsumedCapacityMultiple>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchStatementError {
    #[serde(rename = "Code")]
    pub code: Option<BatchStatementErrorCodeEnum>,
    #[serde(rename = "Message")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchStatementRequest {
    #[serde(rename = "Statement")]
    pub statement: PartiQLStatement,
    #[serde(rename = "Parameters")]
    pub parameters: Option<PreparedStatementParameters>,
    #[serde(rename = "ConsistentRead")]
    pub consistent_read: Option<ConsistentRead>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchStatementResponse {
    #[serde(rename = "Error")]
    pub error: Option<BatchStatementError>,
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "Item")]
    pub item: Option<AttributeMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchWriteItemInput {
    #[serde(rename = "RequestItems")]
    pub request_items: BatchWriteItemRequestMap,
    #[serde(rename = "ReturnConsumedCapacity")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchWriteItemOutput {
    #[serde(rename = "UnprocessedItems")]
    pub unprocessed_items: Option<BatchWriteItemRequestMap>,
    #[serde(rename = "ItemCollectionMetrics")]
    pub item_collection_metrics: Option<ItemCollectionMetricsPerTable>,
    #[serde(rename = "ConsumedCapacity")]
    pub consumed_capacity: Option<ConsumedCapacityMultiple>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BillingModeSummary {
    #[serde(rename = "BillingMode")]
    pub billing_mode: Option<BillingMode>,
    #[serde(rename = "LastUpdateToPayPerRequestDateTime")]
    pub last_update_to_pay_per_request_date_time: Option<Date>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CancellationReason {
    #[serde(rename = "Item")]
    pub item: Option<AttributeMap>,
    #[serde(rename = "Code")]
    pub code: Option<Code>,
    #[serde(rename = "Message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Capacity {
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: Option<ConsumedCapacityUnits>,
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: Option<ConsumedCapacityUnits>,
    #[serde(rename = "CapacityUnits")]
    pub capacity_units: Option<ConsumedCapacityUnits>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "AttributeValueList")]
    pub attribute_value_list: Option<AttributeValueList>,
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: ComparisonOperator,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConditionCheck {
    #[serde(rename = "Key")]
    pub key: Key,
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "ConditionExpression")]
    pub condition_expression: ConditionExpression,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    pub return_values_on_condition_check_failure: Option<ReturnValuesOnConditionCheckFailure>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConditionalCheckFailedException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConsumedCapacity {
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "CapacityUnits")]
    pub capacity_units: Option<ConsumedCapacityUnits>,
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: Option<ConsumedCapacityUnits>,
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: Option<ConsumedCapacityUnits>,
    #[serde(rename = "Table")]
    pub table: Option<Capacity>,
    #[serde(rename = "LocalSecondaryIndexes")]
    pub local_secondary_indexes: Option<SecondaryIndexesCapacityMap>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<SecondaryIndexesCapacityMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContinuousBackupsDescription {
    #[serde(rename = "ContinuousBackupsStatus")]
    pub continuous_backups_status: ContinuousBackupsStatus,
    #[serde(rename = "PointInTimeRecoveryDescription")]
    pub point_in_time_recovery_description: Option<PointInTimeRecoveryDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContinuousBackupsUnavailableException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContributorInsightsSummary {
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ContributorInsightsStatus")]
    pub contributor_insights_status: Option<ContributorInsightsStatus>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateBackupInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "BackupName")]
    pub backup_name: BackupName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateBackupOutput {
    #[serde(rename = "BackupDetails")]
    pub backup_details: Option<BackupDetails>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateGlobalSecondaryIndexAction {
    #[serde(rename = "IndexName")]
    pub index_name: IndexName,
    #[serde(rename = "KeySchema")]
    pub key_schema: KeySchema,
    #[serde(rename = "Projection")]
    pub projection: Projection,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateGlobalTableInput {
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: TableName,
    #[serde(rename = "ReplicationGroup")]
    pub replication_group: ReplicaList,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateGlobalTableOutput {
    #[serde(rename = "GlobalTableDescription")]
    pub global_table_description: Option<GlobalTableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateReplicaAction {
    #[serde(rename = "RegionName")]
    pub region_name: RegionName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateReplicationGroupMemberAction {
    #[serde(rename = "RegionName")]
    pub region_name: RegionName,
    #[serde(rename = "KMSMasterKeyId")]
    pub kms_master_key_id: Option<KMSMasterKeyId>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<ReplicaGlobalSecondaryIndexList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateTableInput {
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: AttributeDefinitions,
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "KeySchema")]
    pub key_schema: KeySchema,
    #[serde(rename = "LocalSecondaryIndexes")]
    pub local_secondary_indexes: Option<LocalSecondaryIndexList>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<GlobalSecondaryIndexList>,
    #[serde(rename = "BillingMode")]
    pub billing_mode: Option<BillingMode>,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "StreamSpecification")]
    pub stream_specification: Option<StreamSpecification>,
    #[serde(rename = "SSESpecification")]
    pub sse_specification: Option<SSESpecification>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateTableOutput {
    #[serde(rename = "TableDescription")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Delete {
    #[serde(rename = "Key")]
    pub key: Key,
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "ConditionExpression")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    pub return_values_on_condition_check_failure: Option<ReturnValuesOnConditionCheckFailure>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBackupInput {
    #[serde(rename = "BackupArn")]
    pub backup_arn: BackupArn,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBackupOutput {
    #[serde(rename = "BackupDescription")]
    pub backup_description: Option<BackupDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteGlobalSecondaryIndexAction {
    #[serde(rename = "IndexName")]
    pub index_name: IndexName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteItemInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "Key")]
    pub key: Key,
    #[serde(rename = "Expected")]
    pub expected: Option<ExpectedAttributeMap>,
    #[serde(rename = "ConditionalOperator")]
    pub conditional_operator: Option<ConditionalOperator>,
    #[serde(rename = "ReturnValues")]
    pub return_values: Option<ReturnValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
    #[serde(rename = "ConditionExpression")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteItemOutput {
    #[serde(rename = "Attributes")]
    pub attributes: Option<AttributeMap>,
    #[serde(rename = "ConsumedCapacity")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "ItemCollectionMetrics")]
    pub item_collection_metrics: Option<ItemCollectionMetrics>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteReplicaAction {
    #[serde(rename = "RegionName")]
    pub region_name: RegionName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteReplicationGroupMemberAction {
    #[serde(rename = "RegionName")]
    pub region_name: RegionName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteRequest {
    #[serde(rename = "Key")]
    pub key: Key,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteTableInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteTableOutput {
    #[serde(rename = "TableDescription")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeBackupInput {
    #[serde(rename = "BackupArn")]
    pub backup_arn: BackupArn,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeBackupOutput {
    #[serde(rename = "BackupDescription")]
    pub backup_description: Option<BackupDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeContinuousBackupsInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeContinuousBackupsOutput {
    #[serde(rename = "ContinuousBackupsDescription")]
    pub continuous_backups_description: Option<ContinuousBackupsDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeContributorInsightsInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeContributorInsightsOutput {
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ContributorInsightsRuleList")]
    pub contributor_insights_rule_list: Option<ContributorInsightsRuleList>,
    #[serde(rename = "ContributorInsightsStatus")]
    pub contributor_insights_status: Option<ContributorInsightsStatus>,
    #[serde(rename = "LastUpdateDateTime")]
    pub last_update_date_time: Option<LastUpdateDateTime>,
    #[serde(rename = "FailureException")]
    pub failure_exception: Option<FailureException>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeEndpointsRequest {
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeEndpointsResponse {
    #[serde(rename = "Endpoints")]
    pub endpoints: Endpoints,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeExportInput {
    #[serde(rename = "ExportArn")]
    pub export_arn: ExportArn,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeExportOutput {
    #[serde(rename = "ExportDescription")]
    pub export_description: Option<ExportDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeGlobalTableInput {
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: TableName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeGlobalTableOutput {
    #[serde(rename = "GlobalTableDescription")]
    pub global_table_description: Option<GlobalTableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeGlobalTableSettingsInput {
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: TableName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeGlobalTableSettingsOutput {
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: Option<TableName>,
    #[serde(rename = "ReplicaSettings")]
    pub replica_settings: Option<ReplicaSettingsDescriptionList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeKinesisStreamingDestinationInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeKinesisStreamingDestinationOutput {
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "KinesisDataStreamDestinations")]
    pub kinesis_data_stream_destinations: Option<KinesisDataStreamDestinations>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeLimitsInput {
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeLimitsOutput {
    #[serde(rename = "AccountMaxReadCapacityUnits")]
    pub account_max_read_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "AccountMaxWriteCapacityUnits")]
    pub account_max_write_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "TableMaxReadCapacityUnits")]
    pub table_max_read_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "TableMaxWriteCapacityUnits")]
    pub table_max_write_capacity_units: Option<PositiveLongObject>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeTableInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeTableOutput {
    #[serde(rename = "Table")]
    pub table: Option<TableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeTableReplicaAutoScalingInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeTableReplicaAutoScalingOutput {
    #[serde(rename = "TableAutoScalingDescription")]
    pub table_auto_scaling_description: Option<TableAutoScalingDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeTimeToLiveInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeTimeToLiveOutput {
    #[serde(rename = "TimeToLiveDescription")]
    pub time_to_live_description: Option<TimeToLiveDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DuplicateItemException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "CachePeriodInMinutes")]
    pub cache_period_in_minutes: Long,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExecuteStatementInput {
    #[serde(rename = "Statement")]
    pub statement: PartiQLStatement,
    #[serde(rename = "Parameters")]
    pub parameters: Option<PreparedStatementParameters>,
    #[serde(rename = "ConsistentRead")]
    pub consistent_read: Option<ConsistentRead>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<PartiQLNextToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExecuteStatementOutput {
    #[serde(rename = "Items")]
    pub items: Option<ItemList>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<PartiQLNextToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExecuteTransactionInput {
    #[serde(rename = "TransactStatements")]
    pub transact_statements: ParameterizedStatements,
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: Option<ClientRequestToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExecuteTransactionOutput {
    #[serde(rename = "Responses")]
    pub responses: Option<ItemResponseList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExpectedAttributeValue {
    #[serde(rename = "Value")]
    pub value: Option<AttributeValue>,
    #[serde(rename = "Exists")]
    pub exists: Option<BooleanObject>,
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: Option<ComparisonOperator>,
    #[serde(rename = "AttributeValueList")]
    pub attribute_value_list: Option<AttributeValueList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportConflictException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportDescription {
    #[serde(rename = "ExportArn")]
    pub export_arn: Option<ExportArn>,
    #[serde(rename = "ExportStatus")]
    pub export_status: Option<ExportStatus>,
    #[serde(rename = "StartTime")]
    pub start_time: Option<ExportStartTime>,
    #[serde(rename = "EndTime")]
    pub end_time: Option<ExportEndTime>,
    #[serde(rename = "ExportManifest")]
    pub export_manifest: Option<ExportManifest>,
    #[serde(rename = "TableArn")]
    pub table_arn: Option<TableArn>,
    #[serde(rename = "TableId")]
    pub table_id: Option<TableId>,
    #[serde(rename = "ExportTime")]
    pub export_time: Option<ExportTime>,
    #[serde(rename = "ClientToken")]
    pub client_token: Option<ClientToken>,
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: Option<S3Bucket>,
    #[serde(rename = "S3BucketOwner")]
    pub s3_bucket_owner: Option<S3BucketOwner>,
    #[serde(rename = "S3Prefix")]
    pub s3_prefix: Option<S3Prefix>,
    #[serde(rename = "S3SseAlgorithm")]
    pub s3_sse_algorithm: Option<S3SseAlgorithm>,
    #[serde(rename = "S3SseKmsKeyId")]
    pub s3_sse_kms_key_id: Option<S3SseKmsKeyId>,
    #[serde(rename = "FailureCode")]
    pub failure_code: Option<FailureCode>,
    #[serde(rename = "FailureMessage")]
    pub failure_message: Option<FailureMessage>,
    #[serde(rename = "ExportFormat")]
    pub export_format: Option<ExportFormat>,
    #[serde(rename = "BilledSizeBytes")]
    pub billed_size_bytes: Option<BilledSizeBytes>,
    #[serde(rename = "ItemCount")]
    pub item_count: Option<ItemCount>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportNotFoundException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportSummary {
    #[serde(rename = "ExportArn")]
    pub export_arn: Option<ExportArn>,
    #[serde(rename = "ExportStatus")]
    pub export_status: Option<ExportStatus>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportTableToPointInTimeInput {
    #[serde(rename = "TableArn")]
    pub table_arn: TableArn,
    #[serde(rename = "ExportTime")]
    pub export_time: Option<ExportTime>,
    #[serde(rename = "ClientToken")]
    pub client_token: Option<ClientToken>,
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: S3Bucket,
    #[serde(rename = "S3BucketOwner")]
    pub s3_bucket_owner: Option<S3BucketOwner>,
    #[serde(rename = "S3Prefix")]
    pub s3_prefix: Option<S3Prefix>,
    #[serde(rename = "S3SseAlgorithm")]
    pub s3_sse_algorithm: Option<S3SseAlgorithm>,
    #[serde(rename = "S3SseKmsKeyId")]
    pub s3_sse_kms_key_id: Option<S3SseKmsKeyId>,
    #[serde(rename = "ExportFormat")]
    pub export_format: Option<ExportFormat>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportTableToPointInTimeOutput {
    #[serde(rename = "ExportDescription")]
    pub export_description: Option<ExportDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FailureException {
    #[serde(rename = "ExceptionName")]
    pub exception_name: Option<ExceptionName>,
    #[serde(rename = "ExceptionDescription")]
    pub exception_description: Option<ExceptionDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Get {
    #[serde(rename = "Key")]
    pub key: Key,
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "ProjectionExpression")]
    pub projection_expression: Option<ProjectionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetItemInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "Key")]
    pub key: Key,
    #[serde(rename = "AttributesToGet")]
    pub attributes_to_get: Option<AttributeNameList>,
    #[serde(rename = "ConsistentRead")]
    pub consistent_read: Option<ConsistentRead>,
    #[serde(rename = "ReturnConsumedCapacity")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ProjectionExpression")]
    pub projection_expression: Option<ProjectionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetItemOutput {
    #[serde(rename = "Item")]
    pub item: Option<AttributeMap>,
    #[serde(rename = "ConsumedCapacity")]
    pub consumed_capacity: Option<ConsumedCapacity>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalSecondaryIndex {
    #[serde(rename = "IndexName")]
    pub index_name: IndexName,
    #[serde(rename = "KeySchema")]
    pub key_schema: KeySchema,
    #[serde(rename = "Projection")]
    pub projection: Projection,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalSecondaryIndexAutoScalingUpdate {
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingUpdate")]
    pub provisioned_write_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalSecondaryIndexDescription {
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "KeySchema")]
    pub key_schema: Option<KeySchema>,
    #[serde(rename = "Projection")]
    pub projection: Option<Projection>,
    #[serde(rename = "IndexStatus")]
    pub index_status: Option<IndexStatus>,
    #[serde(rename = "Backfilling")]
    pub backfilling: Option<Backfilling>,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughputDescription>,
    #[serde(rename = "IndexSizeBytes")]
    pub index_size_bytes: Option<Long>,
    #[serde(rename = "ItemCount")]
    pub item_count: Option<Long>,
    #[serde(rename = "IndexArn")]
    pub index_arn: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalSecondaryIndexInfo {
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "KeySchema")]
    pub key_schema: Option<KeySchema>,
    #[serde(rename = "Projection")]
    pub projection: Option<Projection>,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalSecondaryIndexUpdate {
    #[serde(rename = "Update")]
    pub update: Option<UpdateGlobalSecondaryIndexAction>,
    #[serde(rename = "Create")]
    pub create: Option<CreateGlobalSecondaryIndexAction>,
    #[serde(rename = "Delete")]
    pub delete: Option<DeleteGlobalSecondaryIndexAction>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalTable {
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: Option<TableName>,
    #[serde(rename = "ReplicationGroup")]
    pub replication_group: Option<ReplicaList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalTableAlreadyExistsException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalTableDescription {
    #[serde(rename = "ReplicationGroup")]
    pub replication_group: Option<ReplicaDescriptionList>,
    #[serde(rename = "GlobalTableArn")]
    pub global_table_arn: Option<GlobalTableArnString>,
    #[serde(rename = "CreationDateTime")]
    pub creation_date_time: Option<Date>,
    #[serde(rename = "GlobalTableStatus")]
    pub global_table_status: Option<GlobalTableStatus>,
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: Option<TableName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalTableGlobalSecondaryIndexSettingsUpdate {
    #[serde(rename = "IndexName")]
    pub index_name: IndexName,
    #[serde(rename = "ProvisionedWriteCapacityUnits")]
    pub provisioned_write_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingSettingsUpdate")]
    pub provisioned_write_capacity_auto_scaling_settings_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalTableNotFoundException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IdempotentParameterMismatchException {
    #[serde(rename = "Message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IndexNotFoundException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InternalServerError {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidExportTimeException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidRestoreTimeException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ItemCollectionMetrics {
    #[serde(rename = "ItemCollectionKey")]
    pub item_collection_key: Option<ItemCollectionKeyAttributeMap>,
    #[serde(rename = "SizeEstimateRangeGB")]
    pub size_estimate_range_gb: Option<ItemCollectionSizeEstimateRange>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ItemCollectionSizeLimitExceededException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ItemResponse {
    #[serde(rename = "Item")]
    pub item: Option<AttributeMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KeySchemaElement {
    #[serde(rename = "AttributeName")]
    pub attribute_name: KeySchemaAttributeName,
    #[serde(rename = "KeyType")]
    pub key_type: KeyType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KeysAndAttributes {
    #[serde(rename = "Keys")]
    pub keys: KeyList,
    #[serde(rename = "AttributesToGet")]
    pub attributes_to_get: Option<AttributeNameList>,
    #[serde(rename = "ConsistentRead")]
    pub consistent_read: Option<ConsistentRead>,
    #[serde(rename = "ProjectionExpression")]
    pub projection_expression: Option<ProjectionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KinesisDataStreamDestination {
    #[serde(rename = "StreamArn")]
    pub stream_arn: Option<StreamArn>,
    #[serde(rename = "DestinationStatus")]
    pub destination_status: Option<DestinationStatus>,
    #[serde(rename = "DestinationStatusDescription")]
    pub destination_status_description: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KinesisStreamingDestinationInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "StreamArn")]
    pub stream_arn: StreamArn,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KinesisStreamingDestinationOutput {
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "StreamArn")]
    pub stream_arn: Option<StreamArn>,
    #[serde(rename = "DestinationStatus")]
    pub destination_status: Option<DestinationStatus>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LimitExceededException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListBackupsInput {
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "Limit")]
    pub limit: Option<BackupsInputLimit>,
    #[serde(rename = "TimeRangeLowerBound")]
    pub time_range_lower_bound: Option<TimeRangeLowerBound>,
    #[serde(rename = "TimeRangeUpperBound")]
    pub time_range_upper_bound: Option<TimeRangeUpperBound>,
    #[serde(rename = "ExclusiveStartBackupArn")]
    pub exclusive_start_backup_arn: Option<BackupArn>,
    #[serde(rename = "BackupType")]
    pub backup_type: Option<BackupTypeFilter>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListBackupsOutput {
    #[serde(rename = "BackupSummaries")]
    pub backup_summaries: Option<BackupSummaries>,
    #[serde(rename = "LastEvaluatedBackupArn")]
    pub last_evaluated_backup_arn: Option<BackupArn>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListContributorInsightsInput {
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<NextTokenString>,
    #[serde(rename = "MaxResults")]
    pub max_results: Option<ListContributorInsightsLimit>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListContributorInsightsOutput {
    #[serde(rename = "ContributorInsightsSummaries")]
    pub contributor_insights_summaries: Option<ContributorInsightsSummaries>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<NextTokenString>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListExportsInput {
    #[serde(rename = "TableArn")]
    pub table_arn: Option<TableArn>,
    #[serde(rename = "MaxResults")]
    pub max_results: Option<ListExportsMaxLimit>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<ExportNextToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListExportsOutput {
    #[serde(rename = "ExportSummaries")]
    pub export_summaries: Option<ExportSummaries>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<ExportNextToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListGlobalTablesInput {
    #[serde(rename = "ExclusiveStartGlobalTableName")]
    pub exclusive_start_global_table_name: Option<TableName>,
    #[serde(rename = "Limit")]
    pub limit: Option<PositiveIntegerObject>,
    #[serde(rename = "RegionName")]
    pub region_name: Option<RegionName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListGlobalTablesOutput {
    #[serde(rename = "GlobalTables")]
    pub global_tables: Option<GlobalTableList>,
    #[serde(rename = "LastEvaluatedGlobalTableName")]
    pub last_evaluated_global_table_name: Option<TableName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTablesInput {
    #[serde(rename = "ExclusiveStartTableName")]
    pub exclusive_start_table_name: Option<TableName>,
    #[serde(rename = "Limit")]
    pub limit: Option<ListTablesInputLimit>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTablesOutput {
    #[serde(rename = "TableNames")]
    pub table_names: Option<TableNameList>,
    #[serde(rename = "LastEvaluatedTableName")]
    pub last_evaluated_table_name: Option<TableName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTagsOfResourceInput {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: ResourceArnString,
    #[serde(rename = "NextToken")]
    pub next_token: Option<NextTokenString>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTagsOfResourceOutput {
    #[serde(rename = "Tags")]
    pub tags: Option<TagList>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<NextTokenString>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LocalSecondaryIndex {
    #[serde(rename = "IndexName")]
    pub index_name: IndexName,
    #[serde(rename = "KeySchema")]
    pub key_schema: KeySchema,
    #[serde(rename = "Projection")]
    pub projection: Projection,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LocalSecondaryIndexDescription {
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "KeySchema")]
    pub key_schema: Option<KeySchema>,
    #[serde(rename = "Projection")]
    pub projection: Option<Projection>,
    #[serde(rename = "IndexSizeBytes")]
    pub index_size_bytes: Option<Long>,
    #[serde(rename = "ItemCount")]
    pub item_count: Option<Long>,
    #[serde(rename = "IndexArn")]
    pub index_arn: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LocalSecondaryIndexInfo {
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "KeySchema")]
    pub key_schema: Option<KeySchema>,
    #[serde(rename = "Projection")]
    pub projection: Option<Projection>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParameterizedStatement {
    #[serde(rename = "Statement")]
    pub statement: PartiQLStatement,
    #[serde(rename = "Parameters")]
    pub parameters: Option<PreparedStatementParameters>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PointInTimeRecoveryDescription {
    #[serde(rename = "PointInTimeRecoveryStatus")]
    pub point_in_time_recovery_status: Option<PointInTimeRecoveryStatus>,
    #[serde(rename = "EarliestRestorableDateTime")]
    pub earliest_restorable_date_time: Option<Date>,
    #[serde(rename = "LatestRestorableDateTime")]
    pub latest_restorable_date_time: Option<Date>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PointInTimeRecoverySpecification {
    #[serde(rename = "PointInTimeRecoveryEnabled")]
    pub point_in_time_recovery_enabled: BooleanObject,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PointInTimeRecoveryUnavailableException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Projection {
    #[serde(rename = "ProjectionType")]
    pub projection_type: Option<ProjectionType>,
    #[serde(rename = "NonKeyAttributes")]
    pub non_key_attributes: Option<NonKeyAttributeNameList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProvisionedThroughput {
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: PositiveLongObject,
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: PositiveLongObject,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProvisionedThroughputDescription {
    #[serde(rename = "LastIncreaseDateTime")]
    pub last_increase_date_time: Option<Date>,
    #[serde(rename = "LastDecreaseDateTime")]
    pub last_decrease_date_time: Option<Date>,
    #[serde(rename = "NumberOfDecreasesToday")]
    pub number_of_decreases_today: Option<PositiveLongObject>,
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: Option<NonNegativeLongObject>,
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: Option<NonNegativeLongObject>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProvisionedThroughputExceededException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProvisionedThroughputOverride {
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: Option<PositiveLongObject>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Put {
    #[serde(rename = "Item")]
    pub item: PutItemInputAttributeMap,
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "ConditionExpression")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    pub return_values_on_condition_check_failure: Option<ReturnValuesOnConditionCheckFailure>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutItemInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "Item")]
    pub item: PutItemInputAttributeMap,
    #[serde(rename = "Expected")]
    pub expected: Option<ExpectedAttributeMap>,
    #[serde(rename = "ReturnValues")]
    pub return_values: Option<ReturnValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
    #[serde(rename = "ConditionalOperator")]
    pub conditional_operator: Option<ConditionalOperator>,
    #[serde(rename = "ConditionExpression")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutItemOutput {
    #[serde(rename = "Attributes")]
    pub attributes: Option<AttributeMap>,
    #[serde(rename = "ConsumedCapacity")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "ItemCollectionMetrics")]
    pub item_collection_metrics: Option<ItemCollectionMetrics>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutRequest {
    #[serde(rename = "Item")]
    pub item: PutItemInputAttributeMap,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueryInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "Select")]
    pub select: Option<Select>,
    #[serde(rename = "AttributesToGet")]
    pub attributes_to_get: Option<AttributeNameList>,
    #[serde(rename = "Limit")]
    pub limit: Option<PositiveIntegerObject>,
    #[serde(rename = "ConsistentRead")]
    pub consistent_read: Option<ConsistentRead>,
    #[serde(rename = "KeyConditions")]
    pub key_conditions: Option<KeyConditions>,
    #[serde(rename = "QueryFilter")]
    pub query_filter: Option<FilterConditionMap>,
    #[serde(rename = "ConditionalOperator")]
    pub conditional_operator: Option<ConditionalOperator>,
    #[serde(rename = "ScanIndexForward")]
    pub scan_index_forward: Option<BooleanObject>,
    #[serde(rename = "ExclusiveStartKey")]
    pub exclusive_start_key: Option<Key>,
    #[serde(rename = "ReturnConsumedCapacity")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ProjectionExpression")]
    pub projection_expression: Option<ProjectionExpression>,
    #[serde(rename = "FilterExpression")]
    pub filter_expression: Option<ConditionExpression>,
    #[serde(rename = "KeyConditionExpression")]
    pub key_condition_expression: Option<KeyExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueryOutput {
    #[serde(rename = "Items")]
    pub items: Option<ItemList>,
    #[serde(rename = "Count")]
    pub count: Option<Integer>,
    #[serde(rename = "ScannedCount")]
    pub scanned_count: Option<Integer>,
    #[serde(rename = "LastEvaluatedKey")]
    pub last_evaluated_key: Option<Key>,
    #[serde(rename = "ConsumedCapacity")]
    pub consumed_capacity: Option<ConsumedCapacity>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Replica {
    #[serde(rename = "RegionName")]
    pub region_name: Option<RegionName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaAlreadyExistsException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaAutoScalingDescription {
    #[serde(rename = "RegionName")]
    pub region_name: Option<RegionName>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<ReplicaGlobalSecondaryIndexAutoScalingDescriptionList>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingSettings")]
    pub replica_provisioned_read_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaProvisionedWriteCapacityAutoScalingSettings")]
    pub replica_provisioned_write_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaStatus")]
    pub replica_status: Option<ReplicaStatus>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaAutoScalingUpdate {
    #[serde(rename = "RegionName")]
    pub region_name: RegionName,
    #[serde(rename = "ReplicaGlobalSecondaryIndexUpdates")]
    pub replica_global_secondary_index_updates: Option<ReplicaGlobalSecondaryIndexAutoScalingUpdateList>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingUpdate")]
    pub replica_provisioned_read_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaDescription {
    #[serde(rename = "RegionName")]
    pub region_name: Option<RegionName>,
    #[serde(rename = "ReplicaStatus")]
    pub replica_status: Option<ReplicaStatus>,
    #[serde(rename = "ReplicaStatusDescription")]
    pub replica_status_description: Option<ReplicaStatusDescription>,
    #[serde(rename = "ReplicaStatusPercentProgress")]
    pub replica_status_percent_progress: Option<ReplicaStatusPercentProgress>,
    #[serde(rename = "KMSMasterKeyId")]
    pub kms_master_key_id: Option<KMSMasterKeyId>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<ReplicaGlobalSecondaryIndexDescriptionList>,
    #[serde(rename = "ReplicaInaccessibleDateTime")]
    pub replica_inaccessible_date_time: Option<Date>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndex {
    #[serde(rename = "IndexName")]
    pub index_name: IndexName,
    #[serde(rename = "ProvisionedThroughputOverride")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndexAutoScalingDescription {
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "IndexStatus")]
    pub index_status: Option<IndexStatus>,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingSettings")]
    pub provisioned_read_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingSettings")]
    pub provisioned_write_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndexAutoScalingUpdate {
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingUpdate")]
    pub provisioned_read_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndexDescription {
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndexSettingsDescription {
    #[serde(rename = "IndexName")]
    pub index_name: IndexName,
    #[serde(rename = "IndexStatus")]
    pub index_status: Option<IndexStatus>,
    #[serde(rename = "ProvisionedReadCapacityUnits")]
    pub provisioned_read_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingSettings")]
    pub provisioned_read_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ProvisionedWriteCapacityUnits")]
    pub provisioned_write_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingSettings")]
    pub provisioned_write_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndexSettingsUpdate {
    #[serde(rename = "IndexName")]
    pub index_name: IndexName,
    #[serde(rename = "ProvisionedReadCapacityUnits")]
    pub provisioned_read_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingSettingsUpdate")]
    pub provisioned_read_capacity_auto_scaling_settings_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaNotFoundException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaSettingsDescription {
    #[serde(rename = "RegionName")]
    pub region_name: RegionName,
    #[serde(rename = "ReplicaStatus")]
    pub replica_status: Option<ReplicaStatus>,
    #[serde(rename = "ReplicaBillingModeSummary")]
    pub replica_billing_mode_summary: Option<BillingModeSummary>,
    #[serde(rename = "ReplicaProvisionedReadCapacityUnits")]
    pub replica_provisioned_read_capacity_units: Option<NonNegativeLongObject>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingSettings")]
    pub replica_provisioned_read_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaProvisionedWriteCapacityUnits")]
    pub replica_provisioned_write_capacity_units: Option<NonNegativeLongObject>,
    #[serde(rename = "ReplicaProvisionedWriteCapacityAutoScalingSettings")]
    pub replica_provisioned_write_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaGlobalSecondaryIndexSettings")]
    pub replica_global_secondary_index_settings: Option<ReplicaGlobalSecondaryIndexSettingsDescriptionList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaSettingsUpdate {
    #[serde(rename = "RegionName")]
    pub region_name: RegionName,
    #[serde(rename = "ReplicaProvisionedReadCapacityUnits")]
    pub replica_provisioned_read_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingSettingsUpdate")]
    pub replica_provisioned_read_capacity_auto_scaling_settings_update: Option<AutoScalingSettingsUpdate>,
    #[serde(rename = "ReplicaGlobalSecondaryIndexSettingsUpdate")]
    pub replica_global_secondary_index_settings_update: Option<ReplicaGlobalSecondaryIndexSettingsUpdateList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaUpdate {
    #[serde(rename = "Create")]
    pub create: Option<CreateReplicaAction>,
    #[serde(rename = "Delete")]
    pub delete: Option<DeleteReplicaAction>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicationGroupUpdate {
    #[serde(rename = "Create")]
    pub create: Option<CreateReplicationGroupMemberAction>,
    #[serde(rename = "Update")]
    pub update: Option<UpdateReplicationGroupMemberAction>,
    #[serde(rename = "Delete")]
    pub delete: Option<DeleteReplicationGroupMemberAction>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestLimitExceeded {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceInUseException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceNotFoundException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreSummary {
    #[serde(rename = "SourceBackupArn")]
    pub source_backup_arn: Option<BackupArn>,
    #[serde(rename = "SourceTableArn")]
    pub source_table_arn: Option<TableArn>,
    #[serde(rename = "RestoreDateTime")]
    pub restore_date_time: Date,
    #[serde(rename = "RestoreInProgress")]
    pub restore_in_progress: RestoreInProgress,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreTableFromBackupInput {
    #[serde(rename = "TargetTableName")]
    pub target_table_name: TableName,
    #[serde(rename = "BackupArn")]
    pub backup_arn: BackupArn,
    #[serde(rename = "BillingModeOverride")]
    pub billing_mode_override: Option<BillingMode>,
    #[serde(rename = "GlobalSecondaryIndexOverride")]
    pub global_secondary_index_override: Option<GlobalSecondaryIndexList>,
    #[serde(rename = "LocalSecondaryIndexOverride")]
    pub local_secondary_index_override: Option<LocalSecondaryIndexList>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    pub provisioned_throughput_override: Option<ProvisionedThroughput>,
    #[serde(rename = "SSESpecificationOverride")]
    pub sse_specification_override: Option<SSESpecification>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreTableFromBackupOutput {
    #[serde(rename = "TableDescription")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreTableToPointInTimeInput {
    #[serde(rename = "SourceTableArn")]
    pub source_table_arn: Option<TableArn>,
    #[serde(rename = "SourceTableName")]
    pub source_table_name: Option<TableName>,
    #[serde(rename = "TargetTableName")]
    pub target_table_name: TableName,
    #[serde(rename = "UseLatestRestorableTime")]
    pub use_latest_restorable_time: Option<BooleanObject>,
    #[serde(rename = "RestoreDateTime")]
    pub restore_date_time: Option<Date>,
    #[serde(rename = "BillingModeOverride")]
    pub billing_mode_override: Option<BillingMode>,
    #[serde(rename = "GlobalSecondaryIndexOverride")]
    pub global_secondary_index_override: Option<GlobalSecondaryIndexList>,
    #[serde(rename = "LocalSecondaryIndexOverride")]
    pub local_secondary_index_override: Option<LocalSecondaryIndexList>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    pub provisioned_throughput_override: Option<ProvisionedThroughput>,
    #[serde(rename = "SSESpecificationOverride")]
    pub sse_specification_override: Option<SSESpecification>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreTableToPointInTimeOutput {
    #[serde(rename = "TableDescription")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SSEDescription {
    #[serde(rename = "Status")]
    pub status: Option<SSEStatus>,
    #[serde(rename = "SSEType")]
    pub sse_type: Option<SSEType>,
    #[serde(rename = "KMSMasterKeyArn")]
    pub kms_master_key_arn: Option<KMSMasterKeyArn>,
    #[serde(rename = "InaccessibleEncryptionDateTime")]
    pub inaccessible_encryption_date_time: Option<Date>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SSESpecification {
    #[serde(rename = "Enabled")]
    pub enabled: Option<SSEEnabled>,
    #[serde(rename = "SSEType")]
    pub sse_type: Option<SSEType>,
    #[serde(rename = "KMSMasterKeyId")]
    pub kms_master_key_id: Option<KMSMasterKeyId>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScanInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "AttributesToGet")]
    pub attributes_to_get: Option<AttributeNameList>,
    #[serde(rename = "Limit")]
    pub limit: Option<PositiveIntegerObject>,
    #[serde(rename = "Select")]
    pub select: Option<Select>,
    #[serde(rename = "ScanFilter")]
    pub scan_filter: Option<FilterConditionMap>,
    #[serde(rename = "ConditionalOperator")]
    pub conditional_operator: Option<ConditionalOperator>,
    #[serde(rename = "ExclusiveStartKey")]
    pub exclusive_start_key: Option<Key>,
    #[serde(rename = "ReturnConsumedCapacity")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "TotalSegments")]
    pub total_segments: Option<ScanTotalSegments>,
    #[serde(rename = "Segment")]
    pub segment: Option<ScanSegment>,
    #[serde(rename = "ProjectionExpression")]
    pub projection_expression: Option<ProjectionExpression>,
    #[serde(rename = "FilterExpression")]
    pub filter_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
    #[serde(rename = "ConsistentRead")]
    pub consistent_read: Option<ConsistentRead>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScanOutput {
    #[serde(rename = "Items")]
    pub items: Option<ItemList>,
    #[serde(rename = "Count")]
    pub count: Option<Integer>,
    #[serde(rename = "ScannedCount")]
    pub scanned_count: Option<Integer>,
    #[serde(rename = "LastEvaluatedKey")]
    pub last_evaluated_key: Option<Key>,
    #[serde(rename = "ConsumedCapacity")]
    pub consumed_capacity: Option<ConsumedCapacity>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SourceTableDetails {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "TableId")]
    pub table_id: TableId,
    #[serde(rename = "TableArn")]
    pub table_arn: Option<TableArn>,
    #[serde(rename = "TableSizeBytes")]
    pub table_size_bytes: Option<Long>,
    #[serde(rename = "KeySchema")]
    pub key_schema: KeySchema,
    #[serde(rename = "TableCreationDateTime")]
    pub table_creation_date_time: TableCreationDateTime,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ProvisionedThroughput,
    #[serde(rename = "ItemCount")]
    pub item_count: Option<ItemCount>,
    #[serde(rename = "BillingMode")]
    pub billing_mode: Option<BillingMode>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SourceTableFeatureDetails {
    #[serde(rename = "LocalSecondaryIndexes")]
    pub local_secondary_indexes: Option<LocalSecondaryIndexes>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<GlobalSecondaryIndexes>,
    #[serde(rename = "StreamDescription")]
    pub stream_description: Option<StreamSpecification>,
    #[serde(rename = "TimeToLiveDescription")]
    pub time_to_live_description: Option<TimeToLiveDescription>,
    #[serde(rename = "SSEDescription")]
    pub sse_description: Option<SSEDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StreamSpecification {
    #[serde(rename = "StreamEnabled")]
    pub stream_enabled: StreamEnabled,
    #[serde(rename = "StreamViewType")]
    pub stream_view_type: Option<StreamViewType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TableAlreadyExistsException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TableAutoScalingDescription {
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "TableStatus")]
    pub table_status: Option<TableStatus>,
    #[serde(rename = "Replicas")]
    pub replicas: Option<ReplicaAutoScalingDescriptionList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TableDescription {
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: Option<AttributeDefinitions>,
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "KeySchema")]
    pub key_schema: Option<KeySchema>,
    #[serde(rename = "TableStatus")]
    pub table_status: Option<TableStatus>,
    #[serde(rename = "CreationDateTime")]
    pub creation_date_time: Option<Date>,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughputDescription>,
    #[serde(rename = "TableSizeBytes")]
    pub table_size_bytes: Option<Long>,
    #[serde(rename = "ItemCount")]
    pub item_count: Option<Long>,
    #[serde(rename = "TableArn")]
    pub table_arn: Option<String>,
    #[serde(rename = "TableId")]
    pub table_id: Option<TableId>,
    #[serde(rename = "BillingModeSummary")]
    pub billing_mode_summary: Option<BillingModeSummary>,
    #[serde(rename = "LocalSecondaryIndexes")]
    pub local_secondary_indexes: Option<LocalSecondaryIndexDescriptionList>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<GlobalSecondaryIndexDescriptionList>,
    #[serde(rename = "StreamSpecification")]
    pub stream_specification: Option<StreamSpecification>,
    #[serde(rename = "LatestStreamLabel")]
    pub latest_stream_label: Option<String>,
    #[serde(rename = "LatestStreamArn")]
    pub latest_stream_arn: Option<StreamArn>,
    #[serde(rename = "GlobalTableVersion")]
    pub global_table_version: Option<String>,
    #[serde(rename = "Replicas")]
    pub replicas: Option<ReplicaDescriptionList>,
    #[serde(rename = "RestoreSummary")]
    pub restore_summary: Option<RestoreSummary>,
    #[serde(rename = "SSEDescription")]
    pub sse_description: Option<SSEDescription>,
    #[serde(rename = "ArchivalSummary")]
    pub archival_summary: Option<ArchivalSummary>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TableInUseException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TableNotFoundException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: TagKeyString,
    #[serde(rename = "Value")]
    pub value: TagValueString,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TagResourceInput {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: ResourceArnString,
    #[serde(rename = "Tags")]
    pub tags: TagList,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TimeToLiveDescription {
    #[serde(rename = "TimeToLiveStatus")]
    pub time_to_live_status: Option<TimeToLiveStatus>,
    #[serde(rename = "AttributeName")]
    pub attribute_name: Option<TimeToLiveAttributeName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TimeToLiveSpecification {
    #[serde(rename = "Enabled")]
    pub enabled: TimeToLiveEnabled,
    #[serde(rename = "AttributeName")]
    pub attribute_name: TimeToLiveAttributeName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactGetItem {
    #[serde(rename = "Get")]
    pub get: Get,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactGetItemsInput {
    #[serde(rename = "TransactItems")]
    pub transact_items: TransactGetItemList,
    #[serde(rename = "ReturnConsumedCapacity")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactGetItemsOutput {
    #[serde(rename = "ConsumedCapacity")]
    pub consumed_capacity: Option<ConsumedCapacityMultiple>,
    #[serde(rename = "Responses")]
    pub responses: Option<ItemResponseList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactWriteItem {
    #[serde(rename = "ConditionCheck")]
    pub condition_check: Option<ConditionCheck>,
    #[serde(rename = "Put")]
    pub put: Option<Put>,
    #[serde(rename = "Delete")]
    pub delete: Option<Delete>,
    #[serde(rename = "Update")]
    pub update: Option<Update>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactWriteItemsInput {
    #[serde(rename = "TransactItems")]
    pub transact_items: TransactWriteItemList,
    #[serde(rename = "ReturnConsumedCapacity")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: Option<ClientRequestToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactWriteItemsOutput {
    #[serde(rename = "ConsumedCapacity")]
    pub consumed_capacity: Option<ConsumedCapacityMultiple>,
    #[serde(rename = "ItemCollectionMetrics")]
    pub item_collection_metrics: Option<ItemCollectionMetricsPerTable>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactionCanceledException {
    #[serde(rename = "Message")]
    pub message: Option<ErrorMessage>,
    #[serde(rename = "CancellationReasons")]
    pub cancellation_reasons: Option<CancellationReasonList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactionConflictException {
    #[serde(rename = "message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactionInProgressException {
    #[serde(rename = "Message")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UntagResourceInput {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: ResourceArnString,
    #[serde(rename = "TagKeys")]
    pub tag_keys: TagKeyList,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Update {
    #[serde(rename = "Key")]
    pub key: Key,
    #[serde(rename = "UpdateExpression")]
    pub update_expression: UpdateExpression,
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "ConditionExpression")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    pub return_values_on_condition_check_failure: Option<ReturnValuesOnConditionCheckFailure>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateContinuousBackupsInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "PointInTimeRecoverySpecification")]
    pub point_in_time_recovery_specification: PointInTimeRecoverySpecification,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateContinuousBackupsOutput {
    #[serde(rename = "ContinuousBackupsDescription")]
    pub continuous_backups_description: Option<ContinuousBackupsDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateContributorInsightsInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ContributorInsightsAction")]
    pub contributor_insights_action: ContributorInsightsAction,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateContributorInsightsOutput {
    #[serde(rename = "TableName")]
    pub table_name: Option<TableName>,
    #[serde(rename = "IndexName")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ContributorInsightsStatus")]
    pub contributor_insights_status: Option<ContributorInsightsStatus>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateGlobalSecondaryIndexAction {
    #[serde(rename = "IndexName")]
    pub index_name: IndexName,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ProvisionedThroughput,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateGlobalTableInput {
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: TableName,
    #[serde(rename = "ReplicaUpdates")]
    pub replica_updates: ReplicaUpdateList,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateGlobalTableOutput {
    #[serde(rename = "GlobalTableDescription")]
    pub global_table_description: Option<GlobalTableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateGlobalTableSettingsInput {
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: TableName,
    #[serde(rename = "GlobalTableBillingMode")]
    pub global_table_billing_mode: Option<BillingMode>,
    #[serde(rename = "GlobalTableProvisionedWriteCapacityUnits")]
    pub global_table_provisioned_write_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "GlobalTableProvisionedWriteCapacityAutoScalingSettingsUpdate")]
    pub global_table_provisioned_write_capacity_auto_scaling_settings_update: Option<AutoScalingSettingsUpdate>,
    #[serde(rename = "GlobalTableGlobalSecondaryIndexSettingsUpdate")]
    pub global_table_global_secondary_index_settings_update: Option<GlobalTableGlobalSecondaryIndexSettingsUpdateList>,
    #[serde(rename = "ReplicaSettingsUpdate")]
    pub replica_settings_update: Option<ReplicaSettingsUpdateList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateGlobalTableSettingsOutput {
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: Option<TableName>,
    #[serde(rename = "ReplicaSettings")]
    pub replica_settings: Option<ReplicaSettingsDescriptionList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateItemInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "Key")]
    pub key: Key,
    #[serde(rename = "AttributeUpdates")]
    pub attribute_updates: Option<AttributeUpdates>,
    #[serde(rename = "Expected")]
    pub expected: Option<ExpectedAttributeMap>,
    #[serde(rename = "ConditionalOperator")]
    pub conditional_operator: Option<ConditionalOperator>,
    #[serde(rename = "ReturnValues")]
    pub return_values: Option<ReturnValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
    #[serde(rename = "UpdateExpression")]
    pub update_expression: Option<UpdateExpression>,
    #[serde(rename = "ConditionExpression")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateItemOutput {
    #[serde(rename = "Attributes")]
    pub attributes: Option<AttributeMap>,
    #[serde(rename = "ConsumedCapacity")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "ItemCollectionMetrics")]
    pub item_collection_metrics: Option<ItemCollectionMetrics>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateReplicationGroupMemberAction {
    #[serde(rename = "RegionName")]
    pub region_name: RegionName,
    #[serde(rename = "KMSMasterKeyId")]
    pub kms_master_key_id: Option<KMSMasterKeyId>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<ReplicaGlobalSecondaryIndexList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateTableInput {
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: Option<AttributeDefinitions>,
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "BillingMode")]
    pub billing_mode: Option<BillingMode>,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "GlobalSecondaryIndexUpdates")]
    pub global_secondary_index_updates: Option<GlobalSecondaryIndexUpdateList>,
    #[serde(rename = "StreamSpecification")]
    pub stream_specification: Option<StreamSpecification>,
    #[serde(rename = "SSESpecification")]
    pub sse_specification: Option<SSESpecification>,
    #[serde(rename = "ReplicaUpdates")]
    pub replica_updates: Option<ReplicationGroupUpdateList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateTableOutput {
    #[serde(rename = "TableDescription")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateTableReplicaAutoScalingInput {
    #[serde(rename = "GlobalSecondaryIndexUpdates")]
    pub global_secondary_index_updates: Option<GlobalSecondaryIndexAutoScalingUpdateList>,
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingUpdate")]
    pub provisioned_write_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
    #[serde(rename = "ReplicaUpdates")]
    pub replica_updates: Option<ReplicaAutoScalingUpdateList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateTableReplicaAutoScalingOutput {
    #[serde(rename = "TableAutoScalingDescription")]
    pub table_auto_scaling_description: Option<TableAutoScalingDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateTimeToLiveInput {
    #[serde(rename = "TableName")]
    pub table_name: TableName,
    #[serde(rename = "TimeToLiveSpecification")]
    pub time_to_live_specification: TimeToLiveSpecification,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateTimeToLiveOutput {
    #[serde(rename = "TimeToLiveSpecification")]
    pub time_to_live_specification: Option<TimeToLiveSpecification>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WriteRequest {
    #[serde(rename = "PutRequest")]
    pub put_request: Option<PutRequest>,
    #[serde(rename = "DeleteRequest")]
    pub delete_request: Option<DeleteRequest>,
}

