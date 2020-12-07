// This file is generated!
// See https://github.com/akkoro/asml-aws-codegen

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type AbortDate = String;
pub type AbortRuleId = String;
pub type AcceptRanges = String;
pub type AccountId = String;
pub type AllowQuotedRecordDelimiter = bool;
pub type AllowedHeader = String;
pub type AllowedHeaders = Vec<AllowedHeader>;
pub type AllowedMethod = String;
pub type AllowedMethods = Vec<AllowedMethod>;
pub type AllowedOrigin = String;
pub type AllowedOrigins = Vec<AllowedOrigin>;
pub type AnalyticsConfigurationList = Vec<AnalyticsConfiguration>;
pub type AnalyticsId = String;
pub type AnalyticsS3ExportFileFormat = String;
pub type ArchiveStatus = String;
pub type Body = Vec<u8>;
pub type BucketAccelerateStatus = String;
pub type BucketCannedACL = String;
pub type BucketKeyEnabled = bool;
pub type BucketLocationConstraint = String;
pub type BucketLogsPermission = String;
pub type BucketName = String;
pub type BucketVersioningStatus = String;
pub type Buckets = Vec<Bucket>;
pub type BypassGovernanceRetention = bool;
pub type BytesProcessed = u64;
pub type BytesReturned = u64;
pub type BytesScanned = u64;
pub type CORSRules = Vec<CORSRule>;
pub type CacheControl = String;
pub type CloudFunction = String;
pub type CloudFunctionInvocationRole = String;
pub type Code = String;
pub type Comments = String;
pub type CommonPrefixList = Vec<CommonPrefix>;
pub type CompletedPartList = Vec<CompletedPart>;
pub type CompressionType = String;
pub type ConfirmRemoveSelfBucketAccess = bool;
pub type ContentDisposition = String;
pub type ContentEncoding = String;
pub type ContentLanguage = String;
pub type ContentLength = u64;
pub type ContentMD5 = String;
pub type ContentRange = String;
pub type ContentType = String;
pub type CopySource = String;
pub type CopySourceIfMatch = String;
pub type CopySourceIfModifiedSince = String;
pub type CopySourceIfNoneMatch = String;
pub type CopySourceIfUnmodifiedSince = String;
pub type CopySourceRange = String;
pub type CopySourceSSECustomerAlgorithm = String;
pub type CopySourceSSECustomerKey = String;
pub type CopySourceSSECustomerKeyMD5 = String;
pub type CopySourceVersionId = String;
pub type CreationDate = String;
pub type Date = String;
pub type Days = u64;
pub type DaysAfterInitiation = u64;
pub type DeleteMarker = bool;
pub type DeleteMarkerReplicationStatus = String;
pub type DeleteMarkerVersionId = String;
pub type DeleteMarkers = Vec<DeleteMarkerEntry>;
pub type DeletedObjects = Vec<DeletedObject>;
pub type Delimiter = String;
pub type Description = String;
pub type DisplayName = String;
pub type ETag = String;
pub type EmailAddress = String;
pub type EnableRequestProgress = bool;
pub type EncodingType = String;
pub type End = u64;
pub type Errors = Vec<Error>;
pub type Event = String;
pub type EventList = Vec<Event>;
pub type ExistingObjectReplicationStatus = String;
pub type Expiration = String;
pub type ExpirationStatus = String;
pub type ExpiredObjectDeleteMarker = bool;
pub type Expires = String;
pub type ExposeHeader = String;
pub type ExposeHeaders = Vec<ExposeHeader>;
pub type Expression = String;
pub type ExpressionType = String;
pub type FetchOwner = bool;
pub type FieldDelimiter = String;
pub type FileHeaderInfo = String;
pub type FilterRuleList = Vec<FilterRule>;
pub type FilterRuleName = String;
pub type FilterRuleValue = String;
pub type GrantFullControl = String;
pub type GrantRead = String;
pub type GrantReadACP = String;
pub type GrantWrite = String;
pub type GrantWriteACP = String;
pub type Grants = Vec<Grant>;
pub type HostName = String;
pub type HttpErrorCodeReturnedEquals = String;
pub type HttpRedirectCode = String;
pub type ID = String;
pub type IfMatch = String;
pub type IfModifiedSince = String;
pub type IfNoneMatch = String;
pub type IfUnmodifiedSince = String;
pub type Initiated = String;
pub type IntelligentTieringAccessTier = String;
pub type IntelligentTieringConfigurationList = Vec<IntelligentTieringConfiguration>;
pub type IntelligentTieringDays = u64;
pub type IntelligentTieringId = String;
pub type IntelligentTieringStatus = String;
pub type InventoryConfigurationList = Vec<InventoryConfiguration>;
pub type InventoryFormat = String;
pub type InventoryFrequency = String;
pub type InventoryId = String;
pub type InventoryIncludedObjectVersions = String;
pub type InventoryOptionalField = String;
pub type InventoryOptionalFields = Vec<InventoryOptionalField>;
pub type IsEnabled = bool;
pub type IsLatest = bool;
pub type IsPublic = bool;
pub type IsTruncated = bool;
pub type JSONType = String;
pub type KMSContext = String;
pub type KeyCount = u64;
pub type KeyMarker = String;
pub type KeyPrefixEquals = String;
pub type LambdaFunctionArn = String;
pub type LambdaFunctionConfigurationList = Vec<LambdaFunctionConfiguration>;
pub type LastModified = String;
pub type LifecycleRules = Vec<LifecycleRule>;
pub type Location = String;
pub type LocationPrefix = String;
pub type MFA = String;
pub type MFADelete = String;
pub type MFADeleteStatus = String;
pub type Marker = String;
pub type MaxAgeSeconds = u64;
pub type MaxKeys = u64;
pub type MaxParts = u64;
pub type MaxUploads = u64;
pub type Message = String;
pub type Metadata = HashMap<MetadataKey, MetadataValue>;
pub type MetadataDirective = String;
pub type MetadataKey = String;
pub type MetadataValue = String;
pub type MetricsConfigurationList = Vec<MetricsConfiguration>;
pub type MetricsId = String;
pub type MetricsStatus = String;
pub type Minutes = u64;
pub type MissingMeta = u64;
pub type MultipartUploadId = String;
pub type MultipartUploadList = Vec<MultipartUpload>;
pub type NextKeyMarker = String;
pub type NextMarker = String;
pub type NextPartNumberMarker = u64;
pub type NextToken = String;
pub type NextUploadIdMarker = String;
pub type NextVersionIdMarker = String;
pub type NoncurrentVersionTransitionList = Vec<NoncurrentVersionTransition>;
pub type NotificationId = String;
pub type ObjectCannedACL = String;
pub type ObjectIdentifierList = Vec<ObjectIdentifier>;
pub type ObjectKey = String;
pub type ObjectList = Vec<Object>;
pub type ObjectLockEnabled = String;
pub type ObjectLockEnabledForBucket = bool;
pub type ObjectLockLegalHoldStatus = String;
pub type ObjectLockMode = String;
pub type ObjectLockRetainUntilDate = String;
pub type ObjectLockRetentionMode = String;
pub type ObjectLockToken = String;
pub type ObjectOwnership = String;
pub type ObjectStorageClass = String;
pub type ObjectVersionId = String;
pub type ObjectVersionList = Vec<ObjectVersion>;
pub type ObjectVersionStorageClass = String;
pub type OwnerOverride = String;
pub type OwnershipControlsRules = Vec<OwnershipControlsRule>;
pub type PartNumber = u64;
pub type PartNumberMarker = u64;
pub type Parts = Vec<Part>;
pub type PartsCount = u64;
pub type Payer = String;
pub type Permission = String;
pub type Policy = String;
pub type Prefix = String;
pub type Priority = u64;
pub type Protocol = String;
pub type QueueArn = String;
pub type QueueConfigurationList = Vec<QueueConfiguration>;
pub type Quiet = bool;
pub type QuoteCharacter = String;
pub type QuoteEscapeCharacter = String;
pub type QuoteFields = String;
pub type Range = String;
pub type RecordDelimiter = String;
pub type ReplaceKeyPrefixWith = String;
pub type ReplaceKeyWith = String;
pub type ReplicaKmsKeyID = String;
pub type ReplicaModificationsStatus = String;
pub type ReplicationRuleStatus = String;
pub type ReplicationRules = Vec<ReplicationRule>;
pub type ReplicationStatus = String;
pub type ReplicationTimeStatus = String;
pub type RequestCharged = String;
pub type RequestPayer = String;
pub type ResponseCacheControl = String;
pub type ResponseContentDisposition = String;
pub type ResponseContentEncoding = String;
pub type ResponseContentLanguage = String;
pub type ResponseContentType = String;
pub type ResponseExpires = String;
pub type Restore = String;
pub type RestoreOutputPath = String;
pub type RestoreRequestType = String;
pub type Role = String;
pub type RoutingRules = Vec<RoutingRule>;
pub type Rules = Vec<Rule>;
pub type SSECustomerAlgorithm = String;
pub type SSECustomerKey = String;
pub type SSECustomerKeyMD5 = String;
pub type SSEKMSEncryptionContext = String;
pub type SSEKMSKeyId = String;
pub type ServerSideEncryption = String;
pub type ServerSideEncryptionRules = Vec<ServerSideEncryptionRule>;
pub type Setting = bool;
pub type Size = u64;
pub type SseKmsEncryptedObjectsStatus = String;
pub type Start = u64;
pub type StartAfter = String;
pub type StorageClass = String;
pub type StorageClassAnalysisSchemaVersion = String;
pub type Suffix = String;
pub type TagCount = u64;
pub type TagSet = Vec<Tag>;
pub type TaggingDirective = String;
pub type TaggingHeader = String;
pub type TargetBucket = String;
pub type TargetGrants = Vec<TargetGrant>;
pub type TargetPrefix = String;
pub type Tier = String;
pub type TieringList = Vec<Tiering>;
pub type Token = String;
pub type TopicArn = String;
pub type TopicConfigurationList = Vec<TopicConfiguration>;
pub type TransitionList = Vec<Transition>;
pub type TransitionStorageClass = String;
pub type Type = String;
pub type URI = String;
pub type UploadIdMarker = String;
pub type UserMetadata = Vec<MetadataEntry>;
pub type Value = String;
pub type VersionIdMarker = String;
pub type WebsiteRedirectLocation = String;
pub type Years = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct AbortIncompleteMultipartUpload {
    #[serde(rename = "DaysAfterInitiation")]
    pub days_after_initiation: Option<DaysAfterInitiation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbortMultipartUploadOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbortMultipartUploadRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "UploadId")]
    pub upload_id: MultipartUploadId,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccelerateConfiguration {
    #[serde(rename = "Status")]
    pub status: Option<BucketAccelerateStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessControlPolicy {
    #[serde(rename = "Grants")]
    pub grants: Option<Grants>,
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessControlTranslation {
    #[serde(rename = "Owner")]
    pub owner: OwnerOverride,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsAndOperator {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagSet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsConfiguration {
    #[serde(rename = "Id")]
    pub id: AnalyticsId,
    #[serde(rename = "Filter")]
    pub filter: AnalyticsFilter,
    #[serde(rename = "StorageClassAnalysis")]
    pub storage_class_analysis: StorageClassAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsExportDestination {
    #[serde(rename = "S3BucketDestination")]
    pub s3_bucket_destination: AnalyticsS3BucketDestination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tag")]
    pub tag: Option<Tag>,
    #[serde(rename = "And")]
    pub and: Option<AnalyticsAndOperator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsS3BucketDestination {
    #[serde(rename = "Format")]
    pub format: AnalyticsS3ExportFileFormat,
    #[serde(rename = "BucketAccountId")]
    pub bucket_account_id: AccountId,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bucket {
    #[serde(rename = "Name")]
    pub name: Option<BucketName>,
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<CreationDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketAlreadyExists {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketAlreadyOwnedByYou {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketLifecycleConfiguration {
    #[serde(rename = "Rules")]
    pub rules: LifecycleRules,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketLoggingStatus {
    #[serde(rename = "LoggingEnabled")]
    pub logging_enabled: Option<LoggingEnabled>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CORSConfiguration {
    #[serde(rename = "CORSRules")]
    pub cors_rules: CORSRules,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CORSRule {
    #[serde(rename = "AllowedHeaders")]
    pub allowed_headers: AllowedHeaders,
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: AllowedMethods,
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: AllowedOrigins,
    #[serde(rename = "ExposeHeaders")]
    pub expose_headers: ExposeHeaders,
    #[serde(rename = "MaxAgeSeconds")]
    pub max_age_seconds: MaxAgeSeconds,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CSVInput {
    #[serde(rename = "FileHeaderInfo")]
    pub file_header_info: Option<FileHeaderInfo>,
    #[serde(rename = "Comments")]
    pub comments: Option<Comments>,
    #[serde(rename = "QuoteEscapeCharacter")]
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
    #[serde(rename = "RecordDelimiter")]
    pub record_delimiter: Option<RecordDelimiter>,
    #[serde(rename = "FieldDelimiter")]
    pub field_delimiter: Option<FieldDelimiter>,
    #[serde(rename = "QuoteCharacter")]
    pub quote_character: Option<QuoteCharacter>,
    #[serde(rename = "AllowQuotedRecordDelimiter")]
    pub allow_quoted_record_delimiter: Option<AllowQuotedRecordDelimiter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CSVOutput {
    #[serde(rename = "QuoteFields")]
    pub quote_fields: Option<QuoteFields>,
    #[serde(rename = "QuoteEscapeCharacter")]
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
    #[serde(rename = "RecordDelimiter")]
    pub record_delimiter: Option<RecordDelimiter>,
    #[serde(rename = "FieldDelimiter")]
    pub field_delimiter: Option<FieldDelimiter>,
    #[serde(rename = "QuoteCharacter")]
    pub quote_character: Option<QuoteCharacter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudFunctionConfiguration {
    #[serde(rename = "Id")]
    pub id: Option<NotificationId>,
    #[serde(rename = "Event")]
    pub event: Option<Event>,
    #[serde(rename = "Events")]
    pub events: Option<EventList>,
    #[serde(rename = "CloudFunction")]
    pub cloud_function: Option<CloudFunction>,
    #[serde(rename = "InvocationRole")]
    pub invocation_role: Option<CloudFunctionInvocationRole>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonPrefix {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteMultipartUploadOutput {
    #[serde(rename = "Location")]
    pub location: Option<Location>,
    #[serde(rename = "Bucket")]
    pub bucket: Option<BucketName>,
    #[serde(rename = "Key")]
    pub key: Option<ObjectKey>,
    #[serde(rename = "Expiration")]
    pub expiration: Option<Expiration>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteMultipartUploadRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "MultipartUpload")]
    pub multipart_upload: CompletedMultipartUpload,
    #[serde(rename = "UploadId")]
    pub upload_id: MultipartUploadId,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedMultipartUpload {
    #[serde(rename = "Parts")]
    pub parts: Option<CompletedPartList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedPart {
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "PartNumber")]
    pub part_number: Option<PartNumber>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "HttpErrorCodeReturnedEquals")]
    pub http_error_code_returned_equals: Option<HttpErrorCodeReturnedEquals>,
    #[serde(rename = "KeyPrefixEquals")]
    pub key_prefix_equals: Option<KeyPrefixEquals>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinuationEvent {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyObjectOutput {
    #[serde(rename = "CopyObjectResult")]
    pub copy_object_result: Option<CopyObjectResult>,
    #[serde(rename = "Expiration")]
    pub expiration: Option<Expiration>,
    #[serde(rename = "CopySourceVersionId")]
    pub copy_source_version_id: Option<CopySourceVersionId>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "SSEKMSEncryptionContext")]
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyObjectRequest {
    #[serde(rename = "ACL")]
    pub acl: ObjectCannedACL,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "CacheControl")]
    pub cache_control: CacheControl,
    #[serde(rename = "ContentDisposition")]
    pub content_disposition: ContentDisposition,
    #[serde(rename = "ContentEncoding")]
    pub content_encoding: ContentEncoding,
    #[serde(rename = "ContentLanguage")]
    pub content_language: ContentLanguage,
    #[serde(rename = "ContentType")]
    pub content_type: ContentType,
    #[serde(rename = "CopySource")]
    pub copy_source: CopySource,
    #[serde(rename = "CopySourceIfMatch")]
    pub copy_source_if_match: CopySourceIfMatch,
    #[serde(rename = "CopySourceIfModifiedSince")]
    pub copy_source_if_modified_since: CopySourceIfModifiedSince,
    #[serde(rename = "CopySourceIfNoneMatch")]
    pub copy_source_if_none_match: CopySourceIfNoneMatch,
    #[serde(rename = "CopySourceIfUnmodifiedSince")]
    pub copy_source_if_unmodified_since: CopySourceIfUnmodifiedSince,
    #[serde(rename = "Expires")]
    pub expires: Expires,
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: GrantFullControl,
    #[serde(rename = "GrantRead")]
    pub grant_read: GrantRead,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: GrantReadACP,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: GrantWriteACP,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Metadata")]
    pub metadata: Metadata,
    #[serde(rename = "MetadataDirective")]
    pub metadata_directive: MetadataDirective,
    #[serde(rename = "TaggingDirective")]
    pub tagging_directive: TaggingDirective,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: ServerSideEncryption,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "WebsiteRedirectLocation")]
    pub website_redirect_location: WebsiteRedirectLocation,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: SSECustomerKey,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: SSEKMSKeyId,
    #[serde(rename = "SSEKMSEncryptionContext")]
    pub ssekms_encryption_context: SSEKMSEncryptionContext,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: BucketKeyEnabled,
    #[serde(rename = "CopySourceSSECustomerAlgorithm")]
    pub copy_source_sse_customer_algorithm: CopySourceSSECustomerAlgorithm,
    #[serde(rename = "CopySourceSSECustomerKey")]
    pub copy_source_sse_customer_key: CopySourceSSECustomerKey,
    #[serde(rename = "CopySourceSSECustomerKeyMD5")]
    pub copy_source_sse_customer_key_md5: CopySourceSSECustomerKeyMD5,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "Tagging")]
    pub tagging: TaggingHeader,
    #[serde(rename = "ObjectLockMode")]
    pub object_lock_mode: ObjectLockMode,
    #[serde(rename = "ObjectLockRetainUntilDate")]
    pub object_lock_retain_until_date: ObjectLockRetainUntilDate,
    #[serde(rename = "ObjectLockLegalHoldStatus")]
    pub object_lock_legal_hold_status: ObjectLockLegalHoldStatus,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
    #[serde(rename = "ExpectedSourceBucketOwner")]
    pub expected_source_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyObjectResult {
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<LastModified>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyPartResult {
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<LastModified>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBucketConfiguration {
    #[serde(rename = "LocationConstraint")]
    pub location_constraint: Option<BucketLocationConstraint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBucketOutput {
    #[serde(rename = "Location")]
    pub location: Option<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBucketRequest {
    #[serde(rename = "ACL")]
    pub acl: BucketCannedACL,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "CreateBucketConfiguration")]
    pub create_bucket_configuration: CreateBucketConfiguration,
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: GrantFullControl,
    #[serde(rename = "GrantRead")]
    pub grant_read: GrantRead,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: GrantReadACP,
    #[serde(rename = "GrantWrite")]
    pub grant_write: GrantWrite,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: GrantWriteACP,
    #[serde(rename = "ObjectLockEnabledForBucket")]
    pub object_lock_enabled_for_bucket: ObjectLockEnabledForBucket,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMultipartUploadOutput {
    #[serde(rename = "AbortDate")]
    pub abort_date: Option<AbortDate>,
    #[serde(rename = "AbortRuleId")]
    pub abort_rule_id: Option<AbortRuleId>,
    #[serde(rename = "Bucket")]
    pub bucket: Option<BucketName>,
    #[serde(rename = "Key")]
    pub key: Option<ObjectKey>,
    #[serde(rename = "UploadId")]
    pub upload_id: Option<MultipartUploadId>,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "SSEKMSEncryptionContext")]
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMultipartUploadRequest {
    #[serde(rename = "ACL")]
    pub acl: ObjectCannedACL,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "CacheControl")]
    pub cache_control: CacheControl,
    #[serde(rename = "ContentDisposition")]
    pub content_disposition: ContentDisposition,
    #[serde(rename = "ContentEncoding")]
    pub content_encoding: ContentEncoding,
    #[serde(rename = "ContentLanguage")]
    pub content_language: ContentLanguage,
    #[serde(rename = "ContentType")]
    pub content_type: ContentType,
    #[serde(rename = "Expires")]
    pub expires: Expires,
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: GrantFullControl,
    #[serde(rename = "GrantRead")]
    pub grant_read: GrantRead,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: GrantReadACP,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: GrantWriteACP,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Metadata")]
    pub metadata: Metadata,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: ServerSideEncryption,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "WebsiteRedirectLocation")]
    pub website_redirect_location: WebsiteRedirectLocation,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: SSECustomerKey,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: SSEKMSKeyId,
    #[serde(rename = "SSEKMSEncryptionContext")]
    pub ssekms_encryption_context: SSEKMSEncryptionContext,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: BucketKeyEnabled,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "Tagging")]
    pub tagging: TaggingHeader,
    #[serde(rename = "ObjectLockMode")]
    pub object_lock_mode: ObjectLockMode,
    #[serde(rename = "ObjectLockRetainUntilDate")]
    pub object_lock_retain_until_date: ObjectLockRetainUntilDate,
    #[serde(rename = "ObjectLockLegalHoldStatus")]
    pub object_lock_legal_hold_status: ObjectLockLegalHoldStatus,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultRetention {
    #[serde(rename = "Mode")]
    pub mode: Option<ObjectLockRetentionMode>,
    #[serde(rename = "Days")]
    pub days: Option<Days>,
    #[serde(rename = "Years")]
    pub years: Option<Years>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delete {
    #[serde(rename = "Objects")]
    pub objects: ObjectIdentifierList,
    #[serde(rename = "Quiet")]
    pub quiet: Quiet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketAnalyticsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: AnalyticsId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketCorsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketEncryptionRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketIntelligentTieringConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: IntelligentTieringId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketInventoryConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: InventoryId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketLifecycleRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketMetricsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: MetricsId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketOwnershipControlsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketPolicyRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketReplicationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBucketWebsiteRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMarkerEntry {
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    #[serde(rename = "Key")]
    pub key: Option<ObjectKey>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "IsLatest")]
    pub is_latest: Option<IsLatest>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<LastModified>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMarkerReplication {
    #[serde(rename = "Status")]
    pub status: Option<DeleteMarkerReplicationStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteObjectOutput {
    #[serde(rename = "DeleteMarker")]
    pub delete_marker: Option<DeleteMarker>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteObjectRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "MFA")]
    pub mfa: MFA,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "BypassGovernanceRetention")]
    pub bypass_governance_retention: BypassGovernanceRetention,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteObjectTaggingOutput {
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteObjectTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteObjectsOutput {
    #[serde(rename = "Deleted")]
    pub deleted: Option<DeletedObjects>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
    #[serde(rename = "Errors")]
    pub errors: Option<Errors>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteObjectsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Delete")]
    pub delete: Delete,
    #[serde(rename = "MFA")]
    pub mfa: MFA,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "BypassGovernanceRetention")]
    pub bypass_governance_retention: BypassGovernanceRetention,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePublicAccessBlockRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedObject {
    #[serde(rename = "Key")]
    pub key: Option<ObjectKey>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "DeleteMarker")]
    pub delete_marker: Option<DeleteMarker>,
    #[serde(rename = "DeleteMarkerVersionId")]
    pub delete_marker_version_id: Option<DeleteMarkerVersionId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Destination {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Account")]
    pub account: AccountId,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "AccessControlTranslation")]
    pub access_control_translation: AccessControlTranslation,
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,
    #[serde(rename = "ReplicationTime")]
    pub replication_time: ReplicationTime,
    #[serde(rename = "Metrics")]
    pub metrics: Metrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Encryption {
    #[serde(rename = "EncryptionType")]
    pub encryption_type: ServerSideEncryption,
    #[serde(rename = "KMSKeyId")]
    pub kms_key_id: SSEKMSKeyId,
    #[serde(rename = "KMSContext")]
    pub kms_context: KMSContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "ReplicaKmsKeyID")]
    pub replica_kms_key_id: Option<ReplicaKmsKeyID>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndEvent {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "Key")]
    pub key: Option<ObjectKey>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "Code")]
    pub code: Option<Code>,
    #[serde(rename = "Message")]
    pub message: Option<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDocument {
    #[serde(rename = "Key")]
    pub key: ObjectKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExistingObjectReplication {
    #[serde(rename = "Status")]
    pub status: ExistingObjectReplicationStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterRule {
    #[serde(rename = "Name")]
    pub name: Option<FilterRuleName>,
    #[serde(rename = "Value")]
    pub value: Option<FilterRuleValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketAccelerateConfigurationOutput {
    #[serde(rename = "Status")]
    pub status: Option<BucketAccelerateStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketAccelerateConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketAclOutput {
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    #[serde(rename = "Grants")]
    pub grants: Option<Grants>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketAclRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketAnalyticsConfigurationOutput {
    #[serde(rename = "AnalyticsConfiguration")]
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketAnalyticsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: AnalyticsId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketCorsOutput {
    #[serde(rename = "CORSRules")]
    pub cors_rules: Option<CORSRules>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketCorsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketEncryptionOutput {
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketEncryptionRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketIntelligentTieringConfigurationOutput {
    #[serde(rename = "IntelligentTieringConfiguration")]
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketIntelligentTieringConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: IntelligentTieringId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketInventoryConfigurationOutput {
    #[serde(rename = "InventoryConfiguration")]
    pub inventory_configuration: Option<InventoryConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketInventoryConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: InventoryId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketLifecycleConfigurationOutput {
    #[serde(rename = "Rules")]
    pub rules: Option<LifecycleRules>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketLifecycleConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketLifecycleOutput {
    #[serde(rename = "Rules")]
    pub rules: Option<Rules>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketLifecycleRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketLocationOutput {
    #[serde(rename = "LocationConstraint")]
    pub location_constraint: Option<BucketLocationConstraint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketLocationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketLoggingOutput {
    #[serde(rename = "LoggingEnabled")]
    pub logging_enabled: Option<LoggingEnabled>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketLoggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketMetricsConfigurationOutput {
    #[serde(rename = "MetricsConfiguration")]
    pub metrics_configuration: Option<MetricsConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketMetricsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: MetricsId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketNotificationConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketOwnershipControlsOutput {
    #[serde(rename = "OwnershipControls")]
    pub ownership_controls: Option<OwnershipControls>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketOwnershipControlsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketPolicyOutput {
    #[serde(rename = "Policy")]
    pub policy: Option<Policy>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketPolicyRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketPolicyStatusOutput {
    #[serde(rename = "PolicyStatus")]
    pub policy_status: Option<PolicyStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketPolicyStatusRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketReplicationOutput {
    #[serde(rename = "ReplicationConfiguration")]
    pub replication_configuration: Option<ReplicationConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketReplicationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketRequestPaymentOutput {
    #[serde(rename = "Payer")]
    pub payer: Option<Payer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketRequestPaymentRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketTaggingOutput {
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketVersioningOutput {
    #[serde(rename = "Status")]
    pub status: Option<BucketVersioningStatus>,
    #[serde(rename = "MFADelete")]
    pub mfa_delete: Option<MFADeleteStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketVersioningRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketWebsiteOutput {
    #[serde(rename = "RedirectAllRequestsTo")]
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    #[serde(rename = "IndexDocument")]
    pub index_document: Option<IndexDocument>,
    #[serde(rename = "ErrorDocument")]
    pub error_document: Option<ErrorDocument>,
    #[serde(rename = "RoutingRules")]
    pub routing_rules: Option<RoutingRules>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBucketWebsiteRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectAclOutput {
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    #[serde(rename = "Grants")]
    pub grants: Option<Grants>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectAclRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectLegalHoldOutput {
    #[serde(rename = "LegalHold")]
    pub legal_hold: Option<ObjectLockLegalHold>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectLegalHoldRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectLockConfigurationOutput {
    #[serde(rename = "ObjectLockConfiguration")]
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectLockConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectOutput {
    #[serde(rename = "Body")]
    pub body: Option<Body>,
    #[serde(rename = "DeleteMarker")]
    pub delete_marker: Option<DeleteMarker>,
    #[serde(rename = "AcceptRanges")]
    pub accept_ranges: Option<AcceptRanges>,
    #[serde(rename = "Expiration")]
    pub expiration: Option<Expiration>,
    #[serde(rename = "Restore")]
    pub restore: Option<Restore>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<LastModified>,
    #[serde(rename = "ContentLength")]
    pub content_length: Option<ContentLength>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "MissingMeta")]
    pub missing_meta: Option<MissingMeta>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "CacheControl")]
    pub cache_control: Option<CacheControl>,
    #[serde(rename = "ContentDisposition")]
    pub content_disposition: Option<ContentDisposition>,
    #[serde(rename = "ContentEncoding")]
    pub content_encoding: Option<ContentEncoding>,
    #[serde(rename = "ContentLanguage")]
    pub content_language: Option<ContentLanguage>,
    #[serde(rename = "ContentRange")]
    pub content_range: Option<ContentRange>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<ContentType>,
    #[serde(rename = "Expires")]
    pub expires: Option<Expires>,
    #[serde(rename = "WebsiteRedirectLocation")]
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
    #[serde(rename = "ReplicationStatus")]
    pub replication_status: Option<ReplicationStatus>,
    #[serde(rename = "PartsCount")]
    pub parts_count: Option<PartsCount>,
    #[serde(rename = "TagCount")]
    pub tag_count: Option<TagCount>,
    #[serde(rename = "ObjectLockMode")]
    pub object_lock_mode: Option<ObjectLockMode>,
    #[serde(rename = "ObjectLockRetainUntilDate")]
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    #[serde(rename = "ObjectLockLegalHoldStatus")]
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "IfMatch")]
    pub if_match: IfMatch,
    #[serde(rename = "IfModifiedSince")]
    pub if_modified_since: IfModifiedSince,
    #[serde(rename = "IfNoneMatch")]
    pub if_none_match: IfNoneMatch,
    #[serde(rename = "IfUnmodifiedSince")]
    pub if_unmodified_since: IfUnmodifiedSince,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Range")]
    pub range: Range,
    #[serde(rename = "ResponseCacheControl")]
    pub response_cache_control: ResponseCacheControl,
    #[serde(rename = "ResponseContentDisposition")]
    pub response_content_disposition: ResponseContentDisposition,
    #[serde(rename = "ResponseContentEncoding")]
    pub response_content_encoding: ResponseContentEncoding,
    #[serde(rename = "ResponseContentLanguage")]
    pub response_content_language: ResponseContentLanguage,
    #[serde(rename = "ResponseContentType")]
    pub response_content_type: ResponseContentType,
    #[serde(rename = "ResponseExpires")]
    pub response_expires: ResponseExpires,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: SSECustomerKey,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "PartNumber")]
    pub part_number: PartNumber,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectRetentionOutput {
    #[serde(rename = "Retention")]
    pub retention: Option<ObjectLockRetention>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectRetentionRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectTaggingOutput {
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectTorrentOutput {
    #[serde(rename = "Body")]
    pub body: Option<Body>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectTorrentRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPublicAccessBlockOutput {
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPublicAccessBlockRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlacierJobParameters {
    #[serde(rename = "Tier")]
    pub tier: Tier,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grant {
    #[serde(rename = "Grantee")]
    pub grantee: Option<Grantee>,
    #[serde(rename = "Permission")]
    pub permission: Option<Permission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grantee {
    #[serde(rename = "DisplayName")]
    pub display_name: DisplayName,
    #[serde(rename = "EmailAddress")]
    pub email_address: EmailAddress,
    #[serde(rename = "ID")]
    pub id: ID,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "URI")]
    pub uri: URI,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeadBucketRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeadObjectOutput {
    #[serde(rename = "DeleteMarker")]
    pub delete_marker: Option<DeleteMarker>,
    #[serde(rename = "AcceptRanges")]
    pub accept_ranges: Option<AcceptRanges>,
    #[serde(rename = "Expiration")]
    pub expiration: Option<Expiration>,
    #[serde(rename = "Restore")]
    pub restore: Option<Restore>,
    #[serde(rename = "ArchiveStatus")]
    pub archive_status: Option<ArchiveStatus>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<LastModified>,
    #[serde(rename = "ContentLength")]
    pub content_length: Option<ContentLength>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "MissingMeta")]
    pub missing_meta: Option<MissingMeta>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "CacheControl")]
    pub cache_control: Option<CacheControl>,
    #[serde(rename = "ContentDisposition")]
    pub content_disposition: Option<ContentDisposition>,
    #[serde(rename = "ContentEncoding")]
    pub content_encoding: Option<ContentEncoding>,
    #[serde(rename = "ContentLanguage")]
    pub content_language: Option<ContentLanguage>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<ContentType>,
    #[serde(rename = "Expires")]
    pub expires: Option<Expires>,
    #[serde(rename = "WebsiteRedirectLocation")]
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
    #[serde(rename = "ReplicationStatus")]
    pub replication_status: Option<ReplicationStatus>,
    #[serde(rename = "PartsCount")]
    pub parts_count: Option<PartsCount>,
    #[serde(rename = "ObjectLockMode")]
    pub object_lock_mode: Option<ObjectLockMode>,
    #[serde(rename = "ObjectLockRetainUntilDate")]
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    #[serde(rename = "ObjectLockLegalHoldStatus")]
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeadObjectRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "IfMatch")]
    pub if_match: IfMatch,
    #[serde(rename = "IfModifiedSince")]
    pub if_modified_since: IfModifiedSince,
    #[serde(rename = "IfNoneMatch")]
    pub if_none_match: IfNoneMatch,
    #[serde(rename = "IfUnmodifiedSince")]
    pub if_unmodified_since: IfUnmodifiedSince,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Range")]
    pub range: Range,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: SSECustomerKey,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "PartNumber")]
    pub part_number: PartNumber,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexDocument {
    #[serde(rename = "Suffix")]
    pub suffix: Suffix,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Initiator {
    #[serde(rename = "ID")]
    pub id: Option<ID>,
    #[serde(rename = "DisplayName")]
    pub display_name: Option<DisplayName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputSerialization {
    #[serde(rename = "CSV")]
    pub csv: Option<CSVInput>,
    #[serde(rename = "CompressionType")]
    pub compression_type: Option<CompressionType>,
    #[serde(rename = "JSON")]
    pub json: Option<JSONInput>,
    #[serde(rename = "Parquet")]
    pub parquet: Option<ParquetInput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntelligentTieringAndOperator {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagSet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntelligentTieringConfiguration {
    #[serde(rename = "Id")]
    pub id: IntelligentTieringId,
    #[serde(rename = "Filter")]
    pub filter: IntelligentTieringFilter,
    #[serde(rename = "Status")]
    pub status: IntelligentTieringStatus,
    #[serde(rename = "Tierings")]
    pub tierings: TieringList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntelligentTieringFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tag")]
    pub tag: Option<Tag>,
    #[serde(rename = "And")]
    pub and: Option<IntelligentTieringAndOperator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvalidObjectState {
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    #[serde(rename = "AccessTier")]
    pub access_tier: Option<IntelligentTieringAccessTier>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryConfiguration {
    #[serde(rename = "Destination")]
    pub destination: InventoryDestination,
    #[serde(rename = "IsEnabled")]
    pub is_enabled: IsEnabled,
    #[serde(rename = "Filter")]
    pub filter: InventoryFilter,
    #[serde(rename = "Id")]
    pub id: InventoryId,
    #[serde(rename = "IncludedObjectVersions")]
    pub included_object_versions: InventoryIncludedObjectVersions,
    #[serde(rename = "OptionalFields")]
    pub optional_fields: InventoryOptionalFields,
    #[serde(rename = "Schedule")]
    pub schedule: InventorySchedule,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryDestination {
    #[serde(rename = "S3BucketDestination")]
    pub s3_bucket_destination: InventoryS3BucketDestination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryEncryption {
    #[serde(rename = "SSES3")]
    pub sses3: Option<SSES3>,
    #[serde(rename = "SSEKMS")]
    pub ssekms: Option<SSEKMS>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryS3BucketDestination {
    #[serde(rename = "AccountId")]
    pub account_id: AccountId,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Format")]
    pub format: InventoryFormat,
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
    #[serde(rename = "Encryption")]
    pub encryption: InventoryEncryption,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventorySchedule {
    #[serde(rename = "Frequency")]
    pub frequency: InventoryFrequency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONInput {
    #[serde(rename = "json_type")]
    pub json_type: Option<JSONType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONOutput {
    #[serde(rename = "RecordDelimiter")]
    pub record_delimiter: Option<RecordDelimiter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LambdaFunctionConfiguration {
    #[serde(rename = "Id")]
    pub id: NotificationId,
    #[serde(rename = "LambdaFunctionArn")]
    pub lambda_function_arn: LambdaFunctionArn,
    #[serde(rename = "Events")]
    pub events: EventList,
    #[serde(rename = "Filter")]
    pub filter: NotificationConfigurationFilter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleConfiguration {
    #[serde(rename = "Rules")]
    pub rules: Rules,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleExpiration {
    #[serde(rename = "Date")]
    pub date: Option<Date>,
    #[serde(rename = "Days")]
    pub days: Option<Days>,
    #[serde(rename = "ExpiredObjectDeleteMarker")]
    pub expired_object_delete_marker: Option<ExpiredObjectDeleteMarker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleRule {
    #[serde(rename = "Expiration")]
    pub expiration: LifecycleExpiration,
    #[serde(rename = "ID")]
    pub id: ID,
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
    #[serde(rename = "Filter")]
    pub filter: LifecycleRuleFilter,
    #[serde(rename = "Status")]
    pub status: ExpirationStatus,
    #[serde(rename = "Transitions")]
    pub transitions: TransitionList,
    #[serde(rename = "NoncurrentVersionTransitions")]
    pub noncurrent_version_transitions: NoncurrentVersionTransitionList,
    #[serde(rename = "NoncurrentVersionExpiration")]
    pub noncurrent_version_expiration: NoncurrentVersionExpiration,
    #[serde(rename = "AbortIncompleteMultipartUpload")]
    pub abort_incomplete_multipart_upload: AbortIncompleteMultipartUpload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleRuleAndOperator {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagSet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleRuleFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tag")]
    pub tag: Option<Tag>,
    #[serde(rename = "And")]
    pub and: Option<LifecycleRuleAndOperator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBucketAnalyticsConfigurationsOutput {
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<IsTruncated>,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<Token>,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: Option<NextToken>,
    #[serde(rename = "AnalyticsConfigurationList")]
    pub analytics_configuration_list: Option<AnalyticsConfigurationList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBucketAnalyticsConfigurationsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Token,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBucketIntelligentTieringConfigurationsOutput {
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<IsTruncated>,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<Token>,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: Option<NextToken>,
    #[serde(rename = "IntelligentTieringConfigurationList")]
    pub intelligent_tiering_configuration_list: Option<IntelligentTieringConfigurationList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBucketIntelligentTieringConfigurationsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Token,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBucketInventoryConfigurationsOutput {
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<Token>,
    #[serde(rename = "InventoryConfigurationList")]
    pub inventory_configuration_list: Option<InventoryConfigurationList>,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<IsTruncated>,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: Option<NextToken>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBucketInventoryConfigurationsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Token,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBucketMetricsConfigurationsOutput {
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<IsTruncated>,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<Token>,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: Option<NextToken>,
    #[serde(rename = "MetricsConfigurationList")]
    pub metrics_configuration_list: Option<MetricsConfigurationList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBucketMetricsConfigurationsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Token,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBucketsOutput {
    #[serde(rename = "Buckets")]
    pub buckets: Option<Buckets>,
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMultipartUploadsOutput {
    #[serde(rename = "Bucket")]
    pub bucket: Option<BucketName>,
    #[serde(rename = "KeyMarker")]
    pub key_marker: Option<KeyMarker>,
    #[serde(rename = "UploadIdMarker")]
    pub upload_id_marker: Option<UploadIdMarker>,
    #[serde(rename = "NextKeyMarker")]
    pub next_key_marker: Option<NextKeyMarker>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<Delimiter>,
    #[serde(rename = "NextUploadIdMarker")]
    pub next_upload_id_marker: Option<NextUploadIdMarker>,
    #[serde(rename = "MaxUploads")]
    pub max_uploads: Option<MaxUploads>,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<IsTruncated>,
    #[serde(rename = "Uploads")]
    pub uploads: Option<MultipartUploadList>,
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Option<CommonPrefixList>,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<EncodingType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMultipartUploadsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Delimiter")]
    pub delimiter: Delimiter,
    #[serde(rename = "EncodingType")]
    pub encoding_type: EncodingType,
    #[serde(rename = "KeyMarker")]
    pub key_marker: KeyMarker,
    #[serde(rename = "MaxUploads")]
    pub max_uploads: MaxUploads,
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
    #[serde(rename = "UploadIdMarker")]
    pub upload_id_marker: UploadIdMarker,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListObjectVersionsOutput {
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<IsTruncated>,
    #[serde(rename = "KeyMarker")]
    pub key_marker: Option<KeyMarker>,
    #[serde(rename = "VersionIdMarker")]
    pub version_id_marker: Option<VersionIdMarker>,
    #[serde(rename = "NextKeyMarker")]
    pub next_key_marker: Option<NextKeyMarker>,
    #[serde(rename = "NextVersionIdMarker")]
    pub next_version_id_marker: Option<NextVersionIdMarker>,
    #[serde(rename = "Versions")]
    pub versions: Option<ObjectVersionList>,
    #[serde(rename = "DeleteMarkers")]
    pub delete_markers: Option<DeleteMarkers>,
    #[serde(rename = "Name")]
    pub name: Option<BucketName>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<Delimiter>,
    #[serde(rename = "MaxKeys")]
    pub max_keys: Option<MaxKeys>,
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Option<CommonPrefixList>,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<EncodingType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListObjectVersionsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Delimiter")]
    pub delimiter: Delimiter,
    #[serde(rename = "EncodingType")]
    pub encoding_type: EncodingType,
    #[serde(rename = "KeyMarker")]
    pub key_marker: KeyMarker,
    #[serde(rename = "MaxKeys")]
    pub max_keys: MaxKeys,
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
    #[serde(rename = "VersionIdMarker")]
    pub version_id_marker: VersionIdMarker,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListObjectsOutput {
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<IsTruncated>,
    #[serde(rename = "Marker")]
    pub marker: Option<Marker>,
    #[serde(rename = "NextMarker")]
    pub next_marker: Option<NextMarker>,
    #[serde(rename = "Contents")]
    pub contents: Option<ObjectList>,
    #[serde(rename = "Name")]
    pub name: Option<BucketName>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<Delimiter>,
    #[serde(rename = "MaxKeys")]
    pub max_keys: Option<MaxKeys>,
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Option<CommonPrefixList>,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<EncodingType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListObjectsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Delimiter")]
    pub delimiter: Delimiter,
    #[serde(rename = "EncodingType")]
    pub encoding_type: EncodingType,
    #[serde(rename = "Marker")]
    pub marker: Marker,
    #[serde(rename = "MaxKeys")]
    pub max_keys: MaxKeys,
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListObjectsV2Output {
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<IsTruncated>,
    #[serde(rename = "Contents")]
    pub contents: Option<ObjectList>,
    #[serde(rename = "Name")]
    pub name: Option<BucketName>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<Delimiter>,
    #[serde(rename = "MaxKeys")]
    pub max_keys: Option<MaxKeys>,
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Option<CommonPrefixList>,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<EncodingType>,
    #[serde(rename = "KeyCount")]
    pub key_count: Option<KeyCount>,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<Token>,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: Option<NextToken>,
    #[serde(rename = "StartAfter")]
    pub start_after: Option<StartAfter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListObjectsV2Request {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Delimiter")]
    pub delimiter: Delimiter,
    #[serde(rename = "EncodingType")]
    pub encoding_type: EncodingType,
    #[serde(rename = "MaxKeys")]
    pub max_keys: MaxKeys,
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Token,
    #[serde(rename = "FetchOwner")]
    pub fetch_owner: FetchOwner,
    #[serde(rename = "StartAfter")]
    pub start_after: StartAfter,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPartsOutput {
    #[serde(rename = "AbortDate")]
    pub abort_date: Option<AbortDate>,
    #[serde(rename = "AbortRuleId")]
    pub abort_rule_id: Option<AbortRuleId>,
    #[serde(rename = "Bucket")]
    pub bucket: Option<BucketName>,
    #[serde(rename = "Key")]
    pub key: Option<ObjectKey>,
    #[serde(rename = "UploadId")]
    pub upload_id: Option<MultipartUploadId>,
    #[serde(rename = "PartNumberMarker")]
    pub part_number_marker: Option<PartNumberMarker>,
    #[serde(rename = "NextPartNumberMarker")]
    pub next_part_number_marker: Option<NextPartNumberMarker>,
    #[serde(rename = "MaxParts")]
    pub max_parts: Option<MaxParts>,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: Option<IsTruncated>,
    #[serde(rename = "Parts")]
    pub parts: Option<Parts>,
    #[serde(rename = "Initiator")]
    pub initiator: Option<Initiator>,
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPartsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "MaxParts")]
    pub max_parts: MaxParts,
    #[serde(rename = "PartNumberMarker")]
    pub part_number_marker: PartNumberMarker,
    #[serde(rename = "UploadId")]
    pub upload_id: MultipartUploadId,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingEnabled {
    #[serde(rename = "TargetBucket")]
    pub target_bucket: TargetBucket,
    #[serde(rename = "TargetGrants")]
    pub target_grants: TargetGrants,
    #[serde(rename = "TargetPrefix")]
    pub target_prefix: TargetPrefix,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataEntry {
    #[serde(rename = "Name")]
    pub name: Option<MetadataKey>,
    #[serde(rename = "Value")]
    pub value: Option<MetadataValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    #[serde(rename = "Status")]
    pub status: MetricsStatus,
    #[serde(rename = "EventThreshold")]
    pub event_threshold: ReplicationTimeValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsAndOperator {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagSet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsConfiguration {
    #[serde(rename = "Id")]
    pub id: MetricsId,
    #[serde(rename = "Filter")]
    pub filter: MetricsFilter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tag")]
    pub tag: Option<Tag>,
    #[serde(rename = "And")]
    pub and: Option<MetricsAndOperator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultipartUpload {
    #[serde(rename = "UploadId")]
    pub upload_id: Option<MultipartUploadId>,
    #[serde(rename = "Key")]
    pub key: Option<ObjectKey>,
    #[serde(rename = "Initiated")]
    pub initiated: Option<Initiated>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    #[serde(rename = "Initiator")]
    pub initiator: Option<Initiator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoSuchBucket {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoSuchKey {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoSuchUpload {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoncurrentVersionExpiration {
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: Option<Days>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoncurrentVersionTransition {
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: Option<Days>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<TransitionStorageClass>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationConfiguration {
    #[serde(rename = "TopicConfigurations")]
    pub topic_configurations: Option<TopicConfigurationList>,
    #[serde(rename = "QueueConfigurations")]
    pub queue_configurations: Option<QueueConfigurationList>,
    #[serde(rename = "LambdaFunctionConfigurations")]
    pub lambda_function_configurations: Option<LambdaFunctionConfigurationList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationConfigurationDeprecated {
    #[serde(rename = "TopicConfiguration")]
    pub topic_configuration: Option<TopicConfigurationDeprecated>,
    #[serde(rename = "QueueConfiguration")]
    pub queue_configuration: Option<QueueConfigurationDeprecated>,
    #[serde(rename = "CloudFunctionConfiguration")]
    pub cloud_function_configuration: Option<CloudFunctionConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationConfigurationFilter {
    #[serde(rename = "Key")]
    pub key: Option<S3KeyFilter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    #[serde(rename = "Key")]
    pub key: Option<ObjectKey>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<LastModified>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "Size")]
    pub size: Option<Size>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<ObjectStorageClass>,
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectAlreadyInActiveTierError {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectIdentifier {
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectLockConfiguration {
    #[serde(rename = "ObjectLockEnabled")]
    pub object_lock_enabled: Option<ObjectLockEnabled>,
    #[serde(rename = "Rule")]
    pub rule: Option<ObjectLockRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectLockLegalHold {
    #[serde(rename = "Status")]
    pub status: Option<ObjectLockLegalHoldStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectLockRetention {
    #[serde(rename = "Mode")]
    pub mode: Option<ObjectLockRetentionMode>,
    #[serde(rename = "RetainUntilDate")]
    pub retain_until_date: Option<Date>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectLockRule {
    #[serde(rename = "DefaultRetention")]
    pub default_retention: Option<DefaultRetention>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectNotInActiveTierError {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectVersion {
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "Size")]
    pub size: Option<Size>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<ObjectVersionStorageClass>,
    #[serde(rename = "Key")]
    pub key: Option<ObjectKey>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "IsLatest")]
    pub is_latest: Option<IsLatest>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<LastModified>,
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputLocation {
    #[serde(rename = "S3")]
    pub s3: Option<S3Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputSerialization {
    #[serde(rename = "CSV")]
    pub csv: Option<CSVOutput>,
    #[serde(rename = "JSON")]
    pub json: Option<JSONOutput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "DisplayName")]
    pub display_name: Option<DisplayName>,
    #[serde(rename = "ID")]
    pub id: Option<ID>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnershipControls {
    #[serde(rename = "Rules")]
    pub rules: OwnershipControlsRules,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnershipControlsRule {
    #[serde(rename = "ObjectOwnership")]
    pub object_ownership: ObjectOwnership,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParquetInput {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    #[serde(rename = "PartNumber")]
    pub part_number: Option<PartNumber>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<LastModified>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "Size")]
    pub size: Option<Size>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyStatus {
    #[serde(rename = "IsPublic")]
    pub is_public: Option<IsPublic>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    #[serde(rename = "BytesScanned")]
    pub bytes_scanned: Option<BytesScanned>,
    #[serde(rename = "BytesProcessed")]
    pub bytes_processed: Option<BytesProcessed>,
    #[serde(rename = "BytesReturned")]
    pub bytes_returned: Option<BytesReturned>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressEvent {
    #[serde(rename = "Details")]
    pub details: Option<Progress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicAccessBlockConfiguration {
    #[serde(rename = "BlockPublicAcls")]
    pub block_public_acls: Option<Setting>,
    #[serde(rename = "IgnorePublicAcls")]
    pub ignore_public_acls: Option<Setting>,
    #[serde(rename = "BlockPublicPolicy")]
    pub block_public_policy: Option<Setting>,
    #[serde(rename = "RestrictPublicBuckets")]
    pub restrict_public_buckets: Option<Setting>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketAccelerateConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "AccelerateConfiguration")]
    pub accelerate_configuration: AccelerateConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketAclRequest {
    #[serde(rename = "ACL")]
    pub acl: BucketCannedACL,
    #[serde(rename = "AccessControlPolicy")]
    pub access_control_policy: AccessControlPolicy,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: GrantFullControl,
    #[serde(rename = "GrantRead")]
    pub grant_read: GrantRead,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: GrantReadACP,
    #[serde(rename = "GrantWrite")]
    pub grant_write: GrantWrite,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: GrantWriteACP,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketAnalyticsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: AnalyticsId,
    #[serde(rename = "AnalyticsConfiguration")]
    pub analytics_configuration: AnalyticsConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketCorsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "CORSConfiguration")]
    pub cors_configuration: CORSConfiguration,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketEncryptionRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    pub server_side_encryption_configuration: ServerSideEncryptionConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketIntelligentTieringConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: IntelligentTieringId,
    #[serde(rename = "IntelligentTieringConfiguration")]
    pub intelligent_tiering_configuration: IntelligentTieringConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketInventoryConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: InventoryId,
    #[serde(rename = "InventoryConfiguration")]
    pub inventory_configuration: InventoryConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketLifecycleConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "LifecycleConfiguration")]
    pub lifecycle_configuration: BucketLifecycleConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketLifecycleRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "LifecycleConfiguration")]
    pub lifecycle_configuration: LifecycleConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketLoggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "BucketLoggingStatus")]
    pub bucket_logging_status: BucketLoggingStatus,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketMetricsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: MetricsId,
    #[serde(rename = "MetricsConfiguration")]
    pub metrics_configuration: MetricsConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketNotificationConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "NotificationConfiguration")]
    pub notification_configuration: NotificationConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketNotificationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "NotificationConfiguration")]
    pub notification_configuration: NotificationConfigurationDeprecated,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketOwnershipControlsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
    #[serde(rename = "OwnershipControls")]
    pub ownership_controls: OwnershipControls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketPolicyRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "ConfirmRemoveSelfBucketAccess")]
    pub confirm_remove_self_bucket_access: ConfirmRemoveSelfBucketAccess,
    #[serde(rename = "Policy")]
    pub policy: Policy,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketReplicationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "ReplicationConfiguration")]
    pub replication_configuration: ReplicationConfiguration,
    #[serde(rename = "Token")]
    pub token: ObjectLockToken,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketRequestPaymentRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "RequestPaymentConfiguration")]
    pub request_payment_configuration: RequestPaymentConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "Tagging")]
    pub tagging: Tagging,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketVersioningRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "MFA")]
    pub mfa: MFA,
    #[serde(rename = "VersioningConfiguration")]
    pub versioning_configuration: VersioningConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutBucketWebsiteRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "WebsiteConfiguration")]
    pub website_configuration: WebsiteConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectAclOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectAclRequest {
    #[serde(rename = "ACL")]
    pub acl: ObjectCannedACL,
    #[serde(rename = "AccessControlPolicy")]
    pub access_control_policy: AccessControlPolicy,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: GrantFullControl,
    #[serde(rename = "GrantRead")]
    pub grant_read: GrantRead,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: GrantReadACP,
    #[serde(rename = "GrantWrite")]
    pub grant_write: GrantWrite,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: GrantWriteACP,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectLegalHoldOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectLegalHoldRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "LegalHold")]
    pub legal_hold: ObjectLockLegalHold,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectLockConfigurationOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectLockConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ObjectLockConfiguration")]
    pub object_lock_configuration: ObjectLockConfiguration,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "Token")]
    pub token: ObjectLockToken,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectOutput {
    #[serde(rename = "Expiration")]
    pub expiration: Option<Expiration>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "SSEKMSEncryptionContext")]
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectRequest {
    #[serde(rename = "ACL")]
    pub acl: ObjectCannedACL,
    #[serde(rename = "Body")]
    pub body: Body,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "CacheControl")]
    pub cache_control: CacheControl,
    #[serde(rename = "ContentDisposition")]
    pub content_disposition: ContentDisposition,
    #[serde(rename = "ContentEncoding")]
    pub content_encoding: ContentEncoding,
    #[serde(rename = "ContentLanguage")]
    pub content_language: ContentLanguage,
    #[serde(rename = "ContentLength")]
    pub content_length: ContentLength,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "ContentType")]
    pub content_type: ContentType,
    #[serde(rename = "Expires")]
    pub expires: Expires,
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: GrantFullControl,
    #[serde(rename = "GrantRead")]
    pub grant_read: GrantRead,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: GrantReadACP,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: GrantWriteACP,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Metadata")]
    pub metadata: Metadata,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: ServerSideEncryption,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "WebsiteRedirectLocation")]
    pub website_redirect_location: WebsiteRedirectLocation,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: SSECustomerKey,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: SSEKMSKeyId,
    #[serde(rename = "SSEKMSEncryptionContext")]
    pub ssekms_encryption_context: SSEKMSEncryptionContext,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: BucketKeyEnabled,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "Tagging")]
    pub tagging: TaggingHeader,
    #[serde(rename = "ObjectLockMode")]
    pub object_lock_mode: ObjectLockMode,
    #[serde(rename = "ObjectLockRetainUntilDate")]
    pub object_lock_retain_until_date: ObjectLockRetainUntilDate,
    #[serde(rename = "ObjectLockLegalHoldStatus")]
    pub object_lock_legal_hold_status: ObjectLockLegalHoldStatus,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectRetentionOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectRetentionRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Retention")]
    pub retention: ObjectLockRetention,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "BypassGovernanceRetention")]
    pub bypass_governance_retention: BypassGovernanceRetention,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectTaggingOutput {
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutObjectTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "Tagging")]
    pub tagging: Tagging,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutPublicAccessBlockRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueConfiguration {
    #[serde(rename = "Id")]
    pub id: NotificationId,
    #[serde(rename = "QueueArn")]
    pub queue_arn: QueueArn,
    #[serde(rename = "Events")]
    pub events: EventList,
    #[serde(rename = "Filter")]
    pub filter: NotificationConfigurationFilter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueConfigurationDeprecated {
    #[serde(rename = "Id")]
    pub id: Option<NotificationId>,
    #[serde(rename = "Event")]
    pub event: Option<Event>,
    #[serde(rename = "Events")]
    pub events: Option<EventList>,
    #[serde(rename = "Queue")]
    pub queue: Option<QueueArn>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordsEvent {
    #[serde(rename = "Payload")]
    pub payload: Option<Body>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Redirect {
    #[serde(rename = "HostName")]
    pub host_name: Option<HostName>,
    #[serde(rename = "HttpRedirectCode")]
    pub http_redirect_code: Option<HttpRedirectCode>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<Protocol>,
    #[serde(rename = "ReplaceKeyPrefixWith")]
    pub replace_key_prefix_with: Option<ReplaceKeyPrefixWith>,
    #[serde(rename = "ReplaceKeyWith")]
    pub replace_key_with: Option<ReplaceKeyWith>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedirectAllRequestsTo {
    #[serde(rename = "HostName")]
    pub host_name: HostName,
    #[serde(rename = "Protocol")]
    pub protocol: Protocol,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicaModifications {
    #[serde(rename = "Status")]
    pub status: ReplicaModificationsStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationConfiguration {
    #[serde(rename = "Role")]
    pub role: Role,
    #[serde(rename = "Rules")]
    pub rules: ReplicationRules,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationRule {
    #[serde(rename = "ID")]
    pub id: ID,
    #[serde(rename = "Priority")]
    pub priority: Priority,
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
    #[serde(rename = "Filter")]
    pub filter: ReplicationRuleFilter,
    #[serde(rename = "Status")]
    pub status: ReplicationRuleStatus,
    #[serde(rename = "SourceSelectionCriteria")]
    pub source_selection_criteria: SourceSelectionCriteria,
    #[serde(rename = "ExistingObjectReplication")]
    pub existing_object_replication: ExistingObjectReplication,
    #[serde(rename = "Destination")]
    pub destination: Destination,
    #[serde(rename = "DeleteMarkerReplication")]
    pub delete_marker_replication: DeleteMarkerReplication,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationRuleAndOperator {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagSet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationRuleFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tag")]
    pub tag: Option<Tag>,
    #[serde(rename = "And")]
    pub and: Option<ReplicationRuleAndOperator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationTime {
    #[serde(rename = "Status")]
    pub status: ReplicationTimeStatus,
    #[serde(rename = "Time")]
    pub time: ReplicationTimeValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationTimeValue {
    #[serde(rename = "Minutes")]
    pub minutes: Option<Minutes>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestPaymentConfiguration {
    #[serde(rename = "Payer")]
    pub payer: Payer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestProgress {
    #[serde(rename = "Enabled")]
    pub enabled: Option<EnableRequestProgress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreObjectOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
    #[serde(rename = "RestoreOutputPath")]
    pub restore_output_path: Option<RestoreOutputPath>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreObjectRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: ObjectVersionId,
    #[serde(rename = "RestoreRequest")]
    pub restore_request: RestoreRequest,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreRequest {
    #[serde(rename = "Days")]
    pub days: Option<Days>,
    #[serde(rename = "GlacierJobParameters")]
    pub glacier_job_parameters: Option<GlacierJobParameters>,
    #[serde(rename = "restore_request_type")]
    pub restore_request_type: Option<RestoreRequestType>,
    #[serde(rename = "Tier")]
    pub tier: Option<Tier>,
    #[serde(rename = "Description")]
    pub description: Option<Description>,
    #[serde(rename = "SelectParameters")]
    pub select_parameters: Option<SelectParameters>,
    #[serde(rename = "OutputLocation")]
    pub output_location: Option<OutputLocation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoutingRule {
    #[serde(rename = "Condition")]
    pub condition: Condition,
    #[serde(rename = "Redirect")]
    pub redirect: Redirect,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    #[serde(rename = "Expiration")]
    pub expiration: LifecycleExpiration,
    #[serde(rename = "ID")]
    pub id: ID,
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
    #[serde(rename = "Status")]
    pub status: ExpirationStatus,
    #[serde(rename = "Transition")]
    pub transition: Transition,
    #[serde(rename = "NoncurrentVersionTransition")]
    pub noncurrent_version_transition: NoncurrentVersionTransition,
    #[serde(rename = "NoncurrentVersionExpiration")]
    pub noncurrent_version_expiration: NoncurrentVersionExpiration,
    #[serde(rename = "AbortIncompleteMultipartUpload")]
    pub abort_incomplete_multipart_upload: AbortIncompleteMultipartUpload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct S3KeyFilter {
    #[serde(rename = "FilterRules")]
    pub filter_rules: Option<FilterRuleList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct S3Location {
    #[serde(rename = "BucketName")]
    pub bucket_name: BucketName,
    #[serde(rename = "Prefix")]
    pub prefix: LocationPrefix,
    #[serde(rename = "Encryption")]
    pub encryption: Encryption,
    #[serde(rename = "CannedACL")]
    pub canned_acl: ObjectCannedACL,
    #[serde(rename = "AccessControlList")]
    pub access_control_list: Grants,
    #[serde(rename = "Tagging")]
    pub tagging: Tagging,
    #[serde(rename = "UserMetadata")]
    pub user_metadata: UserMetadata,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SSEKMS {
    #[serde(rename = "KeyId")]
    pub key_id: SSEKMSKeyId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SSES3 {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanRange {
    #[serde(rename = "Start")]
    pub start: Option<Start>,
    #[serde(rename = "End")]
    pub end: Option<End>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectObjectContentEventStream {
    #[serde(rename = "Records")]
    pub records: Option<RecordsEvent>,
    #[serde(rename = "Stats")]
    pub stats: Option<StatsEvent>,
    #[serde(rename = "Progress")]
    pub progress: Option<ProgressEvent>,
    #[serde(rename = "Cont")]
    pub cont: Option<ContinuationEvent>,
    #[serde(rename = "End")]
    pub end: Option<EndEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectObjectContentOutput {
    #[serde(rename = "Payload")]
    pub payload: Option<SelectObjectContentEventStream>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectObjectContentRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: SSECustomerKey,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    #[serde(rename = "Expression")]
    pub expression: Expression,
    #[serde(rename = "ExpressionType")]
    pub expression_type: ExpressionType,
    #[serde(rename = "RequestProgress")]
    pub request_progress: RequestProgress,
    #[serde(rename = "InputSerialization")]
    pub input_serialization: InputSerialization,
    #[serde(rename = "OutputSerialization")]
    pub output_serialization: OutputSerialization,
    #[serde(rename = "ScanRange")]
    pub scan_range: ScanRange,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectParameters {
    #[serde(rename = "InputSerialization")]
    pub input_serialization: InputSerialization,
    #[serde(rename = "ExpressionType")]
    pub expression_type: ExpressionType,
    #[serde(rename = "Expression")]
    pub expression: Expression,
    #[serde(rename = "OutputSerialization")]
    pub output_serialization: OutputSerialization,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSideEncryptionByDefault {
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: ServerSideEncryption,
    #[serde(rename = "KMSMasterKeyID")]
    pub kms_master_key_id: SSEKMSKeyId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSideEncryptionConfiguration {
    #[serde(rename = "Rules")]
    pub rules: ServerSideEncryptionRules,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSideEncryptionRule {
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceSelectionCriteria {
    #[serde(rename = "SseKmsEncryptedObjects")]
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
    #[serde(rename = "ReplicaModifications")]
    pub replica_modifications: Option<ReplicaModifications>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SseKmsEncryptedObjects {
    #[serde(rename = "Status")]
    pub status: SseKmsEncryptedObjectsStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "BytesScanned")]
    pub bytes_scanned: Option<BytesScanned>,
    #[serde(rename = "BytesProcessed")]
    pub bytes_processed: Option<BytesProcessed>,
    #[serde(rename = "BytesReturned")]
    pub bytes_returned: Option<BytesReturned>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsEvent {
    #[serde(rename = "Details")]
    pub details: Option<Stats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageClassAnalysis {
    #[serde(rename = "DataExport")]
    pub data_export: Option<StorageClassAnalysisDataExport>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageClassAnalysisDataExport {
    #[serde(rename = "OutputSchemaVersion")]
    pub output_schema_version: StorageClassAnalysisSchemaVersion,
    #[serde(rename = "Destination")]
    pub destination: AnalyticsExportDestination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Value")]
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tagging {
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetGrant {
    #[serde(rename = "Grantee")]
    pub grantee: Option<Grantee>,
    #[serde(rename = "Permission")]
    pub permission: Option<BucketLogsPermission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tiering {
    #[serde(rename = "Days")]
    pub days: IntelligentTieringDays,
    #[serde(rename = "AccessTier")]
    pub access_tier: IntelligentTieringAccessTier,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicConfiguration {
    #[serde(rename = "Id")]
    pub id: NotificationId,
    #[serde(rename = "TopicArn")]
    pub topic_arn: TopicArn,
    #[serde(rename = "Events")]
    pub events: EventList,
    #[serde(rename = "Filter")]
    pub filter: NotificationConfigurationFilter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicConfigurationDeprecated {
    #[serde(rename = "Id")]
    pub id: Option<NotificationId>,
    #[serde(rename = "Events")]
    pub events: Option<EventList>,
    #[serde(rename = "Event")]
    pub event: Option<Event>,
    #[serde(rename = "Topic")]
    pub topic: Option<TopicArn>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transition {
    #[serde(rename = "Date")]
    pub date: Option<Date>,
    #[serde(rename = "Days")]
    pub days: Option<Days>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<TransitionStorageClass>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadPartCopyOutput {
    #[serde(rename = "CopySourceVersionId")]
    pub copy_source_version_id: Option<CopySourceVersionId>,
    #[serde(rename = "CopyPartResult")]
    pub copy_part_result: Option<CopyPartResult>,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadPartCopyRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "CopySource")]
    pub copy_source: CopySource,
    #[serde(rename = "CopySourceIfMatch")]
    pub copy_source_if_match: CopySourceIfMatch,
    #[serde(rename = "CopySourceIfModifiedSince")]
    pub copy_source_if_modified_since: CopySourceIfModifiedSince,
    #[serde(rename = "CopySourceIfNoneMatch")]
    pub copy_source_if_none_match: CopySourceIfNoneMatch,
    #[serde(rename = "CopySourceIfUnmodifiedSince")]
    pub copy_source_if_unmodified_since: CopySourceIfUnmodifiedSince,
    #[serde(rename = "CopySourceRange")]
    pub copy_source_range: CopySourceRange,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "PartNumber")]
    pub part_number: PartNumber,
    #[serde(rename = "UploadId")]
    pub upload_id: MultipartUploadId,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: SSECustomerKey,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    #[serde(rename = "CopySourceSSECustomerAlgorithm")]
    pub copy_source_sse_customer_algorithm: CopySourceSSECustomerAlgorithm,
    #[serde(rename = "CopySourceSSECustomerKey")]
    pub copy_source_sse_customer_key: CopySourceSSECustomerKey,
    #[serde(rename = "CopySourceSSECustomerKeyMD5")]
    pub copy_source_sse_customer_key_md5: CopySourceSSECustomerKeyMD5,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
    #[serde(rename = "ExpectedSourceBucketOwner")]
    pub expected_source_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadPartOutput {
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadPartRequest {
    #[serde(rename = "Body")]
    pub body: Body,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentLength")]
    pub content_length: ContentLength,
    #[serde(rename = "ContentMD5")]
    pub content_md5: ContentMD5,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "PartNumber")]
    pub part_number: PartNumber,
    #[serde(rename = "UploadId")]
    pub upload_id: MultipartUploadId,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: SSECustomerKey,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    #[serde(rename = "RequestPayer")]
    pub request_payer: RequestPayer,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: AccountId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersioningConfiguration {
    #[serde(rename = "MFADelete")]
    pub mfa_delete: Option<MFADelete>,
    #[serde(rename = "Status")]
    pub status: Option<BucketVersioningStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteConfiguration {
    #[serde(rename = "ErrorDocument")]
    pub error_document: Option<ErrorDocument>,
    #[serde(rename = "IndexDocument")]
    pub index_document: Option<IndexDocument>,
    #[serde(rename = "RedirectAllRequestsTo")]
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    #[serde(rename = "RoutingRules")]
    pub routing_rules: Option<RoutingRules>,
}

