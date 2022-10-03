// This file is generated!
// See https://github.com/akkoro/asml-aws-codegen

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type AddReplicaRegionListType = Vec<ReplicaRegionType>;
pub type AutomaticallyRotateAfterDaysType = u64;
pub type BooleanType = bool;
pub type ClientRequestTokenType = String;
pub type CreatedDateType = f64;
pub type DeletedDateType = f64;
pub type DeletionDateType = f64;
pub type DescriptionType = String;
pub type DurationType = String;
pub type ErrorMessage = String;
pub type ExcludeCharactersType = String;
pub type ExcludeLowercaseType = bool;
pub type ExcludeNumbersType = bool;
pub type ExcludePunctuationType = bool;
pub type ExcludeUppercaseType = bool;
pub type FilterNameStringType = String;
pub type FilterValueStringType = String;
pub type FilterValuesStringList = Vec<FilterValueStringType>;
pub type FiltersListType = Vec<Filter>;
pub type IncludeSpaceType = bool;
pub type KmsKeyIdListType = Vec<KmsKeyIdType>;
pub type KmsKeyIdType = String;
pub type LastAccessedDateType = f64;
pub type LastChangedDateType = f64;
pub type LastRotatedDateType = f64;
pub type MaxResultsType = i64;
pub type NameType = String;
pub type NextTokenType = String;
pub type NonEmptyResourcePolicyType = String;
pub type OwningServiceType = String;
pub type PasswordLengthType = u64;
pub type RandomPasswordType = String;
pub type RecoveryWindowInDaysType = u64;
pub type RegionType = String;
pub type RemoveReplicaRegionListType = Vec<RegionType>;
pub type ReplicationStatusListType = Vec<ReplicationStatusType>;
pub type RequireEachIncludedTypeType = bool;
pub type RotationEnabledType = bool;
pub type RotationLambdaARNType = String;
pub type ScheduleExpressionType = String;
pub type SecretARNType = String;
pub type SecretBinaryType = Vec<u8>;
pub type SecretIdType = String;
pub type SecretListType = Vec<SecretListEntry>;
pub type SecretNameType = String;
pub type SecretStringType = String;
pub type SecretVersionIdType = String;
pub type SecretVersionStageType = String;
pub type SecretVersionStagesType = Vec<SecretVersionStageType>;
pub type SecretVersionsListType = Vec<SecretVersionsListEntry>;
pub type SecretVersionsToStagesMapType = HashMap<SecretVersionIdType, SecretVersionStagesType>;
pub type SortOrderType = String;
pub type StatusMessageType = String;
pub type StatusType = String;
pub type TagKeyListType = Vec<TagKeyType>;
pub type TagKeyType = String;
pub type TagListType = Vec<Tag>;
pub type TagValueType = String;
pub type TimestampType = f64;
pub type ValidationErrorsType = Vec<ValidationErrorsEntry>;
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CancelRotateSecretRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CancelRotateSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<SecretVersionIdType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateSecretRequest {
    #[serde(rename = "Name")]

    pub name: NameType,
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<ClientRequestTokenType>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<DescriptionType>,
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<KmsKeyIdType>,
    #[serde(rename = "SecretBinary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_binary: Option<SecretBinaryType>,
    #[serde(rename = "SecretString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<SecretStringType>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagListType>,
    #[serde(rename = "AddReplicaRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_replica_regions: Option<AddReplicaRegionListType>,
    #[serde(rename = "ForceOverwriteReplicaSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_overwrite_replica_secret: Option<BooleanType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<SecretVersionIdType>,
    #[serde(rename = "ReplicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<ReplicationStatusListType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DecryptionFailure {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteResourcePolicyResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<NameType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteSecretRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "RecoveryWindowInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_window_in_days: Option<RecoveryWindowInDaysType>,
    #[serde(rename = "ForceDeleteWithoutRecovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete_without_recovery: Option<BooleanType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
    #[serde(rename = "DeletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<DeletionDateType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeSecretRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DescribeSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<DescriptionType>,
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<KmsKeyIdType>,
    #[serde(rename = "RotationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_enabled: Option<RotationEnabledType>,
    #[serde(rename = "RotationLambdaARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_arn: Option<RotationLambdaARNType>,
    #[serde(rename = "RotationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRulesType>,
    #[serde(rename = "LastRotatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rotated_date: Option<LastRotatedDateType>,
    #[serde(rename = "LastChangedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_date: Option<LastChangedDateType>,
    #[serde(rename = "LastAccessedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<LastAccessedDateType>,
    #[serde(rename = "DeletedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<DeletedDateType>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagListType>,
    #[serde(rename = "VersionIdsToStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_ids_to_stages: Option<SecretVersionsToStagesMapType>,
    #[serde(rename = "OwningService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_service: Option<OwningServiceType>,
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<TimestampType>,
    #[serde(rename = "PrimaryRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_region: Option<RegionType>,
    #[serde(rename = "ReplicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<ReplicationStatusListType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EncryptionFailure {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<FilterNameStringType>,
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<FilterValuesStringList>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetRandomPasswordRequest {
    #[serde(rename = "PasswordLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_length: Option<PasswordLengthType>,
    #[serde(rename = "ExcludeCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_characters: Option<ExcludeCharactersType>,
    #[serde(rename = "ExcludeNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_numbers: Option<ExcludeNumbersType>,
    #[serde(rename = "ExcludePunctuation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_punctuation: Option<ExcludePunctuationType>,
    #[serde(rename = "ExcludeUppercase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_uppercase: Option<ExcludeUppercaseType>,
    #[serde(rename = "ExcludeLowercase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_lowercase: Option<ExcludeLowercaseType>,
    #[serde(rename = "IncludeSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_space: Option<IncludeSpaceType>,
    #[serde(rename = "RequireEachIncludedType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_each_included_type: Option<RequireEachIncludedTypeType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetRandomPasswordResponse {
    #[serde(rename = "RandomPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_password: Option<RandomPasswordType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetResourcePolicyRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetResourcePolicyResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<NameType>,
    #[serde(rename = "ResourcePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<NonEmptyResourcePolicyType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetSecretValueRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<SecretVersionIdType>,
    #[serde(rename = "VersionStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stage: Option<SecretVersionStageType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetSecretValueResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<SecretVersionIdType>,
    #[serde(rename = "SecretBinary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_binary: Option<SecretBinaryType>,
    #[serde(rename = "SecretString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<SecretStringType>,
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<SecretVersionStagesType>,
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<CreatedDateType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InternalServiceError {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidNextTokenException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidParameterException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidRequestException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LimitExceededException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListSecretVersionIdsRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<MaxResultsType>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<NextTokenType>,
    #[serde(rename = "IncludeDeprecated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deprecated: Option<BooleanType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListSecretVersionIdsResponse {
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<SecretVersionsListType>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<NextTokenType>,
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListSecretsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<MaxResultsType>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<NextTokenType>,
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FiltersListType>,
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrderType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListSecretsResponse {
    #[serde(rename = "SecretList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_list: Option<SecretListType>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<NextTokenType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MalformedPolicyDocumentException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PreconditionNotMetException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PublicPolicyException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "ResourcePolicy")]

    pub resource_policy: NonEmptyResourcePolicyType,
    #[serde(rename = "BlockPublicPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_policy: Option<BooleanType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<NameType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutSecretValueRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<ClientRequestTokenType>,
    #[serde(rename = "SecretBinary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_binary: Option<SecretBinaryType>,
    #[serde(rename = "SecretString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<SecretStringType>,
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<SecretVersionStagesType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutSecretValueResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<SecretVersionIdType>,
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<SecretVersionStagesType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RemoveRegionsFromReplicationRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "RemoveReplicaRegions")]

    pub remove_replica_regions: RemoveReplicaRegionListType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RemoveRegionsFromReplicationResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "ReplicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<ReplicationStatusListType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaRegionType {
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<RegionType>,
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<KmsKeyIdType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicateSecretToRegionsRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "AddReplicaRegions")]

    pub add_replica_regions: AddReplicaRegionListType,
    #[serde(rename = "ForceOverwriteReplicaSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_overwrite_replica_secret: Option<BooleanType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicateSecretToRegionsResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "ReplicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<ReplicationStatusListType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicationStatusType {
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<RegionType>,
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<KmsKeyIdType>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusType>,
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<StatusMessageType>,
    #[serde(rename = "LastAccessedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<LastAccessedDateType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceExistsException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceNotFoundException {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreSecretRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RotateSecretRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<ClientRequestTokenType>,
    #[serde(rename = "RotationLambdaARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_arn: Option<RotationLambdaARNType>,
    #[serde(rename = "RotationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRulesType>,
    #[serde(rename = "RotateImmediately")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_immediately: Option<BooleanType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RotateSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<SecretVersionIdType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RotationRulesType {
    #[serde(rename = "AutomaticallyAfterDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_after_days: Option<AutomaticallyRotateAfterDaysType>,
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<DurationType>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<ScheduleExpressionType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SecretListEntry {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<DescriptionType>,
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<KmsKeyIdType>,
    #[serde(rename = "RotationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_enabled: Option<RotationEnabledType>,
    #[serde(rename = "RotationLambdaARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_arn: Option<RotationLambdaARNType>,
    #[serde(rename = "RotationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRulesType>,
    #[serde(rename = "LastRotatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rotated_date: Option<LastRotatedDateType>,
    #[serde(rename = "LastChangedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_date: Option<LastChangedDateType>,
    #[serde(rename = "LastAccessedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<LastAccessedDateType>,
    #[serde(rename = "DeletedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<DeletedDateType>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagListType>,
    #[serde(rename = "SecretVersionsToStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_versions_to_stages: Option<SecretVersionsToStagesMapType>,
    #[serde(rename = "OwningService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_service: Option<OwningServiceType>,
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<TimestampType>,
    #[serde(rename = "PrimaryRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_region: Option<RegionType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SecretVersionsListEntry {
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<SecretVersionIdType>,
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<SecretVersionStagesType>,
    #[serde(rename = "LastAccessedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<LastAccessedDateType>,
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<CreatedDateType>,
    #[serde(rename = "KmsKeyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_ids: Option<KmsKeyIdListType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StopReplicationToReplicaRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StopReplicationToReplicaResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<TagKeyType>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TagValueType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TagResourceRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "Tags")]

    pub tags: TagListType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "TagKeys")]

    pub tag_keys: TagKeyListType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateSecretRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<ClientRequestTokenType>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<DescriptionType>,
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<KmsKeyIdType>,
    #[serde(rename = "SecretBinary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_binary: Option<SecretBinaryType>,
    #[serde(rename = "SecretString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<SecretStringType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<SecretVersionIdType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateSecretVersionStageRequest {
    #[serde(rename = "SecretId")]

    pub secret_id: SecretIdType,
    #[serde(rename = "VersionStage")]

    pub version_stage: SecretVersionStageType,
    #[serde(rename = "RemoveFromVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_from_version_id: Option<SecretVersionIdType>,
    #[serde(rename = "MoveToVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_version_id: Option<SecretVersionIdType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateSecretVersionStageResponse {
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<SecretARNType>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SecretNameType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ValidateResourcePolicyRequest {
    #[serde(rename = "SecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<SecretIdType>,
    #[serde(rename = "ResourcePolicy")]

    pub resource_policy: NonEmptyResourcePolicyType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ValidateResourcePolicyResponse {
    #[serde(rename = "PolicyValidationPassed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_validation_passed: Option<BooleanType>,
    #[serde(rename = "ValidationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<ValidationErrorsType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ValidationErrorsEntry {
    #[serde(rename = "CheckName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_name: Option<NameType>,
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<ErrorMessage>,
}
