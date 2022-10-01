// This file is generated!
// See https://github.com/akkoro/asml-aws-codegen

use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub type Action = String;
pub type AdditionalVersion = String;
pub type AdditionalVersionWeights = HashMap<AdditionalVersion, Weight>;
pub type Alias = String;
pub type AliasList = Vec<AliasConfiguration>;
pub type AllowCredentials = bool;
pub type AllowMethodsList = Vec<Method>;
pub type AllowOriginsList = Vec<Origin>;
pub type Architecture = String;
pub type ArchitecturesList = Vec<Architecture>;
pub type Arn = String;
pub type BatchSize = i64;
pub type BisectBatchOnFunctionError = bool;
pub type Blob = Vec<u8>;
pub type BlobStream = Vec<u8>;
pub type Boolean = bool;
pub type CodeSigningConfigArn = String;
pub type CodeSigningConfigId = String;
pub type CodeSigningConfigList = Vec<CodeSigningConfig>;
pub type CodeSigningPolicy = String;
pub type CompatibleArchitectures = Vec<Architecture>;
pub type CompatibleRuntimes = Vec<Runtime>;
pub type Date = String;
pub type Description = String;
pub type DestinationArn = String;
pub type Enabled = bool;
pub type EndPointType = String;
pub type Endpoint = String;
pub type EndpointLists = Vec<Endpoint>;
pub type Endpoints = HashMap<EndPointType, EndpointLists>;
pub type EnvironmentVariableName = String;
pub type EnvironmentVariableValue = String;
pub type EnvironmentVariables = HashMap<EnvironmentVariableName, EnvironmentVariableValue>;
pub type EphemeralStorageSize = i64;
pub type EventSourceMappingsList = Vec<EventSourceMappingConfiguration>;
pub type EventSourcePosition = String;
pub type EventSourceToken = String;
pub type FileSystemArn = String;
pub type FileSystemConfigList = Vec<FileSystemConfig>;
pub type FilterList = Vec<Filter>;
pub type FunctionArn = String;
pub type FunctionArnList = Vec<FunctionArn>;
pub type FunctionEventInvokeConfigList = Vec<FunctionEventInvokeConfig>;
pub type FunctionList = Vec<FunctionConfiguration>;
pub type FunctionName = String;
pub type FunctionResponseType = String;
pub type FunctionResponseTypeList = Vec<FunctionResponseType>;
pub type FunctionUrl = String;
pub type FunctionUrlAuthType = String;
pub type FunctionUrlConfigList = Vec<FunctionUrlConfig>;
pub type FunctionUrlQualifier = String;
pub type FunctionVersion = String;
pub type Handler = String;
pub type Header = String;
pub type HeadersList = Vec<Header>;
pub type HttpStatus = i64;
pub type Integer = i64;
pub type InvocationType = String;
pub type KMSKeyArn = String;
pub type LastUpdateStatus = String;
pub type LastUpdateStatusReason = String;
pub type LastUpdateStatusReasonCode = String;
pub type LayerArn = String;
pub type LayerList = Vec<LayerVersionArn>;
pub type LayerName = String;
pub type LayerPermissionAllowedAction = String;
pub type LayerPermissionAllowedPrincipal = String;
pub type LayerVersionArn = String;
pub type LayerVersionNumber = u64;
pub type LayerVersionsList = Vec<LayerVersionsListItem>;
pub type LayersList = Vec<LayersListItem>;
pub type LayersReferenceList = Vec<Layer>;
pub type LicenseInfo = String;
pub type LocalMountPath = String;
pub type LogType = String;
pub type Long = u64;
pub type MasterRegion = String;
pub type MaxAge = i64;
pub type MaxFunctionEventInvokeConfigListItems = i64;
pub type MaxItems = i64;
pub type MaxLayerListItems = i64;
pub type MaxListItems = i64;
pub type MaxProvisionedConcurrencyConfigListItems = i64;
pub type MaximumBatchingWindowInSeconds = i64;
pub type MaximumEventAgeInSeconds = i64;
pub type MaximumRecordAgeInSeconds = i64;
pub type MaximumRetryAttempts = i64;
pub type MaximumRetryAttemptsEventSourceMapping = i64;
pub type MemorySize = i64;
pub type Method = String;
pub type NameSpacedFunctionArn = String;
pub type NamespacedFunctionName = String;
pub type NamespacedStatementId = String;
pub type NonNegativeInteger = i64;
pub type OrganizationId = String;
pub type Origin = String;
pub type PackageType = String;
pub type ParallelizationFactor = i64;
pub type Pattern = String;
pub type PositiveInteger = i64;
pub type Principal = String;
pub type PrincipalOrgID = String;
pub type ProvisionedConcurrencyConfigList = Vec<ProvisionedConcurrencyConfigListItem>;
pub type ProvisionedConcurrencyStatusEnum = String;
pub type Qualifier = String;
pub type Queue = String;
pub type Queues = Vec<Queue>;
pub type ReservedConcurrentExecutions = i64;
pub type ResourceArn = String;
pub type RoleArn = String;
pub type Runtime = String;
pub type S3Bucket = String;
pub type S3Key = String;
pub type S3ObjectVersion = String;
pub type SecurityGroupId = String;
pub type SecurityGroupIds = Vec<SecurityGroupId>;
pub type SensitiveString = String;
pub type SigningProfileVersionArns = Vec<Arn>;
pub type SourceAccessConfigurations = Vec<SourceAccessConfiguration>;
pub type SourceAccessType = String;
pub type SourceOwner = String;
pub type State = String;
pub type StateReason = String;
pub type StateReasonCode = String;
pub type StatementId = String;
pub type StringList = Vec<String>;
pub type SubnetId = String;
pub type SubnetIds = Vec<SubnetId>;
pub type TagKey = String;
pub type TagKeyList = Vec<TagKey>;
pub type TagValue = String;
pub type Tags = HashMap<TagKey, TagValue>;
pub type ThrottleReason = String;
pub type Timeout = i64;
pub type Timestamp = String;
pub type Topic = String;
pub type Topics = Vec<Topic>;
pub type TracingMode = String;
pub type TumblingWindowInSeconds = i64;
pub type URI = String;
pub type UnreservedConcurrentExecutions = i64;
pub type Version = String;
pub type VpcId = String;
pub type Weight = f64;
pub type WorkingDirectory = String;
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountLimit {
    #[serde(rename = "TotalCodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_code_size: Option<Long>,
    #[serde(rename = "CodeSizeUnzipped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size_unzipped: Option<Long>,
    #[serde(rename = "CodeSizeZipped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size_zipped: Option<Long>,
    #[serde(rename = "ConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_executions: Option<Integer>,
    #[serde(rename = "UnreservedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreserved_concurrent_executions: Option<UnreservedConcurrentExecutions>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountUsage {
    #[serde(rename = "TotalCodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_code_size: Option<Long>,
    #[serde(rename = "FunctionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_count: Option<Long>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AddLayerVersionPermissionRequest {
    #[serde(rename = "LayerName")]

    pub layer_name: LayerName,
    #[serde(rename = "VersionNumber")]

    pub version_number: LayerVersionNumber,
    #[serde(rename = "StatementId")]

    pub statement_id: StatementId,
    #[serde(rename = "Action")]

    pub action: LayerPermissionAllowedAction,
    #[serde(rename = "Principal")]

    pub principal: LayerPermissionAllowedPrincipal,
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<OrganizationId>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AddLayerVersionPermissionResponse {
    #[serde(rename = "Statement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AddPermissionRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "StatementId")]

    pub statement_id: StatementId,
    #[serde(rename = "Action")]

    pub action: Action,
    #[serde(rename = "Principal")]

    pub principal: Principal,
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<Arn>,
    #[serde(rename = "SourceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<SourceOwner>,
    #[serde(rename = "EventSourceToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_token: Option<EventSourceToken>,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "PrincipalOrgID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_org_id: Option<PrincipalOrgID>,
    #[serde(rename = "FunctionUrlAuthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_url_auth_type: Option<FunctionUrlAuthType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AddPermissionResponse {
    #[serde(rename = "Statement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AliasConfiguration {
    #[serde(rename = "AliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<FunctionArn>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Alias>,
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<Version>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AliasRoutingConfiguration {
    #[serde(rename = "AdditionalVersionWeights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_version_weights: Option<AdditionalVersionWeights>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AllowedPublishers {
    #[serde(rename = "SigningProfileVersionArns")]

    pub signing_profile_version_arns: SigningProfileVersionArns,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AmazonManagedKafkaEventSourceConfig {
    #[serde(rename = "ConsumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<URI>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CodeSigningConfig {
    #[serde(rename = "CodeSigningConfigId")]

    pub code_signing_config_id: CodeSigningConfigId,
    #[serde(rename = "CodeSigningConfigArn")]

    pub code_signing_config_arn: CodeSigningConfigArn,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "AllowedPublishers")]

    pub allowed_publishers: AllowedPublishers,
    #[serde(rename = "CodeSigningPolicies")]

    pub code_signing_policies: CodeSigningPolicies,
    #[serde(rename = "LastModified")]

    pub last_modified: Timestamp,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CodeSigningConfigNotFoundException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CodeSigningPolicies {
    #[serde(rename = "UntrustedArtifactOnDeployment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untrusted_artifact_on_deployment: Option<CodeSigningPolicy>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CodeStorageExceededException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CodeVerificationFailedException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Concurrency {
    #[serde(rename = "ReservedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_concurrent_executions: Option<ReservedConcurrentExecutions>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Cors {
    #[serde(rename = "AllowCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<AllowCredentials>,
    #[serde(rename = "AllowHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_headers: Option<HeadersList>,
    #[serde(rename = "AllowMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_methods: Option<AllowMethodsList>,
    #[serde(rename = "AllowOrigins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_origins: Option<AllowOriginsList>,
    #[serde(rename = "ExposeHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<HeadersList>,
    #[serde(rename = "MaxAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<MaxAge>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateAliasRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Name")]

    pub name: Alias,
    #[serde(rename = "FunctionVersion")]

    pub function_version: Version,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateCodeSigningConfigRequest {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "AllowedPublishers")]

    pub allowed_publishers: AllowedPublishers,
    #[serde(rename = "CodeSigningPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_policies: Option<CodeSigningPolicies>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateCodeSigningConfigResponse {
    #[serde(rename = "CodeSigningConfig")]

    pub code_signing_config: CodeSigningConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateEventSourceMappingRequest {
    #[serde(rename = "EventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<Arn>,
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<BatchSize>,
    #[serde(rename = "FilterCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<MaximumBatchingWindowInSeconds>,
    #[serde(rename = "ParallelizationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<ParallelizationFactor>,
    #[serde(rename = "StartingPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<EventSourcePosition>,
    #[serde(rename = "StartingPositionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position_timestamp: Option<Date>,
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<MaximumRecordAgeInSeconds>,
    #[serde(rename = "BisectBatchOnFunctionError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisect_batch_on_function_error: Option<BisectBatchOnFunctionError>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<MaximumRetryAttemptsEventSourceMapping>,
    #[serde(rename = "TumblingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumbling_window_in_seconds: Option<TumblingWindowInSeconds>,
    #[serde(rename = "Topics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Topics>,
    #[serde(rename = "Queues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Queues>,
    #[serde(rename = "SourceAccessConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_access_configurations: Option<SourceAccessConfigurations>,
    #[serde(rename = "SelfManagedEventSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_event_source: Option<SelfManagedEventSource>,
    #[serde(rename = "FunctionResponseTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response_types: Option<FunctionResponseTypeList>,
    #[serde(rename = "AmazonManagedKafkaEventSourceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_managed_kafka_event_source_config: Option<AmazonManagedKafkaEventSourceConfig>,
    #[serde(rename = "SelfManagedKafkaEventSourceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_kafka_event_source_config: Option<SelfManagedKafkaEventSourceConfig>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateFunctionRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<Runtime>,
    #[serde(rename = "Role")]

    pub role: RoleArn,
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<Handler>,
    #[serde(rename = "Code")]

    pub code: FunctionCode,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Timeout>,
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<MemorySize>,
    #[serde(rename = "Publish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<Boolean>,
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
    #[serde(rename = "PackageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<PackageType>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<KMSKeyArn>,
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfig>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<LayerList>,
    #[serde(rename = "FileSystemConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_configs: Option<FileSystemConfigList>,
    #[serde(rename = "ImageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageConfig>,
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_config_arn: Option<CodeSigningConfigArn>,
    #[serde(rename = "Architectures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<ArchitecturesList>,
    #[serde(rename = "EphemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateFunctionUrlConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<FunctionUrlQualifier>,
    #[serde(rename = "AuthType")]

    pub auth_type: FunctionUrlAuthType,
    #[serde(rename = "Cors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateFunctionUrlConfigResponse {
    #[serde(rename = "FunctionUrl")]

    pub function_url: FunctionUrl,
    #[serde(rename = "FunctionArn")]

    pub function_arn: FunctionArn,
    #[serde(rename = "AuthType")]

    pub auth_type: FunctionUrlAuthType,
    #[serde(rename = "Cors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
    #[serde(rename = "CreationTime")]

    pub creation_time: Timestamp,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeadLetterConfig {
    #[serde(rename = "TargetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<ResourceArn>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteAliasRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Name")]

    pub name: Alias,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteCodeSigningConfigRequest {
    #[serde(rename = "CodeSigningConfigArn")]

    pub code_signing_config_arn: CodeSigningConfigArn,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteCodeSigningConfigResponse {
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteEventSourceMappingRequest {
    #[serde(rename = "UUID")]

    pub uuid: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteFunctionCodeSigningConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteFunctionConcurrencyRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteFunctionEventInvokeConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteFunctionRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteFunctionUrlConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<FunctionUrlQualifier>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteLayerVersionRequest {
    #[serde(rename = "LayerName")]

    pub layer_name: LayerName,
    #[serde(rename = "VersionNumber")]

    pub version_number: LayerVersionNumber,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteProvisionedConcurrencyConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]

    pub qualifier: Qualifier,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DestinationConfig {
    #[serde(rename = "OnSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_success: Option<OnSuccess>,
    #[serde(rename = "OnFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<OnFailure>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EC2AccessDeniedException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EC2ThrottledException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EC2UnexpectedException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "EC2ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_error_code: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EFSIOException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EFSMountConnectivityException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EFSMountFailureException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EFSMountTimeoutException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ENILimitReachedException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Environment {
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<EnvironmentVariables>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EnvironmentError {
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<SensitiveString>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EnvironmentResponse {
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<EnvironmentVariables>,
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EnvironmentError>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EphemeralStorage {
    #[serde(rename = "Size")]

    pub size: EphemeralStorageSize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EventSourceMappingConfiguration {
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "StartingPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<EventSourcePosition>,
    #[serde(rename = "StartingPositionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position_timestamp: Option<Date>,
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<BatchSize>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<MaximumBatchingWindowInSeconds>,
    #[serde(rename = "ParallelizationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<ParallelizationFactor>,
    #[serde(rename = "EventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<Arn>,
    #[serde(rename = "FilterCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<FunctionArn>,
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<Date>,
    #[serde(rename = "LastProcessingResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_processing_result: Option<String>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateTransitionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_transition_reason: Option<String>,
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    #[serde(rename = "Topics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Topics>,
    #[serde(rename = "Queues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Queues>,
    #[serde(rename = "SourceAccessConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_access_configurations: Option<SourceAccessConfigurations>,
    #[serde(rename = "SelfManagedEventSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_event_source: Option<SelfManagedEventSource>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<MaximumRecordAgeInSeconds>,
    #[serde(rename = "BisectBatchOnFunctionError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisect_batch_on_function_error: Option<BisectBatchOnFunctionError>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<MaximumRetryAttemptsEventSourceMapping>,
    #[serde(rename = "TumblingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumbling_window_in_seconds: Option<TumblingWindowInSeconds>,
    #[serde(rename = "FunctionResponseTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response_types: Option<FunctionResponseTypeList>,
    #[serde(rename = "AmazonManagedKafkaEventSourceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_managed_kafka_event_source_config: Option<AmazonManagedKafkaEventSourceConfig>,
    #[serde(rename = "SelfManagedKafkaEventSourceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_kafka_event_source_config: Option<SelfManagedKafkaEventSourceConfig>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FileSystemConfig {
    #[serde(rename = "Arn")]

    pub arn: FileSystemArn,
    #[serde(rename = "LocalMountPath")]

    pub local_mount_path: LocalMountPath,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "Pattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Pattern>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FilterCriteria {
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FunctionCode {
    #[serde(rename = "ZipFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<Blob>,
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<S3Bucket>,
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<S3Key>,
    #[serde(rename = "S3ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<S3ObjectVersion>,
    #[serde(rename = "ImageUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FunctionCodeLocation {
    #[serde(rename = "RepositoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_type: Option<String>,
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "ImageUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    #[serde(rename = "ResolvedImageUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_image_uri: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FunctionConfiguration {
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<NamespacedFunctionName>,
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<NameSpacedFunctionArn>,
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<Runtime>,
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleArn>,
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<Handler>,
    #[serde(rename = "CodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<Long>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Timeout>,
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<MemorySize>,
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<Timestamp>,
    #[serde(rename = "CodeSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha256: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfigResponse>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<EnvironmentResponse>,
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<KMSKeyArn>,
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfigResponse>,
    #[serde(rename = "MasterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_arn: Option<FunctionArn>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<LayersReferenceList>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "StateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<StateReason>,
    #[serde(rename = "StateReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason_code: Option<StateReasonCode>,
    #[serde(rename = "LastUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status: Option<LastUpdateStatus>,
    #[serde(rename = "LastUpdateStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status_reason: Option<LastUpdateStatusReason>,
    #[serde(rename = "LastUpdateStatusReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status_reason_code: Option<LastUpdateStatusReasonCode>,
    #[serde(rename = "FileSystemConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_configs: Option<FileSystemConfigList>,
    #[serde(rename = "PackageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<PackageType>,
    #[serde(rename = "ImageConfigResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config_response: Option<ImageConfigResponse>,
    #[serde(rename = "SigningProfileVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_version_arn: Option<Arn>,
    #[serde(rename = "SigningJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_job_arn: Option<Arn>,
    #[serde(rename = "Architectures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<ArchitecturesList>,
    #[serde(rename = "EphemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FunctionEventInvokeConfig {
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<Date>,
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<FunctionArn>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<MaximumRetryAttempts>,
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<MaximumEventAgeInSeconds>,
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FunctionUrlConfig {
    #[serde(rename = "FunctionUrl")]

    pub function_url: FunctionUrl,
    #[serde(rename = "FunctionArn")]

    pub function_arn: FunctionArn,
    #[serde(rename = "CreationTime")]

    pub creation_time: Timestamp,
    #[serde(rename = "LastModifiedTime")]

    pub last_modified_time: Timestamp,
    #[serde(rename = "Cors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
    #[serde(rename = "AuthType")]

    pub auth_type: FunctionUrlAuthType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetAccountSettingsRequest {
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetAccountSettingsResponse {
    #[serde(rename = "AccountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_limit: Option<AccountLimit>,
    #[serde(rename = "AccountUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_usage: Option<AccountUsage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetAliasRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Name")]

    pub name: Alias,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetCodeSigningConfigRequest {
    #[serde(rename = "CodeSigningConfigArn")]

    pub code_signing_config_arn: CodeSigningConfigArn,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetCodeSigningConfigResponse {
    #[serde(rename = "CodeSigningConfig")]

    pub code_signing_config: CodeSigningConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetEventSourceMappingRequest {
    #[serde(rename = "UUID")]

    pub uuid: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetFunctionCodeSigningConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetFunctionCodeSigningConfigResponse {
    #[serde(rename = "CodeSigningConfigArn")]

    pub code_signing_config_arn: CodeSigningConfigArn,
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetFunctionConcurrencyRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetFunctionConcurrencyResponse {
    #[serde(rename = "ReservedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_concurrent_executions: Option<ReservedConcurrentExecutions>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetFunctionConfigurationRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: NamespacedFunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetFunctionEventInvokeConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetFunctionRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: NamespacedFunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetFunctionResponse {
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<FunctionConfiguration>,
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<FunctionCodeLocation>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "Concurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetFunctionUrlConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<FunctionUrlQualifier>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetFunctionUrlConfigResponse {
    #[serde(rename = "FunctionUrl")]

    pub function_url: FunctionUrl,
    #[serde(rename = "FunctionArn")]

    pub function_arn: FunctionArn,
    #[serde(rename = "AuthType")]

    pub auth_type: FunctionUrlAuthType,
    #[serde(rename = "Cors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
    #[serde(rename = "CreationTime")]

    pub creation_time: Timestamp,
    #[serde(rename = "LastModifiedTime")]

    pub last_modified_time: Timestamp,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetLayerVersionByArnRequest {
    #[serde(rename = "Arn")]

    pub arn: LayerVersionArn,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetLayerVersionPolicyRequest {
    #[serde(rename = "LayerName")]

    pub layer_name: LayerName,
    #[serde(rename = "VersionNumber")]

    pub version_number: LayerVersionNumber,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetLayerVersionPolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetLayerVersionRequest {
    #[serde(rename = "LayerName")]

    pub layer_name: LayerName,
    #[serde(rename = "VersionNumber")]

    pub version_number: LayerVersionNumber,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetLayerVersionResponse {
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<LayerVersionContentOutput>,
    #[serde(rename = "LayerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_arn: Option<LayerArn>,
    #[serde(rename = "LayerVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_version_arn: Option<LayerVersionArn>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<Timestamp>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<LayerVersionNumber>,
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<CompatibleRuntimes>,
    #[serde(rename = "LicenseInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<LicenseInfo>,
    #[serde(rename = "CompatibleArchitectures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architectures: Option<CompatibleArchitectures>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetPolicyRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: NamespacedFunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetPolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetProvisionedConcurrencyConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]

    pub qualifier: Qualifier,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetProvisionedConcurrencyConfigResponse {
    #[serde(rename = "RequestedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_provisioned_concurrent_executions: Option<PositiveInteger>,
    #[serde(rename = "AvailableProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_provisioned_concurrent_executions: Option<NonNegativeInteger>,
    #[serde(rename = "AllocatedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_provisioned_concurrent_executions: Option<NonNegativeInteger>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProvisionedConcurrencyStatusEnum>,
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<Timestamp>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ImageConfig {
    #[serde(rename = "EntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<StringList>,
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<StringList>,
    #[serde(rename = "WorkingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<WorkingDirectory>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ImageConfigError {
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<SensitiveString>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ImageConfigResponse {
    #[serde(rename = "ImageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageConfig>,
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ImageConfigError>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidCodeSignatureException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidParameterValueException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidRequestContentException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidRuntimeException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidSecurityGroupIDException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidSubnetIDException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidZipFileException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvocationRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: NamespacedFunctionName,
    #[serde(rename = "InvocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_type: Option<InvocationType>,
    #[serde(rename = "LogType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<LogType>,
    #[serde(rename = "ClientContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_context: Option<String>,
    #[serde(rename = "Payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Blob>,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvocationResponse {
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<Integer>,
    #[serde(rename = "FunctionError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_error: Option<String>,
    #[serde(rename = "LogResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_result: Option<String>,
    #[serde(rename = "Payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Blob>,
    #[serde(rename = "ExecutedVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_version: Option<Version>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvokeAsyncRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: NamespacedFunctionName,
    #[serde(rename = "InvokeArgs")]

    pub invoke_args: BlobStream,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvokeAsyncResponse {
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<HttpStatus>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KMSAccessDeniedException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KMSDisabledException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KMSInvalidStateException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KMSNotFoundException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Layer {
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<LayerVersionArn>,
    #[serde(rename = "CodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<Long>,
    #[serde(rename = "SigningProfileVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_version_arn: Option<Arn>,
    #[serde(rename = "SigningJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_job_arn: Option<Arn>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerVersionContentInput {
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<S3Bucket>,
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<S3Key>,
    #[serde(rename = "S3ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<S3ObjectVersion>,
    #[serde(rename = "ZipFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<Blob>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerVersionContentOutput {
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "CodeSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha256: Option<String>,
    #[serde(rename = "CodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<Long>,
    #[serde(rename = "SigningProfileVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_version_arn: Option<String>,
    #[serde(rename = "SigningJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_job_arn: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerVersionsListItem {
    #[serde(rename = "LayerVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_version_arn: Option<LayerVersionArn>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<LayerVersionNumber>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<Timestamp>,
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<CompatibleRuntimes>,
    #[serde(rename = "LicenseInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<LicenseInfo>,
    #[serde(rename = "CompatibleArchitectures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architectures: Option<CompatibleArchitectures>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayersListItem {
    #[serde(rename = "LayerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_name: Option<LayerName>,
    #[serde(rename = "LayerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_arn: Option<LayerArn>,
    #[serde(rename = "LatestMatchingVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_matching_version: Option<LayerVersionsListItem>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListAliasesRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<Version>,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<MaxListItems>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListAliasesResponse {
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<AliasList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListCodeSigningConfigsRequest {
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<MaxListItems>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListCodeSigningConfigsResponse {
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "CodeSigningConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_configs: Option<CodeSigningConfigList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListEventSourceMappingsRequest {
    #[serde(rename = "EventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<Arn>,
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<FunctionName>,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<MaxListItems>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListEventSourceMappingsResponse {
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "EventSourceMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_mappings: Option<EventSourceMappingsList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListFunctionEventInvokeConfigsRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<MaxFunctionEventInvokeConfigListItems>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListFunctionEventInvokeConfigsResponse {
    #[serde(rename = "FunctionEventInvokeConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_event_invoke_configs: Option<FunctionEventInvokeConfigList>,
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListFunctionUrlConfigsRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<MaxItems>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListFunctionUrlConfigsResponse {
    #[serde(rename = "FunctionUrlConfigs")]

    pub function_url_configs: FunctionUrlConfigList,
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListFunctionsByCodeSigningConfigRequest {
    #[serde(rename = "CodeSigningConfigArn")]

    pub code_signing_config_arn: CodeSigningConfigArn,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<MaxListItems>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListFunctionsByCodeSigningConfigResponse {
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "FunctionArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arns: Option<FunctionArnList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListFunctionsRequest {
    #[serde(rename = "MasterRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_region: Option<MasterRegion>,
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<FunctionVersion>,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<MaxListItems>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListFunctionsResponse {
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<FunctionList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListLayerVersionsRequest {
    #[serde(rename = "CompatibleRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtime: Option<Runtime>,
    #[serde(rename = "LayerName")]

    pub layer_name: LayerName,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<MaxLayerListItems>,
    #[serde(rename = "CompatibleArchitecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architecture: Option<Architecture>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListLayerVersionsResponse {
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "LayerVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_versions: Option<LayerVersionsList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListLayersRequest {
    #[serde(rename = "CompatibleRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtime: Option<Runtime>,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<MaxLayerListItems>,
    #[serde(rename = "CompatibleArchitecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architecture: Option<Architecture>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListLayersResponse {
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<LayersList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListProvisionedConcurrencyConfigsRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<MaxProvisionedConcurrencyConfigListItems>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListProvisionedConcurrencyConfigsResponse {
    #[serde(rename = "ProvisionedConcurrencyConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_concurrency_configs: Option<ProvisionedConcurrencyConfigList>,
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTagsRequest {
    #[serde(rename = "Resource")]

    pub resource: FunctionArn,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTagsResponse {
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListVersionsByFunctionRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: NamespacedFunctionName,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<MaxListItems>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListVersionsByFunctionResponse {
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<FunctionList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OnFailure {
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DestinationArn>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OnSuccess {
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DestinationArn>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PolicyLengthExceededException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PreconditionFailedException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProvisionedConcurrencyConfigListItem {
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<FunctionArn>,
    #[serde(rename = "RequestedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_provisioned_concurrent_executions: Option<PositiveInteger>,
    #[serde(rename = "AvailableProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_provisioned_concurrent_executions: Option<NonNegativeInteger>,
    #[serde(rename = "AllocatedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_provisioned_concurrent_executions: Option<NonNegativeInteger>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProvisionedConcurrencyStatusEnum>,
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<Timestamp>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProvisionedConcurrencyConfigNotFoundException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PublishLayerVersionRequest {
    #[serde(rename = "LayerName")]

    pub layer_name: LayerName,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "Content")]

    pub content: LayerVersionContentInput,
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<CompatibleRuntimes>,
    #[serde(rename = "LicenseInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<LicenseInfo>,
    #[serde(rename = "CompatibleArchitectures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architectures: Option<CompatibleArchitectures>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PublishLayerVersionResponse {
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<LayerVersionContentOutput>,
    #[serde(rename = "LayerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_arn: Option<LayerArn>,
    #[serde(rename = "LayerVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_version_arn: Option<LayerVersionArn>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<Timestamp>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<LayerVersionNumber>,
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<CompatibleRuntimes>,
    #[serde(rename = "LicenseInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<LicenseInfo>,
    #[serde(rename = "CompatibleArchitectures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_architectures: Option<CompatibleArchitectures>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PublishVersionRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "CodeSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha256: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutFunctionCodeSigningConfigRequest {
    #[serde(rename = "CodeSigningConfigArn")]

    pub code_signing_config_arn: CodeSigningConfigArn,
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutFunctionCodeSigningConfigResponse {
    #[serde(rename = "CodeSigningConfigArn")]

    pub code_signing_config_arn: CodeSigningConfigArn,
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutFunctionConcurrencyRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "ReservedConcurrentExecutions")]

    pub reserved_concurrent_executions: ReservedConcurrentExecutions,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutFunctionEventInvokeConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<MaximumRetryAttempts>,
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<MaximumEventAgeInSeconds>,
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutProvisionedConcurrencyConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]

    pub qualifier: Qualifier,
    #[serde(rename = "ProvisionedConcurrentExecutions")]

    pub provisioned_concurrent_executions: PositiveInteger,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutProvisionedConcurrencyConfigResponse {
    #[serde(rename = "RequestedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_provisioned_concurrent_executions: Option<PositiveInteger>,
    #[serde(rename = "AvailableProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_provisioned_concurrent_executions: Option<NonNegativeInteger>,
    #[serde(rename = "AllocatedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_provisioned_concurrent_executions: Option<NonNegativeInteger>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProvisionedConcurrencyStatusEnum>,
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<Timestamp>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RemoveLayerVersionPermissionRequest {
    #[serde(rename = "LayerName")]

    pub layer_name: LayerName,
    #[serde(rename = "VersionNumber")]

    pub version_number: LayerVersionNumber,
    #[serde(rename = "StatementId")]

    pub statement_id: StatementId,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RemovePermissionRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "StatementId")]

    pub statement_id: NamespacedStatementId,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestTooLargeException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceConflictException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceInUseException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceNotFoundException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceNotReadyException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SelfManagedEventSource {
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Endpoints>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SelfManagedKafkaEventSourceConfig {
    #[serde(rename = "ConsumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<URI>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServiceException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SourceAccessConfiguration {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SourceAccessType>,
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<URI>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SubnetIPAddressLimitReachedException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TagResourceRequest {
    #[serde(rename = "Resource")]

    pub resource: FunctionArn,
    #[serde(rename = "Tags")]

    pub tags: Tags,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TooManyRequestsException {
    #[serde(rename = "retryAfterSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<ThrottleReason>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TracingConfig {
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<TracingMode>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TracingConfigResponse {
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<TracingMode>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UnsupportedMediaTypeException {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "Resource")]

    pub resource: FunctionArn,
    #[serde(rename = "TagKeys")]

    pub tag_keys: TagKeyList,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateAliasRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Name")]

    pub name: Alias,
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<Version>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateCodeSigningConfigRequest {
    #[serde(rename = "CodeSigningConfigArn")]

    pub code_signing_config_arn: CodeSigningConfigArn,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "AllowedPublishers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_publishers: Option<AllowedPublishers>,
    #[serde(rename = "CodeSigningPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_policies: Option<CodeSigningPolicies>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateCodeSigningConfigResponse {
    #[serde(rename = "CodeSigningConfig")]

    pub code_signing_config: CodeSigningConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateEventSourceMappingRequest {
    #[serde(rename = "UUID")]

    pub uuid: String,
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<FunctionName>,
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<BatchSize>,
    #[serde(rename = "FilterCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<MaximumBatchingWindowInSeconds>,
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<MaximumRecordAgeInSeconds>,
    #[serde(rename = "BisectBatchOnFunctionError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisect_batch_on_function_error: Option<BisectBatchOnFunctionError>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<MaximumRetryAttemptsEventSourceMapping>,
    #[serde(rename = "ParallelizationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<ParallelizationFactor>,
    #[serde(rename = "SourceAccessConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_access_configurations: Option<SourceAccessConfigurations>,
    #[serde(rename = "TumblingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumbling_window_in_seconds: Option<TumblingWindowInSeconds>,
    #[serde(rename = "FunctionResponseTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response_types: Option<FunctionResponseTypeList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateFunctionCodeRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "ZipFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<Blob>,
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<S3Bucket>,
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<S3Key>,
    #[serde(rename = "S3ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<S3ObjectVersion>,
    #[serde(rename = "ImageUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    #[serde(rename = "Publish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<Boolean>,
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<Boolean>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "Architectures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<ArchitecturesList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateFunctionConfigurationRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleArn>,
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<Handler>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Timeout>,
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<MemorySize>,
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<Runtime>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<KMSKeyArn>,
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfig>,
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<LayerList>,
    #[serde(rename = "FileSystemConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_configs: Option<FileSystemConfigList>,
    #[serde(rename = "ImageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageConfig>,
    #[serde(rename = "EphemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateFunctionEventInvokeConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<MaximumRetryAttempts>,
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<MaximumEventAgeInSeconds>,
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateFunctionUrlConfigRequest {
    #[serde(rename = "FunctionName")]

    pub function_name: FunctionName,
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<FunctionUrlQualifier>,
    #[serde(rename = "AuthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<FunctionUrlAuthType>,
    #[serde(rename = "Cors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateFunctionUrlConfigResponse {
    #[serde(rename = "FunctionUrl")]

    pub function_url: FunctionUrl,
    #[serde(rename = "FunctionArn")]

    pub function_arn: FunctionArn,
    #[serde(rename = "AuthType")]

    pub auth_type: FunctionUrlAuthType,
    #[serde(rename = "Cors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Cors>,
    #[serde(rename = "CreationTime")]

    pub creation_time: Timestamp,
    #[serde(rename = "LastModifiedTime")]

    pub last_modified_time: Timestamp,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VpcConfig {
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<SubnetIds>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<SecurityGroupIds>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VpcConfigResponse {
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<SubnetIds>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<SecurityGroupIds>,
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<VpcId>,
}
