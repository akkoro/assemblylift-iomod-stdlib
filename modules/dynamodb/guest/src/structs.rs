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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival_date_time: Option<Date>,
    #[serde(rename = "ArchivalReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival_reason: Option<ArchivalReason>,
    #[serde(rename = "ArchivalBackupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<StringAttributeValue>,
    #[serde(rename = "N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<NumberAttributeValue>,
    #[serde(rename = "B")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<BinaryAttributeValue>,
    #[serde(rename = "SS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ss: Option<StringSetAttributeValue>,
    #[serde(rename = "NS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns: Option<NumberSetAttributeValue>,
    #[serde(rename = "BS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bs: Option<BinarySetAttributeValue>,
    #[serde(rename = "M")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<MapAttributeValue>,
    #[serde(rename = "L")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l: Option<ListAttributeValue>,
    #[serde(rename = "NULL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null: Option<NullAttributeValue>,
    #[serde(rename = "BOOL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool: Option<BooleanAttributeValue>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AttributeValueUpdate {
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<AttributeAction>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingPolicyDescription {
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<AutoScalingPolicyName>,
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tracking_scaling_policy_configuration: Option<AutoScalingTargetTrackingScalingPolicyConfigurationDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingPolicyUpdate {
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<AutoScalingPolicyName>,
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    
    pub target_tracking_scaling_policy_configuration: AutoScalingTargetTrackingScalingPolicyConfigurationUpdate,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingSettingsDescription {
    #[serde(rename = "MinimumUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_units: Option<PositiveLongObject>,
    #[serde(rename = "MaximumUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_units: Option<PositiveLongObject>,
    #[serde(rename = "AutoScalingDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_disabled: Option<BooleanObject>,
    #[serde(rename = "AutoScalingRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_role_arn: Option<String>,
    #[serde(rename = "ScalingPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<AutoScalingPolicyDescriptionList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingSettingsUpdate {
    #[serde(rename = "MinimumUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_units: Option<PositiveLongObject>,
    #[serde(rename = "MaximumUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_units: Option<PositiveLongObject>,
    #[serde(rename = "AutoScalingDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_disabled: Option<BooleanObject>,
    #[serde(rename = "AutoScalingRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_role_arn: Option<AutoScalingRoleArn>,
    #[serde(rename = "ScalingPolicyUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policy_update: Option<AutoScalingPolicyUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingTargetTrackingScalingPolicyConfigurationDescription {
    #[serde(rename = "DisableScaleIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_scale_in: Option<BooleanObject>,
    #[serde(rename = "ScaleInCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_cooldown: Option<IntegerObject>,
    #[serde(rename = "ScaleOutCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_out_cooldown: Option<IntegerObject>,
    #[serde(rename = "TargetValue")]
    
    pub target_value: Double,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AutoScalingTargetTrackingScalingPolicyConfigurationUpdate {
    #[serde(rename = "DisableScaleIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_scale_in: Option<BooleanObject>,
    #[serde(rename = "ScaleInCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_in_cooldown: Option<IntegerObject>,
    #[serde(rename = "ScaleOutCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_out_cooldown: Option<IntegerObject>,
    #[serde(rename = "TargetValue")]
    
    pub target_value: Double,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackupDescription {
    #[serde(rename = "BackupDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_details: Option<BackupDetails>,
    #[serde(rename = "SourceTableDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_details: Option<SourceTableDetails>,
    #[serde(rename = "SourceTableFeatureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_feature_details: Option<SourceTableFeatureDetails>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackupDetails {
    #[serde(rename = "BackupArn")]
    
    pub backup_arn: BackupArn,
    #[serde(rename = "BackupName")]
    
    pub backup_name: BackupName,
    #[serde(rename = "BackupSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_bytes: Option<BackupSizeBytes>,
    #[serde(rename = "BackupStatus")]
    
    pub backup_status: BackupStatus,
    #[serde(rename = "BackupType")]
    
    pub backup_type: BackupType,
    #[serde(rename = "BackupCreationDateTime")]
    
    pub backup_creation_date_time: BackupCreationDateTime,
    #[serde(rename = "BackupExpiryDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_expiry_date_time: Option<Date>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackupInUseException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackupNotFoundException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BackupSummary {
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "TableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<TableId>,
    #[serde(rename = "TableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<TableArn>,
    #[serde(rename = "BackupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_arn: Option<BackupArn>,
    #[serde(rename = "BackupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_name: Option<BackupName>,
    #[serde(rename = "BackupCreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_creation_date_time: Option<BackupCreationDateTime>,
    #[serde(rename = "BackupExpiryDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_expiry_date_time: Option<Date>,
    #[serde(rename = "BackupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_status: Option<BackupStatus>,
    #[serde(rename = "BackupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<BackupType>,
    #[serde(rename = "BackupSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<PartiQLBatchResponse>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchGetItemInput {
    #[serde(rename = "RequestItems")]
    
    pub request_items: BatchGetRequestMap,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchGetItemOutput {
    #[serde(rename = "Responses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<BatchGetResponseMap>,
    #[serde(rename = "UnprocessedKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_keys: Option<BatchGetRequestMap>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacityMultiple>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchStatementError {
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<BatchStatementErrorCodeEnum>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchStatementRequest {
    #[serde(rename = "Statement")]
    
    pub statement: PartiQLStatement,
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<PreparedStatementParameters>,
    #[serde(rename = "ConsistentRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<ConsistentRead>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchStatementResponse {
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<BatchStatementError>,
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<AttributeMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchWriteItemInput {
    #[serde(rename = "RequestItems")]
    
    pub request_items: BatchWriteItemRequestMap,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchWriteItemOutput {
    #[serde(rename = "UnprocessedItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_items: Option<BatchWriteItemRequestMap>,
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics: Option<ItemCollectionMetricsPerTable>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacityMultiple>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BillingModeSummary {
    #[serde(rename = "BillingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<BillingMode>,
    #[serde(rename = "LastUpdateToPayPerRequestDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_to_pay_per_request_date_time: Option<Date>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CancellationReason {
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<AttributeMap>,
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Capacity {
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<ConsumedCapacityUnits>,
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<ConsumedCapacityUnits>,
    #[serde(rename = "CapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<ConsumedCapacityUnits>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "AttributeValueList")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<ReturnValuesOnConditionCheckFailure>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConditionalCheckFailedException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConsumedCapacity {
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "CapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<ConsumedCapacityUnits>,
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<ConsumedCapacityUnits>,
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<ConsumedCapacityUnits>,
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Capacity>,
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<SecondaryIndexesCapacityMap>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<SecondaryIndexesCapacityMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContinuousBackupsDescription {
    #[serde(rename = "ContinuousBackupsStatus")]
    
    pub continuous_backups_status: ContinuousBackupsStatus,
    #[serde(rename = "PointInTimeRecoveryDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery_description: Option<PointInTimeRecoveryDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContinuousBackupsUnavailableException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContributorInsightsSummary {
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ContributorInsightsStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<KMSMasterKeyId>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<LocalSecondaryIndexList>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<GlobalSecondaryIndexList>,
    #[serde(rename = "BillingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<BillingMode>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "StreamSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,
    #[serde(rename = "SSESpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_specification: Option<SSESpecification>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateTableOutput {
    #[serde(rename = "TableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Delete {
    #[serde(rename = "Key")]
    
    pub key: Key,
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "ConditionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<ExpectedAttributeMap>,
    #[serde(rename = "ConditionalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<ConditionalOperator>,
    #[serde(rename = "ReturnValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values: Option<ReturnValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
    #[serde(rename = "ConditionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteItemOutput {
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributeMap>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_backups_description: Option<ContinuousBackupsDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeContributorInsightsInput {
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeContributorInsightsOutput {
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ContributorInsightsRuleList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_rule_list: Option<ContributorInsightsRuleList>,
    #[serde(rename = "ContributorInsightsStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_status: Option<ContributorInsightsStatus>,
    #[serde(rename = "LastUpdateDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<LastUpdateDateTime>,
    #[serde(rename = "FailureException")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_name: Option<TableName>,
    #[serde(rename = "ReplicaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "KinesisDataStreamDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_data_stream_destinations: Option<KinesisDataStreamDestinations>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeLimitsInput {
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeLimitsOutput {
    #[serde(rename = "AccountMaxReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_max_read_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "AccountMaxWriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_max_write_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "TableMaxReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_max_read_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "TableMaxWriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_description: Option<TimeToLiveDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DuplicateItemException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<PreparedStatementParameters>,
    #[serde(rename = "ConsistentRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<ConsistentRead>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<PartiQLNextToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExecuteStatementOutput {
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ItemList>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<PartiQLNextToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExecuteTransactionInput {
    #[serde(rename = "TransactStatements")]
    
    pub transact_statements: ParameterizedStatements,
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<ClientRequestToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExecuteTransactionOutput {
    #[serde(rename = "Responses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<ItemResponseList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExpectedAttributeValue {
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
    #[serde(rename = "Exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<BooleanObject>,
    #[serde(rename = "ComparisonOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<ComparisonOperator>,
    #[serde(rename = "AttributeValueList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value_list: Option<AttributeValueList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportConflictException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportDescription {
    #[serde(rename = "ExportArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<ExportArn>,
    #[serde(rename = "ExportStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<ExportStatus>,
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<ExportStartTime>,
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<ExportEndTime>,
    #[serde(rename = "ExportManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_manifest: Option<ExportManifest>,
    #[serde(rename = "TableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<TableArn>,
    #[serde(rename = "TableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<TableId>,
    #[serde(rename = "ExportTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_time: Option<ExportTime>,
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<ClientToken>,
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<S3Bucket>,
    #[serde(rename = "S3BucketOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_owner: Option<S3BucketOwner>,
    #[serde(rename = "S3Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<S3Prefix>,
    #[serde(rename = "S3SseAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_algorithm: Option<S3SseAlgorithm>,
    #[serde(rename = "S3SseKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_kms_key_id: Option<S3SseKmsKeyId>,
    #[serde(rename = "FailureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<FailureCode>,
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<FailureMessage>,
    #[serde(rename = "ExportFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<ExportFormat>,
    #[serde(rename = "BilledSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billed_size_bytes: Option<BilledSizeBytes>,
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<ItemCount>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportNotFoundException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportSummary {
    #[serde(rename = "ExportArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<ExportArn>,
    #[serde(rename = "ExportStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<ExportStatus>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportTableToPointInTimeInput {
    #[serde(rename = "TableArn")]
    
    pub table_arn: TableArn,
    #[serde(rename = "ExportTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_time: Option<ExportTime>,
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<ClientToken>,
    #[serde(rename = "S3Bucket")]
    
    pub s3_bucket: S3Bucket,
    #[serde(rename = "S3BucketOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_owner: Option<S3BucketOwner>,
    #[serde(rename = "S3Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<S3Prefix>,
    #[serde(rename = "S3SseAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_algorithm: Option<S3SseAlgorithm>,
    #[serde(rename = "S3SseKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_kms_key_id: Option<S3SseKmsKeyId>,
    #[serde(rename = "ExportFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<ExportFormat>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportTableToPointInTimeOutput {
    #[serde(rename = "ExportDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_description: Option<ExportDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FailureException {
    #[serde(rename = "ExceptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_name: Option<ExceptionName>,
    #[serde(rename = "ExceptionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_description: Option<ExceptionDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Get {
    #[serde(rename = "Key")]
    
    pub key: Key,
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "ProjectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<ProjectionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetItemInput {
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "Key")]
    
    pub key: Key,
    #[serde(rename = "AttributesToGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<AttributeNameList>,
    #[serde(rename = "ConsistentRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<ConsistentRead>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ProjectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<ProjectionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetItemOutput {
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<AttributeMap>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalSecondaryIndexAutoScalingUpdate {
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalSecondaryIndexDescription {
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<KeySchema>,
    #[serde(rename = "Projection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    #[serde(rename = "IndexStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<IndexStatus>,
    #[serde(rename = "Backfilling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfilling: Option<Backfilling>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughputDescription>,
    #[serde(rename = "IndexSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_size_bytes: Option<Long>,
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<Long>,
    #[serde(rename = "IndexArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalSecondaryIndexInfo {
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<KeySchema>,
    #[serde(rename = "Projection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalSecondaryIndexUpdate {
    #[serde(rename = "Update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateGlobalSecondaryIndexAction>,
    #[serde(rename = "Create")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<CreateGlobalSecondaryIndexAction>,
    #[serde(rename = "Delete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<DeleteGlobalSecondaryIndexAction>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalTable {
    #[serde(rename = "GlobalTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_name: Option<TableName>,
    #[serde(rename = "ReplicationGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicaList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalTableAlreadyExistsException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalTableDescription {
    #[serde(rename = "ReplicationGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicaDescriptionList>,
    #[serde(rename = "GlobalTableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_arn: Option<GlobalTableArnString>,
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<Date>,
    #[serde(rename = "GlobalTableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_status: Option<GlobalTableStatus>,
    #[serde(rename = "GlobalTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_name: Option<TableName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalTableGlobalSecondaryIndexSettingsUpdate {
    #[serde(rename = "IndexName")]
    
    pub index_name: IndexName,
    #[serde(rename = "ProvisionedWriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingSettingsUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_auto_scaling_settings_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalTableNotFoundException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IdempotentParameterMismatchException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IndexNotFoundException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InternalServerError {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidExportTimeException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidRestoreTimeException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ItemCollectionMetrics {
    #[serde(rename = "ItemCollectionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_key: Option<ItemCollectionKeyAttributeMap>,
    #[serde(rename = "SizeEstimateRangeGB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_estimate_range_gb: Option<ItemCollectionSizeEstimateRange>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ItemCollectionSizeLimitExceededException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ItemResponse {
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<AttributeNameList>,
    #[serde(rename = "ConsistentRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<ConsistentRead>,
    #[serde(rename = "ProjectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<ProjectionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KinesisDataStreamDestination {
    #[serde(rename = "StreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<StreamArn>,
    #[serde(rename = "DestinationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_status: Option<DestinationStatus>,
    #[serde(rename = "DestinationStatusDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "StreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<StreamArn>,
    #[serde(rename = "DestinationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_status: Option<DestinationStatus>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LimitExceededException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListBackupsInput {
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<BackupsInputLimit>,
    #[serde(rename = "TimeRangeLowerBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range_lower_bound: Option<TimeRangeLowerBound>,
    #[serde(rename = "TimeRangeUpperBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range_upper_bound: Option<TimeRangeUpperBound>,
    #[serde(rename = "ExclusiveStartBackupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_backup_arn: Option<BackupArn>,
    #[serde(rename = "BackupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<BackupTypeFilter>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListBackupsOutput {
    #[serde(rename = "BackupSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_summaries: Option<BackupSummaries>,
    #[serde(rename = "LastEvaluatedBackupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_backup_arn: Option<BackupArn>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListContributorInsightsInput {
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<NextTokenString>,
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<ListContributorInsightsLimit>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListContributorInsightsOutput {
    #[serde(rename = "ContributorInsightsSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_summaries: Option<ContributorInsightsSummaries>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<NextTokenString>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListExportsInput {
    #[serde(rename = "TableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<TableArn>,
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<ListExportsMaxLimit>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<ExportNextToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListExportsOutput {
    #[serde(rename = "ExportSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_summaries: Option<ExportSummaries>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<ExportNextToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListGlobalTablesInput {
    #[serde(rename = "ExclusiveStartGlobalTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_global_table_name: Option<TableName>,
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<PositiveIntegerObject>,
    #[serde(rename = "RegionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<RegionName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListGlobalTablesOutput {
    #[serde(rename = "GlobalTables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_tables: Option<GlobalTableList>,
    #[serde(rename = "LastEvaluatedGlobalTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_global_table_name: Option<TableName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTablesInput {
    #[serde(rename = "ExclusiveStartTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_table_name: Option<TableName>,
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<ListTablesInputLimit>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTablesOutput {
    #[serde(rename = "TableNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_names: Option<TableNameList>,
    #[serde(rename = "LastEvaluatedTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_table_name: Option<TableName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTagsOfResourceInput {
    #[serde(rename = "ResourceArn")]
    
    pub resource_arn: ResourceArnString,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<NextTokenString>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTagsOfResourceOutput {
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<KeySchema>,
    #[serde(rename = "Projection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    #[serde(rename = "IndexSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_size_bytes: Option<Long>,
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<Long>,
    #[serde(rename = "IndexArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LocalSecondaryIndexInfo {
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<KeySchema>,
    #[serde(rename = "Projection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParameterizedStatement {
    #[serde(rename = "Statement")]
    
    pub statement: PartiQLStatement,
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<PreparedStatementParameters>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PointInTimeRecoveryDescription {
    #[serde(rename = "PointInTimeRecoveryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery_status: Option<PointInTimeRecoveryStatus>,
    #[serde(rename = "EarliestRestorableDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_restorable_date_time: Option<Date>,
    #[serde(rename = "LatestRestorableDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Projection {
    #[serde(rename = "ProjectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_type: Option<ProjectionType>,
    #[serde(rename = "NonKeyAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_increase_date_time: Option<Date>,
    #[serde(rename = "LastDecreaseDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_decrease_date_time: Option<Date>,
    #[serde(rename = "NumberOfDecreasesToday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_decreases_today: Option<PositiveLongObject>,
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<NonNegativeLongObject>,
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<NonNegativeLongObject>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProvisionedThroughputExceededException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProvisionedThroughputOverride {
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<PositiveLongObject>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Put {
    #[serde(rename = "Item")]
    
    pub item: PutItemInputAttributeMap,
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "ConditionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values_on_condition_check_failure: Option<ReturnValuesOnConditionCheckFailure>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutItemInput {
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "Item")]
    
    pub item: PutItemInputAttributeMap,
    #[serde(rename = "Expected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<ExpectedAttributeMap>,
    #[serde(rename = "ReturnValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values: Option<ReturnValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
    #[serde(rename = "ConditionalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<ConditionalOperator>,
    #[serde(rename = "ConditionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutItemOutput {
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributeMap>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "Select")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<Select>,
    #[serde(rename = "AttributesToGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<AttributeNameList>,
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<PositiveIntegerObject>,
    #[serde(rename = "ConsistentRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<ConsistentRead>,
    #[serde(rename = "KeyConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_conditions: Option<KeyConditions>,
    #[serde(rename = "QueryFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_filter: Option<FilterConditionMap>,
    #[serde(rename = "ConditionalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<ConditionalOperator>,
    #[serde(rename = "ScanIndexForward")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_index_forward: Option<BooleanObject>,
    #[serde(rename = "ExclusiveStartKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_key: Option<Key>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ProjectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<ProjectionExpression>,
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<ConditionExpression>,
    #[serde(rename = "KeyConditionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_condition_expression: Option<KeyExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueryOutput {
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ItemList>,
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<Integer>,
    #[serde(rename = "ScannedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_count: Option<Integer>,
    #[serde(rename = "LastEvaluatedKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_key: Option<Key>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Replica {
    #[serde(rename = "RegionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<RegionName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaAlreadyExistsException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaAutoScalingDescription {
    #[serde(rename = "RegionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<RegionName>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<ReplicaGlobalSecondaryIndexAutoScalingDescriptionList>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaProvisionedWriteCapacityAutoScalingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_write_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status: Option<ReplicaStatus>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaAutoScalingUpdate {
    #[serde(rename = "RegionName")]
    
    pub region_name: RegionName,
    #[serde(rename = "ReplicaGlobalSecondaryIndexUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_global_secondary_index_updates: Option<ReplicaGlobalSecondaryIndexAutoScalingUpdateList>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaDescription {
    #[serde(rename = "RegionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<RegionName>,
    #[serde(rename = "ReplicaStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status: Option<ReplicaStatus>,
    #[serde(rename = "ReplicaStatusDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status_description: Option<ReplicaStatusDescription>,
    #[serde(rename = "ReplicaStatusPercentProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status_percent_progress: Option<ReplicaStatusPercentProgress>,
    #[serde(rename = "KMSMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<KMSMasterKeyId>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<ReplicaGlobalSecondaryIndexDescriptionList>,
    #[serde(rename = "ReplicaInaccessibleDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_inaccessible_date_time: Option<Date>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndex {
    #[serde(rename = "IndexName")]
    
    pub index_name: IndexName,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndexAutoScalingDescription {
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "IndexStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<IndexStatus>,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndexAutoScalingUpdate {
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndexDescription {
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndexSettingsDescription {
    #[serde(rename = "IndexName")]
    
    pub index_name: IndexName,
    #[serde(rename = "IndexStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<IndexStatus>,
    #[serde(rename = "ProvisionedReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ProvisionedWriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaGlobalSecondaryIndexSettingsUpdate {
    #[serde(rename = "IndexName")]
    
    pub index_name: IndexName,
    #[serde(rename = "ProvisionedReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "ProvisionedReadCapacityAutoScalingSettingsUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_read_capacity_auto_scaling_settings_update: Option<AutoScalingSettingsUpdate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaNotFoundException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaSettingsDescription {
    #[serde(rename = "RegionName")]
    
    pub region_name: RegionName,
    #[serde(rename = "ReplicaStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status: Option<ReplicaStatus>,
    #[serde(rename = "ReplicaBillingModeSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_billing_mode_summary: Option<BillingModeSummary>,
    #[serde(rename = "ReplicaProvisionedReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_units: Option<NonNegativeLongObject>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaProvisionedWriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_write_capacity_units: Option<NonNegativeLongObject>,
    #[serde(rename = "ReplicaProvisionedWriteCapacityAutoScalingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_write_capacity_auto_scaling_settings: Option<AutoScalingSettingsDescription>,
    #[serde(rename = "ReplicaGlobalSecondaryIndexSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_global_secondary_index_settings: Option<ReplicaGlobalSecondaryIndexSettingsDescriptionList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaSettingsUpdate {
    #[serde(rename = "RegionName")]
    
    pub region_name: RegionName,
    #[serde(rename = "ReplicaProvisionedReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "ReplicaProvisionedReadCapacityAutoScalingSettingsUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_provisioned_read_capacity_auto_scaling_settings_update: Option<AutoScalingSettingsUpdate>,
    #[serde(rename = "ReplicaGlobalSecondaryIndexSettingsUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_global_secondary_index_settings_update: Option<ReplicaGlobalSecondaryIndexSettingsUpdateList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaUpdate {
    #[serde(rename = "Create")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<CreateReplicaAction>,
    #[serde(rename = "Delete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<DeleteReplicaAction>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicationGroupUpdate {
    #[serde(rename = "Create")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<CreateReplicationGroupMemberAction>,
    #[serde(rename = "Update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateReplicationGroupMemberAction>,
    #[serde(rename = "Delete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<DeleteReplicationGroupMemberAction>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestLimitExceeded {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceInUseException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceNotFoundException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreSummary {
    #[serde(rename = "SourceBackupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_arn: Option<BackupArn>,
    #[serde(rename = "SourceTableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode_override: Option<BillingMode>,
    #[serde(rename = "GlobalSecondaryIndexOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_index_override: Option<GlobalSecondaryIndexList>,
    #[serde(rename = "LocalSecondaryIndexOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_index_override: Option<LocalSecondaryIndexList>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughput>,
    #[serde(rename = "SSESpecificationOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_specification_override: Option<SSESpecification>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreTableFromBackupOutput {
    #[serde(rename = "TableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreTableToPointInTimeInput {
    #[serde(rename = "SourceTableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_arn: Option<TableArn>,
    #[serde(rename = "SourceTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_name: Option<TableName>,
    #[serde(rename = "TargetTableName")]
    
    pub target_table_name: TableName,
    #[serde(rename = "UseLatestRestorableTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_time: Option<BooleanObject>,
    #[serde(rename = "RestoreDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_date_time: Option<Date>,
    #[serde(rename = "BillingModeOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode_override: Option<BillingMode>,
    #[serde(rename = "GlobalSecondaryIndexOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_index_override: Option<GlobalSecondaryIndexList>,
    #[serde(rename = "LocalSecondaryIndexOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_index_override: Option<LocalSecondaryIndexList>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughput>,
    #[serde(rename = "SSESpecificationOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_specification_override: Option<SSESpecification>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreTableToPointInTimeOutput {
    #[serde(rename = "TableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SSEDescription {
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SSEStatus>,
    #[serde(rename = "SSEType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_type: Option<SSEType>,
    #[serde(rename = "KMSMasterKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_arn: Option<KMSMasterKeyArn>,
    #[serde(rename = "InaccessibleEncryptionDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inaccessible_encryption_date_time: Option<Date>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SSESpecification {
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<SSEEnabled>,
    #[serde(rename = "SSEType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_type: Option<SSEType>,
    #[serde(rename = "KMSMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<KMSMasterKeyId>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScanInput {
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "AttributesToGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<AttributeNameList>,
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<PositiveIntegerObject>,
    #[serde(rename = "Select")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<Select>,
    #[serde(rename = "ScanFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_filter: Option<FilterConditionMap>,
    #[serde(rename = "ConditionalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<ConditionalOperator>,
    #[serde(rename = "ExclusiveStartKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_key: Option<Key>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "TotalSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_segments: Option<ScanTotalSegments>,
    #[serde(rename = "Segment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<ScanSegment>,
    #[serde(rename = "ProjectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<ProjectionExpression>,
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
    #[serde(rename = "ConsistentRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<ConsistentRead>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScanOutput {
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ItemList>,
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<Integer>,
    #[serde(rename = "ScannedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_count: Option<Integer>,
    #[serde(rename = "LastEvaluatedKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_key: Option<Key>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SourceTableDetails {
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "TableId")]
    
    pub table_id: TableId,
    #[serde(rename = "TableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<TableArn>,
    #[serde(rename = "TableSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_size_bytes: Option<Long>,
    #[serde(rename = "KeySchema")]
    
    pub key_schema: KeySchema,
    #[serde(rename = "TableCreationDateTime")]
    
    pub table_creation_date_time: TableCreationDateTime,
    #[serde(rename = "ProvisionedThroughput")]
    
    pub provisioned_throughput: ProvisionedThroughput,
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<ItemCount>,
    #[serde(rename = "BillingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<BillingMode>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SourceTableFeatureDetails {
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<LocalSecondaryIndexes>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<GlobalSecondaryIndexes>,
    #[serde(rename = "StreamDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_description: Option<StreamSpecification>,
    #[serde(rename = "TimeToLiveDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_description: Option<TimeToLiveDescription>,
    #[serde(rename = "SSEDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_description: Option<SSEDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StreamSpecification {
    #[serde(rename = "StreamEnabled")]
    
    pub stream_enabled: StreamEnabled,
    #[serde(rename = "StreamViewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view_type: Option<StreamViewType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TableAlreadyExistsException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TableAutoScalingDescription {
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "TableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_status: Option<TableStatus>,
    #[serde(rename = "Replicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<ReplicaAutoScalingDescriptionList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TableDescription {
    #[serde(rename = "AttributeDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definitions: Option<AttributeDefinitions>,
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<KeySchema>,
    #[serde(rename = "TableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_status: Option<TableStatus>,
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<Date>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughputDescription>,
    #[serde(rename = "TableSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_size_bytes: Option<Long>,
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<Long>,
    #[serde(rename = "TableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "TableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<TableId>,
    #[serde(rename = "BillingModeSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode_summary: Option<BillingModeSummary>,
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<LocalSecondaryIndexDescriptionList>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<GlobalSecondaryIndexDescriptionList>,
    #[serde(rename = "StreamSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,
    #[serde(rename = "LatestStreamLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_stream_label: Option<String>,
    #[serde(rename = "LatestStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_stream_arn: Option<StreamArn>,
    #[serde(rename = "GlobalTableVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_version: Option<String>,
    #[serde(rename = "Replicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<ReplicaDescriptionList>,
    #[serde(rename = "RestoreSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_summary: Option<RestoreSummary>,
    #[serde(rename = "SSEDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_description: Option<SSEDescription>,
    #[serde(rename = "ArchivalSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival_summary: Option<ArchivalSummary>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TableInUseException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TableNotFoundException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_status: Option<TimeToLiveStatus>,
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactGetItemsOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacityMultiple>,
    #[serde(rename = "Responses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<ItemResponseList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactWriteItem {
    #[serde(rename = "ConditionCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_check: Option<ConditionCheck>,
    #[serde(rename = "Put")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Put>,
    #[serde(rename = "Delete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Delete>,
    #[serde(rename = "Update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactWriteItemsInput {
    #[serde(rename = "TransactItems")]
    
    pub transact_items: TransactWriteItemList,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<ClientRequestToken>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactWriteItemsOutput {
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacityMultiple>,
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics: Option<ItemCollectionMetricsPerTable>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactionCanceledException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
    #[serde(rename = "CancellationReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reasons: Option<CancellationReasonList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactionConflictException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactionInProgressException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
    #[serde(rename = "ReturnValuesOnConditionCheckFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_backups_description: Option<ContinuousBackupsDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateContributorInsightsInput {
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ContributorInsightsAction")]
    
    pub contributor_insights_action: ContributorInsightsAction,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateContributorInsightsOutput {
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,
    #[serde(rename = "ContributorInsightsStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_description: Option<GlobalTableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateGlobalTableSettingsInput {
    #[serde(rename = "GlobalTableName")]
    
    pub global_table_name: TableName,
    #[serde(rename = "GlobalTableBillingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_billing_mode: Option<BillingMode>,
    #[serde(rename = "GlobalTableProvisionedWriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_provisioned_write_capacity_units: Option<PositiveLongObject>,
    #[serde(rename = "GlobalTableProvisionedWriteCapacityAutoScalingSettingsUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_provisioned_write_capacity_auto_scaling_settings_update: Option<AutoScalingSettingsUpdate>,
    #[serde(rename = "GlobalTableGlobalSecondaryIndexSettingsUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_global_secondary_index_settings_update: Option<GlobalTableGlobalSecondaryIndexSettingsUpdateList>,
    #[serde(rename = "ReplicaSettingsUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_settings_update: Option<ReplicaSettingsUpdateList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateGlobalTableSettingsOutput {
    #[serde(rename = "GlobalTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_name: Option<TableName>,
    #[serde(rename = "ReplicaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_settings: Option<ReplicaSettingsDescriptionList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateItemInput {
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "Key")]
    
    pub key: Key,
    #[serde(rename = "AttributeUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_updates: Option<AttributeUpdates>,
    #[serde(rename = "Expected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<ExpectedAttributeMap>,
    #[serde(rename = "ConditionalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<ConditionalOperator>,
    #[serde(rename = "ReturnValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values: Option<ReturnValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
    #[serde(rename = "UpdateExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_expression: Option<UpdateExpression>,
    #[serde(rename = "ConditionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<ConditionExpression>,
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateItemOutput {
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributeMap>,
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics: Option<ItemCollectionMetrics>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateReplicationGroupMemberAction {
    #[serde(rename = "RegionName")]
    
    pub region_name: RegionName,
    #[serde(rename = "KMSMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<KMSMasterKeyId>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<ProvisionedThroughputOverride>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<ReplicaGlobalSecondaryIndexList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateTableInput {
    #[serde(rename = "AttributeDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definitions: Option<AttributeDefinitions>,
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "BillingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<BillingMode>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "GlobalSecondaryIndexUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_index_updates: Option<GlobalSecondaryIndexUpdateList>,
    #[serde(rename = "StreamSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,
    #[serde(rename = "SSESpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_specification: Option<SSESpecification>,
    #[serde(rename = "ReplicaUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_updates: Option<ReplicationGroupUpdateList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateTableOutput {
    #[serde(rename = "TableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateTableReplicaAutoScalingInput {
    #[serde(rename = "GlobalSecondaryIndexUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_index_updates: Option<GlobalSecondaryIndexAutoScalingUpdateList>,
    #[serde(rename = "TableName")]
    
    pub table_name: TableName,
    #[serde(rename = "ProvisionedWriteCapacityAutoScalingUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_write_capacity_auto_scaling_update: Option<AutoScalingSettingsUpdate>,
    #[serde(rename = "ReplicaUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_updates: Option<ReplicaAutoScalingUpdateList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateTableReplicaAutoScalingOutput {
    #[serde(rename = "TableAutoScalingDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_specification: Option<TimeToLiveSpecification>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WriteRequest {
    #[serde(rename = "PutRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_request: Option<PutRequest>,
    #[serde(rename = "DeleteRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_request: Option<DeleteRequest>,
}

