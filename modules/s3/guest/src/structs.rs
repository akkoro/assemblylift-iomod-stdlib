// This file is generated!
// See https://github.com/akkoro/asml-aws-codegen

use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use xml;

use crate::xml_util::error::*;
use crate::xml_util::util::{
    deserialize_elements, find_start_element, skip_tree, write_characters_element, deserialize_primitive
};
use crate::xml_util::util::{Next, Peek, XmlParseError, XmlResponse};

pub type AbortDate = String;
pub type AbortRuleId = String;
pub type AcceptRanges = String;
pub type AccountId = String;
pub type AllowQuotedRecordDelimiter = bool;
pub type AllowedHeader = String;
pub type AllowedHeaders = Vec<AllowedHeader>;
#[allow(dead_code)]
pub struct AllowedHeadersDeserializer;
impl AllowedHeadersDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AllowedHeader>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "AllowedHeader" {
                obj.push(deserialize_primitive(tag_name, stack, Ok)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type AllowedMethod = String;
pub type AllowedMethods = Vec<AllowedMethod>;
#[allow(dead_code)]
pub struct AllowedMethodsDeserializer;
impl AllowedMethodsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AllowedMethod>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "AllowedMethod" {
                obj.push(deserialize_primitive(tag_name, stack, Ok)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type AllowedOrigin = String;
pub type AllowedOrigins = Vec<AllowedOrigin>;
#[allow(dead_code)]
pub struct AllowedOriginsDeserializer;
impl AllowedOriginsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AllowedOrigin>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "AllowedOrigin" {
                obj.push(deserialize_primitive(tag_name, stack, Ok)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type AnalyticsConfigurationList = Vec<AnalyticsConfiguration>;
#[allow(dead_code)]
pub struct AnalyticsConfigurationListDeserializer;
impl AnalyticsConfigurationListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AnalyticsConfiguration>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "AnalyticsConfiguration" {
                obj.push(AnalyticsConfigurationDeserializer::deserialize("AnalyticsConfiguration", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
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
#[allow(dead_code)]
pub struct BucketsDeserializer;
impl BucketsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Bucket>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Bucket" {
                obj.push(BucketDeserializer::deserialize("Bucket", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type BypassGovernanceRetention = bool;
pub type BytesProcessed = u64;
pub type BytesReturned = u64;
pub type BytesScanned = u64;
pub type CORSRules = Vec<CORSRule>;
#[allow(dead_code)]
pub struct CORSRulesDeserializer;
impl CORSRulesDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CORSRule>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CORSRule" {
                obj.push(CORSRuleDeserializer::deserialize("CORSRule", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type CacheControl = String;
pub type CloudFunction = String;
pub type CloudFunctionInvocationRole = String;
pub type Code = String;
pub type Comments = String;
pub type CommonPrefixList = Vec<CommonPrefix>;
#[allow(dead_code)]
pub struct CommonPrefixListDeserializer;
impl CommonPrefixListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CommonPrefix>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CommonPrefix" {
                obj.push(CommonPrefixDeserializer::deserialize("CommonPrefix", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type CompletedPartList = Vec<CompletedPart>;
#[allow(dead_code)]
pub struct CompletedPartListDeserializer;
impl CompletedPartListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CompletedPart>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CompletedPart" {
                obj.push(CompletedPartDeserializer::deserialize("CompletedPart", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
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
pub type Days = i64;
pub type DaysAfterInitiation = i64;
pub type DeleteMarker = bool;
pub type DeleteMarkerReplicationStatus = String;
pub type DeleteMarkerVersionId = String;
pub type DeleteMarkers = Vec<DeleteMarkerEntry>;
#[allow(dead_code)]
pub struct DeleteMarkersDeserializer;
impl DeleteMarkersDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DeleteMarkerEntry>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DeleteMarkerEntry" {
                obj.push(DeleteMarkerEntryDeserializer::deserialize("DeleteMarkerEntry", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type DeletedObjects = Vec<DeletedObject>;
#[allow(dead_code)]
pub struct DeletedObjectsDeserializer;
impl DeletedObjectsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DeletedObject>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "DeletedObject" {
                obj.push(DeletedObjectDeserializer::deserialize("DeletedObject", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type Delimiter = String;
pub type Description = String;
pub type DisplayName = String;
pub type ETag = String;
pub type EmailAddress = String;
pub type EnableRequestProgress = bool;
pub type EncodingType = String;
pub type End = u64;
pub type Errors = Vec<Error>;
#[allow(dead_code)]
pub struct ErrorsDeserializer;
impl ErrorsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Error>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Error" {
                obj.push(ErrorDeserializer::deserialize("Error", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type Event = String;
pub type EventList = Vec<Event>;
#[allow(dead_code)]
pub struct EventListDeserializer;
impl EventListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Event>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Event" {
                obj.push(deserialize_primitive(tag_name, stack, Ok)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type ExistingObjectReplicationStatus = String;
pub type Expiration = String;
pub type ExpirationStatus = String;
pub type ExpiredObjectDeleteMarker = bool;
pub type Expires = String;
pub type ExposeHeader = String;
pub type ExposeHeaders = Vec<ExposeHeader>;
#[allow(dead_code)]
pub struct ExposeHeadersDeserializer;
impl ExposeHeadersDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ExposeHeader>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ExposeHeader" {
                obj.push(deserialize_primitive(tag_name, stack, Ok)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type Expression = String;
pub type ExpressionType = String;
pub type FetchOwner = bool;
pub type FieldDelimiter = String;
pub type FileHeaderInfo = String;
pub type FilterRuleList = Vec<FilterRule>;
#[allow(dead_code)]
pub struct FilterRuleListDeserializer;
impl FilterRuleListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<FilterRule>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "FilterRule" {
                obj.push(FilterRuleDeserializer::deserialize("FilterRule", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type FilterRuleName = String;
pub type FilterRuleValue = String;
pub type GrantFullControl = String;
pub type GrantRead = String;
pub type GrantReadACP = String;
pub type GrantWrite = String;
pub type GrantWriteACP = String;
pub type Grants = Vec<Grant>;
#[allow(dead_code)]
pub struct GrantsDeserializer;
impl GrantsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Grant>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Grant" {
                obj.push(GrantDeserializer::deserialize("Grant", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
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
#[allow(dead_code)]
pub struct IntelligentTieringConfigurationListDeserializer;
impl IntelligentTieringConfigurationListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<IntelligentTieringConfiguration>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "IntelligentTieringConfiguration" {
                obj.push(IntelligentTieringConfigurationDeserializer::deserialize("IntelligentTieringConfiguration", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type IntelligentTieringDays = i64;
pub type IntelligentTieringId = String;
pub type IntelligentTieringStatus = String;
pub type InventoryConfigurationList = Vec<InventoryConfiguration>;
#[allow(dead_code)]
pub struct InventoryConfigurationListDeserializer;
impl InventoryConfigurationListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<InventoryConfiguration>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "InventoryConfiguration" {
                obj.push(InventoryConfigurationDeserializer::deserialize("InventoryConfiguration", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type InventoryFormat = String;
pub type InventoryFrequency = String;
pub type InventoryId = String;
pub type InventoryIncludedObjectVersions = String;
pub type InventoryOptionalField = String;
pub type InventoryOptionalFields = Vec<InventoryOptionalField>;
#[allow(dead_code)]
pub struct InventoryOptionalFieldsDeserializer;
impl InventoryOptionalFieldsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<InventoryOptionalField>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "InventoryOptionalField" {
                obj.push(deserialize_primitive(tag_name, stack, Ok)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type IsEnabled = bool;
pub type IsLatest = bool;
pub type IsPublic = bool;
pub type IsTruncated = bool;
pub type JSONType = String;
pub type KMSContext = String;
pub type KeyCount = i64;
pub type KeyMarker = String;
pub type KeyPrefixEquals = String;
pub type LambdaFunctionArn = String;
pub type LambdaFunctionConfigurationList = Vec<LambdaFunctionConfiguration>;
#[allow(dead_code)]
pub struct LambdaFunctionConfigurationListDeserializer;
impl LambdaFunctionConfigurationListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LambdaFunctionConfiguration>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "LambdaFunctionConfiguration" {
                obj.push(LambdaFunctionConfigurationDeserializer::deserialize("LambdaFunctionConfiguration", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type LastModified = String;
pub type LifecycleRules = Vec<LifecycleRule>;
#[allow(dead_code)]
pub struct LifecycleRulesDeserializer;
impl LifecycleRulesDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LifecycleRule>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "LifecycleRule" {
                obj.push(LifecycleRuleDeserializer::deserialize("LifecycleRule", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type Location = String;
pub type LocationPrefix = String;
pub type MFA = String;
pub type MFADelete = String;
pub type MFADeleteStatus = String;
pub type Marker = String;
pub type MaxAgeSeconds = i64;
pub type MaxKeys = i64;
pub type MaxParts = i64;
pub type MaxUploads = i64;
pub type Message = String;
pub type Metadata = HashMap<MetadataKey, MetadataValue>;
pub type MetadataDirective = String;
pub type MetadataKey = String;
pub type MetadataValue = String;
pub type MetricsConfigurationList = Vec<MetricsConfiguration>;
#[allow(dead_code)]
pub struct MetricsConfigurationListDeserializer;
impl MetricsConfigurationListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricsConfiguration>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "MetricsConfiguration" {
                obj.push(MetricsConfigurationDeserializer::deserialize("MetricsConfiguration", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type MetricsId = String;
pub type MetricsStatus = String;
pub type Minutes = i64;
pub type MissingMeta = i64;
pub type MultipartUploadId = String;
pub type MultipartUploadList = Vec<MultipartUpload>;
#[allow(dead_code)]
pub struct MultipartUploadListDeserializer;
impl MultipartUploadListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MultipartUpload>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "MultipartUpload" {
                obj.push(MultipartUploadDeserializer::deserialize("MultipartUpload", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type NextKeyMarker = String;
pub type NextMarker = String;
pub type NextPartNumberMarker = i64;
pub type NextToken = String;
pub type NextUploadIdMarker = String;
pub type NextVersionIdMarker = String;
pub type NoncurrentVersionTransitionList = Vec<NoncurrentVersionTransition>;
#[allow(dead_code)]
pub struct NoncurrentVersionTransitionListDeserializer;
impl NoncurrentVersionTransitionListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<NoncurrentVersionTransition>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "NoncurrentVersionTransition" {
                obj.push(NoncurrentVersionTransitionDeserializer::deserialize("NoncurrentVersionTransition", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type NotificationId = String;
pub type ObjectCannedACL = String;
pub type ObjectIdentifierList = Vec<ObjectIdentifier>;
#[allow(dead_code)]
pub struct ObjectIdentifierListDeserializer;
impl ObjectIdentifierListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ObjectIdentifier>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ObjectIdentifier" {
                obj.push(ObjectIdentifierDeserializer::deserialize("ObjectIdentifier", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type ObjectKey = String;
pub type ObjectList = Vec<Object>;
#[allow(dead_code)]
pub struct ObjectListDeserializer;
impl ObjectListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Object>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Object" {
                obj.push(ObjectDeserializer::deserialize("Object", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
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
#[allow(dead_code)]
pub struct ObjectVersionListDeserializer;
impl ObjectVersionListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ObjectVersion>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ObjectVersion" {
                obj.push(ObjectVersionDeserializer::deserialize("ObjectVersion", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type ObjectVersionStorageClass = String;
pub type OwnerOverride = String;
pub type OwnershipControlsRules = Vec<OwnershipControlsRule>;
#[allow(dead_code)]
pub struct OwnershipControlsRulesDeserializer;
impl OwnershipControlsRulesDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<OwnershipControlsRule>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "OwnershipControlsRule" {
                obj.push(OwnershipControlsRuleDeserializer::deserialize("OwnershipControlsRule", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type PartNumber = i64;
pub type PartNumberMarker = i64;
pub type Parts = Vec<Part>;
#[allow(dead_code)]
pub struct PartsDeserializer;
impl PartsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Part>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Part" {
                obj.push(PartDeserializer::deserialize("Part", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type PartsCount = i64;
pub type Payer = String;
pub type Permission = String;
pub type Policy = String;
pub type Prefix = String;
pub type Priority = i64;
pub type Protocol = String;
pub type QueueArn = String;
pub type QueueConfigurationList = Vec<QueueConfiguration>;
#[allow(dead_code)]
pub struct QueueConfigurationListDeserializer;
impl QueueConfigurationListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<QueueConfiguration>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "QueueConfiguration" {
                obj.push(QueueConfigurationDeserializer::deserialize("QueueConfiguration", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
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
#[allow(dead_code)]
pub struct ReplicationRulesDeserializer;
impl ReplicationRulesDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReplicationRule>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ReplicationRule" {
                obj.push(ReplicationRuleDeserializer::deserialize("ReplicationRule", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
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
#[allow(dead_code)]
pub struct RoutingRulesDeserializer;
impl RoutingRulesDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<RoutingRule>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "RoutingRule" {
                obj.push(RoutingRuleDeserializer::deserialize("RoutingRule", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type Rules = Vec<Rule>;
#[allow(dead_code)]
pub struct RulesDeserializer;
impl RulesDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Rule>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Rule" {
                obj.push(RuleDeserializer::deserialize("Rule", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type SSECustomerAlgorithm = String;
pub type SSECustomerKey = String;
pub type SSECustomerKeyMD5 = String;
pub type SSEKMSEncryptionContext = String;
pub type SSEKMSKeyId = String;
pub type ServerSideEncryption = String;
pub type ServerSideEncryptionRules = Vec<ServerSideEncryptionRule>;
#[allow(dead_code)]
pub struct ServerSideEncryptionRulesDeserializer;
impl ServerSideEncryptionRulesDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ServerSideEncryptionRule>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ServerSideEncryptionRule" {
                obj.push(ServerSideEncryptionRuleDeserializer::deserialize("ServerSideEncryptionRule", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type Setting = bool;
pub type Size = i64;
pub type SseKmsEncryptedObjectsStatus = String;
pub type Start = u64;
pub type StartAfter = String;
pub type StorageClass = String;
pub type StorageClassAnalysisSchemaVersion = String;
pub type Suffix = String;
pub type TagCount = i64;
pub type TagSet = Vec<Tag>;
#[allow(dead_code)]
pub struct TagSetDeserializer;
impl TagSetDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Tag" {
                obj.push(TagDeserializer::deserialize("Tag", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type TaggingDirective = String;
pub type TaggingHeader = String;
pub type TargetBucket = String;
pub type TargetGrants = Vec<TargetGrant>;
#[allow(dead_code)]
pub struct TargetGrantsDeserializer;
impl TargetGrantsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TargetGrant>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "TargetGrant" {
                obj.push(TargetGrantDeserializer::deserialize("TargetGrant", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type TargetPrefix = String;
pub type Tier = String;
pub type TieringList = Vec<Tiering>;
#[allow(dead_code)]
pub struct TieringListDeserializer;
impl TieringListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tiering>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Tiering" {
                obj.push(TieringDeserializer::deserialize("Tiering", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type Token = String;
pub type TopicArn = String;
pub type TopicConfigurationList = Vec<TopicConfiguration>;
#[allow(dead_code)]
pub struct TopicConfigurationListDeserializer;
impl TopicConfigurationListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TopicConfiguration>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "TopicConfiguration" {
                obj.push(TopicConfigurationDeserializer::deserialize("TopicConfiguration", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type TransitionList = Vec<Transition>;
#[allow(dead_code)]
pub struct TransitionListDeserializer;
impl TransitionListDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Transition>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Transition" {
                obj.push(TransitionDeserializer::deserialize("Transition", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type TransitionStorageClass = String;
pub type Type = String;
pub type URI = String;
pub type UploadIdMarker = String;
pub type UserMetadata = Vec<MetadataEntry>;
#[allow(dead_code)]
pub struct UserMetadataDeserializer;
impl UserMetadataDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetadataEntry>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "MetadataEntry" {
                obj.push(MetadataEntryDeserializer::deserialize("MetadataEntry", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
pub type Value = String;
pub type VersionIdMarker = String;
pub type WebsiteRedirectLocation = String;
pub type Years = i64;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AbortIncompleteMultipartUpload {
    #[serde(rename = "DaysAfterInitiation")]
    pub days_after_initiation: Option<DaysAfterInitiation>,
}
#[allow(dead_code)]
pub struct AbortIncompleteMultipartUploadDeserializer;
impl AbortIncompleteMultipartUploadDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AbortIncompleteMultipartUpload, XmlParseError> {
        deserialize_elements::<_, AbortIncompleteMultipartUpload, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DaysAfterInitiation" => {
                        obj.days_after_initiation = Some(deserialize_primitive("DaysAfterInitiation", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AbortMultipartUploadOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}
#[allow(dead_code)]
pub struct AbortMultipartUploadOutputDeserializer;
impl AbortMultipartUploadOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AbortMultipartUploadOutput, XmlParseError> {
        deserialize_elements::<_, AbortMultipartUploadOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AbortMultipartUploadRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "UploadId")]
    pub upload_id: MultipartUploadId,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct AbortMultipartUploadRequestDeserializer;
impl AbortMultipartUploadRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AbortMultipartUploadRequest, XmlParseError> {
        deserialize_elements::<_, AbortMultipartUploadRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "UploadId" => {
                        obj.upload_id = deserialize_primitive("UploadId", stack, Ok)?;
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccelerateConfiguration {
    #[serde(rename = "Status")]
    pub status: Option<BucketAccelerateStatus>,
}
#[allow(dead_code)]
pub struct AccelerateConfigurationDeserializer;
impl AccelerateConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccelerateConfiguration, XmlParseError> {
        deserialize_elements::<_, AccelerateConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = Some(deserialize_primitive("Status", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccessControlPolicy {
    #[serde(rename = "Grants")]
    pub grants: Option<Grants>,
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
}
#[allow(dead_code)]
pub struct AccessControlPolicyDeserializer;
impl AccessControlPolicyDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccessControlPolicy, XmlParseError> {
        deserialize_elements::<_, AccessControlPolicy, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Grants" => {
                        obj.grants
                            .get_or_insert(vec![])
                            .extend(GrantsDeserializer::deserialize("Grants", stack)?);
                    }
                    "Owner" => {
                        obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccessControlTranslation {
    #[serde(rename = "Owner")]
    pub owner: OwnerOverride,
}
#[allow(dead_code)]
pub struct AccessControlTranslationDeserializer;
impl AccessControlTranslationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccessControlTranslation, XmlParseError> {
        deserialize_elements::<_, AccessControlTranslation, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Owner" => {
                        obj.owner = deserialize_primitive("Owner", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalyticsAndOperator {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagSet>,
}
#[allow(dead_code)]
pub struct AnalyticsAndOperatorDeserializer;
impl AnalyticsAndOperatorDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsAndOperator, XmlParseError> {
        deserialize_elements::<_, AnalyticsAndOperator, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Tags" => {
                        obj.tags
                            .get_or_insert(vec![])
                            .extend(TagSetDeserializer::deserialize("Tags", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalyticsConfiguration {
    #[serde(rename = "Id")]
    pub id: AnalyticsId,
    #[serde(rename = "Filter")]
    pub filter: Option<AnalyticsFilter>,
    #[serde(rename = "StorageClassAnalysis")]
    pub storage_class_analysis: StorageClassAnalysis,
}
#[allow(dead_code)]
pub struct AnalyticsConfigurationDeserializer;
impl AnalyticsConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsConfiguration, XmlParseError> {
        deserialize_elements::<_, AnalyticsConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "Filter" => {
                        obj.filter = Some(AnalyticsFilterDeserializer::deserialize("Filter", stack)?);
                    }
                    "StorageClassAnalysis" => {
                        obj.storage_class_analysis = StorageClassAnalysisDeserializer::deserialize("StorageClassAnalysis", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalyticsExportDestination {
    #[serde(rename = "S3BucketDestination")]
    pub s3_bucket_destination: AnalyticsS3BucketDestination,
}
#[allow(dead_code)]
pub struct AnalyticsExportDestinationDeserializer;
impl AnalyticsExportDestinationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsExportDestination, XmlParseError> {
        deserialize_elements::<_, AnalyticsExportDestination, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "S3BucketDestination" => {
                        obj.s3_bucket_destination = AnalyticsS3BucketDestinationDeserializer::deserialize("S3BucketDestination", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalyticsFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tag")]
    pub tag: Option<Tag>,
    #[serde(rename = "And")]
    pub and: Option<AnalyticsAndOperator>,
}
#[allow(dead_code)]
pub struct AnalyticsFilterDeserializer;
impl AnalyticsFilterDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsFilter, XmlParseError> {
        deserialize_elements::<_, AnalyticsFilter, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Tag" => {
                        obj.tag = Some(TagDeserializer::deserialize("Tag", stack)?);
                    }
                    "And" => {
                        obj.and = Some(AnalyticsAndOperatorDeserializer::deserialize("And", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalyticsS3BucketDestination {
    #[serde(rename = "Format")]
    pub format: AnalyticsS3ExportFileFormat,
    #[serde(rename = "BucketAccountId")]
    pub bucket_account_id: Option<AccountId>,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
}
#[allow(dead_code)]
pub struct AnalyticsS3BucketDestinationDeserializer;
impl AnalyticsS3BucketDestinationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsS3BucketDestination, XmlParseError> {
        deserialize_elements::<_, AnalyticsS3BucketDestination, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Format" => {
                        obj.format = deserialize_primitive("Format", stack, Ok)?;
                    }
                    "BucketAccountId" => {
                        obj.bucket_account_id = Some(deserialize_primitive("BucketAccountId", stack, Ok)?);
                    }
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Bucket {
    #[serde(rename = "Name")]
    pub name: Option<BucketName>,
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<CreationDate>,
}
#[allow(dead_code)]
pub struct BucketDeserializer;
impl BucketDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Bucket, XmlParseError> {
        deserialize_elements::<_, Bucket, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Name" => {
                        obj.name = Some(deserialize_primitive("Name", stack, Ok)?);
                    }
                    "CreationDate" => {
                        obj.creation_date = Some(deserialize_primitive("CreationDate", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BucketAlreadyExists {
}
#[allow(dead_code)]
pub struct BucketAlreadyExistsDeserializer;
impl BucketAlreadyExistsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BucketAlreadyExists, XmlParseError> {
        deserialize_elements::<_, BucketAlreadyExists, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BucketAlreadyOwnedByYou {
}
#[allow(dead_code)]
pub struct BucketAlreadyOwnedByYouDeserializer;
impl BucketAlreadyOwnedByYouDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BucketAlreadyOwnedByYou, XmlParseError> {
        deserialize_elements::<_, BucketAlreadyOwnedByYou, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BucketLifecycleConfiguration {
    #[serde(rename = "Rules")]
    pub rules: LifecycleRules,
}
#[allow(dead_code)]
pub struct BucketLifecycleConfigurationDeserializer;
impl BucketLifecycleConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BucketLifecycleConfiguration, XmlParseError> {
        deserialize_elements::<_, BucketLifecycleConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Rules" => {
                        obj.rules
                            .extend(LifecycleRulesDeserializer::deserialize("Rules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BucketLoggingStatus {
    #[serde(rename = "LoggingEnabled")]
    pub logging_enabled: Option<LoggingEnabled>,
}
#[allow(dead_code)]
pub struct BucketLoggingStatusDeserializer;
impl BucketLoggingStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<BucketLoggingStatus, XmlParseError> {
        deserialize_elements::<_, BucketLoggingStatus, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LoggingEnabled" => {
                        obj.logging_enabled = Some(LoggingEnabledDeserializer::deserialize("LoggingEnabled", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CORSConfiguration {
    #[serde(rename = "CORSRules")]
    pub cors_rules: CORSRules,
}
#[allow(dead_code)]
pub struct CORSConfigurationDeserializer;
impl CORSConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CORSConfiguration, XmlParseError> {
        deserialize_elements::<_, CORSConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CORSRules" => {
                        obj.cors_rules
                            .extend(CORSRulesDeserializer::deserialize("CORSRules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CORSRule {
    #[serde(rename = "AllowedHeaders")]
    pub allowed_headers: Option<AllowedHeaders>,
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: AllowedMethods,
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: AllowedOrigins,
    #[serde(rename = "ExposeHeaders")]
    pub expose_headers: Option<ExposeHeaders>,
    #[serde(rename = "MaxAgeSeconds")]
    pub max_age_seconds: Option<MaxAgeSeconds>,
}
#[allow(dead_code)]
pub struct CORSRuleDeserializer;
impl CORSRuleDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CORSRule, XmlParseError> {
        deserialize_elements::<_, CORSRule, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AllowedHeaders" => {
                        obj.allowed_headers
                            .get_or_insert(vec![])
                            .extend(AllowedHeadersDeserializer::deserialize("AllowedHeaders", stack)?);
                    }
                    "AllowedMethods" => {
                        obj.allowed_methods
                            .extend(AllowedMethodsDeserializer::deserialize("AllowedMethods", stack)?);
                    }
                    "AllowedOrigins" => {
                        obj.allowed_origins
                            .extend(AllowedOriginsDeserializer::deserialize("AllowedOrigins", stack)?);
                    }
                    "ExposeHeaders" => {
                        obj.expose_headers
                            .get_or_insert(vec![])
                            .extend(ExposeHeadersDeserializer::deserialize("ExposeHeaders", stack)?);
                    }
                    "MaxAgeSeconds" => {
                        obj.max_age_seconds = Some(deserialize_primitive("MaxAgeSeconds", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct CSVInputDeserializer;
impl CSVInputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CSVInput, XmlParseError> {
        deserialize_elements::<_, CSVInput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "FileHeaderInfo" => {
                        obj.file_header_info = Some(deserialize_primitive("FileHeaderInfo", stack, Ok)?);
                    }
                    "Comments" => {
                        obj.comments = Some(deserialize_primitive("Comments", stack, Ok)?);
                    }
                    "QuoteEscapeCharacter" => {
                        obj.quote_escape_character = Some(deserialize_primitive("QuoteEscapeCharacter", stack, Ok)?);
                    }
                    "RecordDelimiter" => {
                        obj.record_delimiter = Some(deserialize_primitive("RecordDelimiter", stack, Ok)?);
                    }
                    "FieldDelimiter" => {
                        obj.field_delimiter = Some(deserialize_primitive("FieldDelimiter", stack, Ok)?);
                    }
                    "QuoteCharacter" => {
                        obj.quote_character = Some(deserialize_primitive("QuoteCharacter", stack, Ok)?);
                    }
                    "AllowQuotedRecordDelimiter" => {
                        obj.allow_quoted_record_delimiter = Some(deserialize_primitive("AllowQuotedRecordDelimiter", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct CSVOutputDeserializer;
impl CSVOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CSVOutput, XmlParseError> {
        deserialize_elements::<_, CSVOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "QuoteFields" => {
                        obj.quote_fields = Some(deserialize_primitive("QuoteFields", stack, Ok)?);
                    }
                    "QuoteEscapeCharacter" => {
                        obj.quote_escape_character = Some(deserialize_primitive("QuoteEscapeCharacter", stack, Ok)?);
                    }
                    "RecordDelimiter" => {
                        obj.record_delimiter = Some(deserialize_primitive("RecordDelimiter", stack, Ok)?);
                    }
                    "FieldDelimiter" => {
                        obj.field_delimiter = Some(deserialize_primitive("FieldDelimiter", stack, Ok)?);
                    }
                    "QuoteCharacter" => {
                        obj.quote_character = Some(deserialize_primitive("QuoteCharacter", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct CloudFunctionConfigurationDeserializer;
impl CloudFunctionConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudFunctionConfiguration, XmlParseError> {
        deserialize_elements::<_, CloudFunctionConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Id" => {
                        obj.id = Some(deserialize_primitive("Id", stack, Ok)?);
                    }
                    "Event" => {
                        obj.event = Some(deserialize_primitive("Event", stack, Ok)?);
                    }
                    "Events" => {
                        obj.events
                            .get_or_insert(vec![])
                            .extend(EventListDeserializer::deserialize("Events", stack)?);
                    }
                    "CloudFunction" => {
                        obj.cloud_function = Some(deserialize_primitive("CloudFunction", stack, Ok)?);
                    }
                    "InvocationRole" => {
                        obj.invocation_role = Some(deserialize_primitive("InvocationRole", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CommonPrefix {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
}
#[allow(dead_code)]
pub struct CommonPrefixDeserializer;
impl CommonPrefixDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CommonPrefix, XmlParseError> {
        deserialize_elements::<_, CommonPrefix, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct CompleteMultipartUploadOutputDeserializer;
impl CompleteMultipartUploadOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CompleteMultipartUploadOutput, XmlParseError> {
        deserialize_elements::<_, CompleteMultipartUploadOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Location" => {
                        obj.location = Some(deserialize_primitive("Location", stack, Ok)?);
                    }
                    "Bucket" => {
                        obj.bucket = Some(deserialize_primitive("Bucket", stack, Ok)?);
                    }
                    "Key" => {
                        obj.key = Some(deserialize_primitive("Key", stack, Ok)?);
                    }
                    "Expiration" => {
                        obj.expiration = Some(deserialize_primitive("Expiration", stack, Ok)?);
                    }
                    "ETag" => {
                        obj.e_tag = Some(deserialize_primitive("ETag", stack, Ok)?);
                    }
                    "ServerSideEncryption" => {
                        obj.server_side_encryption = Some(deserialize_primitive("ServerSideEncryption", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "SSEKMSKeyId" => {
                        obj.ssekms_key_id = Some(deserialize_primitive("SSEKMSKeyId", stack, Ok)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompleteMultipartUploadRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "MultipartUpload")]
    pub multipart_upload: Option<CompletedMultipartUpload>,
    #[serde(rename = "UploadId")]
    pub upload_id: MultipartUploadId,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct CompleteMultipartUploadRequestDeserializer;
impl CompleteMultipartUploadRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CompleteMultipartUploadRequest, XmlParseError> {
        deserialize_elements::<_, CompleteMultipartUploadRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "MultipartUpload" => {
                        obj.multipart_upload = Some(CompletedMultipartUploadDeserializer::deserialize("MultipartUpload", stack)?);
                    }
                    "UploadId" => {
                        obj.upload_id = deserialize_primitive("UploadId", stack, Ok)?;
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompletedMultipartUpload {
    #[serde(rename = "Parts")]
    pub parts: Option<CompletedPartList>,
}
#[allow(dead_code)]
pub struct CompletedMultipartUploadDeserializer;
impl CompletedMultipartUploadDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CompletedMultipartUpload, XmlParseError> {
        deserialize_elements::<_, CompletedMultipartUpload, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Parts" => {
                        obj.parts
                            .get_or_insert(vec![])
                            .extend(CompletedPartListDeserializer::deserialize("Parts", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompletedPart {
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "PartNumber")]
    pub part_number: Option<PartNumber>,
}
#[allow(dead_code)]
pub struct CompletedPartDeserializer;
impl CompletedPartDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CompletedPart, XmlParseError> {
        deserialize_elements::<_, CompletedPart, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ETag" => {
                        obj.e_tag = Some(deserialize_primitive("ETag", stack, Ok)?);
                    }
                    "PartNumber" => {
                        obj.part_number = Some(deserialize_primitive("PartNumber", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "HttpErrorCodeReturnedEquals")]
    pub http_error_code_returned_equals: Option<HttpErrorCodeReturnedEquals>,
    #[serde(rename = "KeyPrefixEquals")]
    pub key_prefix_equals: Option<KeyPrefixEquals>,
}
#[allow(dead_code)]
pub struct ConditionDeserializer;
impl ConditionDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Condition, XmlParseError> {
        deserialize_elements::<_, Condition, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HttpErrorCodeReturnedEquals" => {
                        obj.http_error_code_returned_equals = Some(deserialize_primitive("HttpErrorCodeReturnedEquals", stack, Ok)?);
                    }
                    "KeyPrefixEquals" => {
                        obj.key_prefix_equals = Some(deserialize_primitive("KeyPrefixEquals", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContinuationEvent {
}
#[allow(dead_code)]
pub struct ContinuationEventDeserializer;
impl ContinuationEventDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ContinuationEvent, XmlParseError> {
        deserialize_elements::<_, ContinuationEvent, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct CopyObjectOutputDeserializer;
impl CopyObjectOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyObjectOutput, XmlParseError> {
        deserialize_elements::<_, CopyObjectOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CopyObjectResult" => {
                        obj.copy_object_result = Some(CopyObjectResultDeserializer::deserialize("CopyObjectResult", stack)?);
                    }
                    "Expiration" => {
                        obj.expiration = Some(deserialize_primitive("Expiration", stack, Ok)?);
                    }
                    "CopySourceVersionId" => {
                        obj.copy_source_version_id = Some(deserialize_primitive("CopySourceVersionId", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "ServerSideEncryption" => {
                        obj.server_side_encryption = Some(deserialize_primitive("ServerSideEncryption", stack, Ok)?);
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "SSEKMSKeyId" => {
                        obj.ssekms_key_id = Some(deserialize_primitive("SSEKMSKeyId", stack, Ok)?);
                    }
                    "SSEKMSEncryptionContext" => {
                        obj.ssekms_encryption_context = Some(deserialize_primitive("SSEKMSEncryptionContext", stack, Ok)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CopyObjectRequest {
    #[serde(rename = "ACL")]
    pub acl: Option<ObjectCannedACL>,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
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
    #[serde(rename = "CopySource")]
    pub copy_source: CopySource,
    #[serde(rename = "CopySourceIfMatch")]
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    #[serde(rename = "CopySourceIfModifiedSince")]
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    #[serde(rename = "CopySourceIfNoneMatch")]
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    #[serde(rename = "CopySourceIfUnmodifiedSince")]
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    #[serde(rename = "Expires")]
    pub expires: Option<Expires>,
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: Option<GrantFullControl>,
    #[serde(rename = "GrantRead")]
    pub grant_read: Option<GrantRead>,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: Option<GrantReadACP>,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: Option<GrantWriteACP>,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "MetadataDirective")]
    pub metadata_directive: Option<MetadataDirective>,
    #[serde(rename = "TaggingDirective")]
    pub tagging_directive: Option<TaggingDirective>,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    #[serde(rename = "WebsiteRedirectLocation")]
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: Option<SSECustomerKey>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "SSEKMSEncryptionContext")]
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    #[serde(rename = "CopySourceSSECustomerAlgorithm")]
    pub copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm>,
    #[serde(rename = "CopySourceSSECustomerKey")]
    pub copy_source_sse_customer_key: Option<CopySourceSSECustomerKey>,
    #[serde(rename = "CopySourceSSECustomerKeyMD5")]
    pub copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "Tagging")]
    pub tagging: Option<TaggingHeader>,
    #[serde(rename = "ObjectLockMode")]
    pub object_lock_mode: Option<ObjectLockMode>,
    #[serde(rename = "ObjectLockRetainUntilDate")]
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    #[serde(rename = "ObjectLockLegalHoldStatus")]
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
    #[serde(rename = "ExpectedSourceBucketOwner")]
    pub expected_source_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct CopyObjectRequestDeserializer;
impl CopyObjectRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyObjectRequest, XmlParseError> {
        deserialize_elements::<_, CopyObjectRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ACL" => {
                        obj.acl = Some(deserialize_primitive("ACL", stack, Ok)?);
                    }
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "CacheControl" => {
                        obj.cache_control = Some(deserialize_primitive("CacheControl", stack, Ok)?);
                    }
                    "ContentDisposition" => {
                        obj.content_disposition = Some(deserialize_primitive("ContentDisposition", stack, Ok)?);
                    }
                    "ContentEncoding" => {
                        obj.content_encoding = Some(deserialize_primitive("ContentEncoding", stack, Ok)?);
                    }
                    "ContentLanguage" => {
                        obj.content_language = Some(deserialize_primitive("ContentLanguage", stack, Ok)?);
                    }
                    "ContentType" => {
                        obj.content_type = Some(deserialize_primitive("ContentType", stack, Ok)?);
                    }
                    "CopySource" => {
                        obj.copy_source = deserialize_primitive("CopySource", stack, Ok)?;
                    }
                    "CopySourceIfMatch" => {
                        obj.copy_source_if_match = Some(deserialize_primitive("CopySourceIfMatch", stack, Ok)?);
                    }
                    "CopySourceIfModifiedSince" => {
                        obj.copy_source_if_modified_since = Some(deserialize_primitive("CopySourceIfModifiedSince", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "CopySourceIfNoneMatch" => {
                        obj.copy_source_if_none_match = Some(deserialize_primitive("CopySourceIfNoneMatch", stack, Ok)?);
                    }
                    "CopySourceIfUnmodifiedSince" => {
                        obj.copy_source_if_unmodified_since = Some(deserialize_primitive("CopySourceIfUnmodifiedSince", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "Expires" => {
                        obj.expires = Some(deserialize_primitive("Expires", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "GrantFullControl" => {
                        obj.grant_full_control = Some(deserialize_primitive("GrantFullControl", stack, Ok)?);
                    }
                    "GrantRead" => {
                        obj.grant_read = Some(deserialize_primitive("GrantRead", stack, Ok)?);
                    }
                    "GrantReadACP" => {
                        obj.grant_read_acp = Some(deserialize_primitive("GrantReadACP", stack, Ok)?);
                    }
                    "GrantWriteACP" => {
                        obj.grant_write_acp = Some(deserialize_primitive("GrantWriteACP", stack, Ok)?);
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "Metadata" => {
                        obj.metadata = Some(Default::default());
                    }
                    "MetadataDirective" => {
                        obj.metadata_directive = Some(deserialize_primitive("MetadataDirective", stack, Ok)?);
                    }
                    "TaggingDirective" => {
                        obj.tagging_directive = Some(deserialize_primitive("TaggingDirective", stack, Ok)?);
                    }
                    "ServerSideEncryption" => {
                        obj.server_side_encryption = Some(deserialize_primitive("ServerSideEncryption", stack, Ok)?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    "WebsiteRedirectLocation" => {
                        obj.website_redirect_location = Some(deserialize_primitive("WebsiteRedirectLocation", stack, Ok)?);
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKey" => {
                        obj.sse_customer_key = Some(deserialize_primitive("SSECustomerKey", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "SSEKMSKeyId" => {
                        obj.ssekms_key_id = Some(deserialize_primitive("SSEKMSKeyId", stack, Ok)?);
                    }
                    "SSEKMSEncryptionContext" => {
                        obj.ssekms_encryption_context = Some(deserialize_primitive("SSEKMSEncryptionContext", stack, Ok)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "CopySourceSSECustomerAlgorithm" => {
                        obj.copy_source_sse_customer_algorithm = Some(deserialize_primitive("CopySourceSSECustomerAlgorithm", stack, Ok)?);
                    }
                    "CopySourceSSECustomerKey" => {
                        obj.copy_source_sse_customer_key = Some(deserialize_primitive("CopySourceSSECustomerKey", stack, Ok)?);
                    }
                    "CopySourceSSECustomerKeyMD5" => {
                        obj.copy_source_sse_customer_key_md5 = Some(deserialize_primitive("CopySourceSSECustomerKeyMD5", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "Tagging" => {
                        obj.tagging = Some(deserialize_primitive("Tagging", stack, Ok)?);
                    }
                    "ObjectLockMode" => {
                        obj.object_lock_mode = Some(deserialize_primitive("ObjectLockMode", stack, Ok)?);
                    }
                    "ObjectLockRetainUntilDate" => {
                        obj.object_lock_retain_until_date = Some(deserialize_primitive("ObjectLockRetainUntilDate", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "ObjectLockLegalHoldStatus" => {
                        obj.object_lock_legal_hold_status = Some(deserialize_primitive("ObjectLockLegalHoldStatus", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    "ExpectedSourceBucketOwner" => {
                        obj.expected_source_bucket_owner = Some(deserialize_primitive("ExpectedSourceBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CopyObjectResult {
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<LastModified>,
}
#[allow(dead_code)]
pub struct CopyObjectResultDeserializer;
impl CopyObjectResultDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyObjectResult, XmlParseError> {
        deserialize_elements::<_, CopyObjectResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ETag" => {
                        obj.e_tag = Some(deserialize_primitive("ETag", stack, Ok)?);
                    }
                    "LastModified" => {
                        obj.last_modified = Some(deserialize_primitive("LastModified", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CopyPartResult {
    #[serde(rename = "ETag")]
    pub e_tag: Option<ETag>,
    #[serde(rename = "LastModified")]
    pub last_modified: Option<LastModified>,
}
#[allow(dead_code)]
pub struct CopyPartResultDeserializer;
impl CopyPartResultDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyPartResult, XmlParseError> {
        deserialize_elements::<_, CopyPartResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ETag" => {
                        obj.e_tag = Some(deserialize_primitive("ETag", stack, Ok)?);
                    }
                    "LastModified" => {
                        obj.last_modified = Some(deserialize_primitive("LastModified", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateBucketConfiguration {
    #[serde(rename = "LocationConstraint")]
    pub location_constraint: Option<BucketLocationConstraint>,
}
#[allow(dead_code)]
pub struct CreateBucketConfigurationDeserializer;
impl CreateBucketConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateBucketConfiguration, XmlParseError> {
        deserialize_elements::<_, CreateBucketConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LocationConstraint" => {
                        obj.location_constraint = Some(deserialize_primitive("LocationConstraint", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateBucketOutput {
    #[serde(rename = "Location")]
    pub location: Option<Location>,
}
#[allow(dead_code)]
pub struct CreateBucketOutputDeserializer;
impl CreateBucketOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateBucketOutput, XmlParseError> {
        deserialize_elements::<_, CreateBucketOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Location" => {
                        obj.location = Some(deserialize_primitive("Location", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateBucketRequest {
    #[serde(rename = "ACL")]
    pub acl: Option<BucketCannedACL>,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "CreateBucketConfiguration")]
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: Option<GrantFullControl>,
    #[serde(rename = "GrantRead")]
    pub grant_read: Option<GrantRead>,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: Option<GrantReadACP>,
    #[serde(rename = "GrantWrite")]
    pub grant_write: Option<GrantWrite>,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: Option<GrantWriteACP>,
    #[serde(rename = "ObjectLockEnabledForBucket")]
    pub object_lock_enabled_for_bucket: Option<ObjectLockEnabledForBucket>,
}
#[allow(dead_code)]
pub struct CreateBucketRequestDeserializer;
impl CreateBucketRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateBucketRequest, XmlParseError> {
        deserialize_elements::<_, CreateBucketRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ACL" => {
                        obj.acl = Some(deserialize_primitive("ACL", stack, Ok)?);
                    }
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "CreateBucketConfiguration" => {
                        obj.create_bucket_configuration = Some(CreateBucketConfigurationDeserializer::deserialize("CreateBucketConfiguration", stack)?);
                    }
                    "GrantFullControl" => {
                        obj.grant_full_control = Some(deserialize_primitive("GrantFullControl", stack, Ok)?);
                    }
                    "GrantRead" => {
                        obj.grant_read = Some(deserialize_primitive("GrantRead", stack, Ok)?);
                    }
                    "GrantReadACP" => {
                        obj.grant_read_acp = Some(deserialize_primitive("GrantReadACP", stack, Ok)?);
                    }
                    "GrantWrite" => {
                        obj.grant_write = Some(deserialize_primitive("GrantWrite", stack, Ok)?);
                    }
                    "GrantWriteACP" => {
                        obj.grant_write_acp = Some(deserialize_primitive("GrantWriteACP", stack, Ok)?);
                    }
                    "ObjectLockEnabledForBucket" => {
                        obj.object_lock_enabled_for_bucket = Some(deserialize_primitive("ObjectLockEnabledForBucket", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct CreateMultipartUploadOutputDeserializer;
impl CreateMultipartUploadOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateMultipartUploadOutput, XmlParseError> {
        deserialize_elements::<_, CreateMultipartUploadOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AbortDate" => {
                        obj.abort_date = Some(deserialize_primitive("AbortDate", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "AbortRuleId" => {
                        obj.abort_rule_id = Some(deserialize_primitive("AbortRuleId", stack, Ok)?);
                    }
                    "Bucket" => {
                        obj.bucket = Some(deserialize_primitive("Bucket", stack, Ok)?);
                    }
                    "Key" => {
                        obj.key = Some(deserialize_primitive("Key", stack, Ok)?);
                    }
                    "UploadId" => {
                        obj.upload_id = Some(deserialize_primitive("UploadId", stack, Ok)?);
                    }
                    "ServerSideEncryption" => {
                        obj.server_side_encryption = Some(deserialize_primitive("ServerSideEncryption", stack, Ok)?);
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "SSEKMSKeyId" => {
                        obj.ssekms_key_id = Some(deserialize_primitive("SSEKMSKeyId", stack, Ok)?);
                    }
                    "SSEKMSEncryptionContext" => {
                        obj.ssekms_encryption_context = Some(deserialize_primitive("SSEKMSEncryptionContext", stack, Ok)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateMultipartUploadRequest {
    #[serde(rename = "ACL")]
    pub acl: Option<ObjectCannedACL>,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
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
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: Option<GrantFullControl>,
    #[serde(rename = "GrantRead")]
    pub grant_read: Option<GrantRead>,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: Option<GrantReadACP>,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: Option<GrantWriteACP>,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    #[serde(rename = "WebsiteRedirectLocation")]
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: Option<SSECustomerKey>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "SSEKMSEncryptionContext")]
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "Tagging")]
    pub tagging: Option<TaggingHeader>,
    #[serde(rename = "ObjectLockMode")]
    pub object_lock_mode: Option<ObjectLockMode>,
    #[serde(rename = "ObjectLockRetainUntilDate")]
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    #[serde(rename = "ObjectLockLegalHoldStatus")]
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct CreateMultipartUploadRequestDeserializer;
impl CreateMultipartUploadRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateMultipartUploadRequest, XmlParseError> {
        deserialize_elements::<_, CreateMultipartUploadRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ACL" => {
                        obj.acl = Some(deserialize_primitive("ACL", stack, Ok)?);
                    }
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "CacheControl" => {
                        obj.cache_control = Some(deserialize_primitive("CacheControl", stack, Ok)?);
                    }
                    "ContentDisposition" => {
                        obj.content_disposition = Some(deserialize_primitive("ContentDisposition", stack, Ok)?);
                    }
                    "ContentEncoding" => {
                        obj.content_encoding = Some(deserialize_primitive("ContentEncoding", stack, Ok)?);
                    }
                    "ContentLanguage" => {
                        obj.content_language = Some(deserialize_primitive("ContentLanguage", stack, Ok)?);
                    }
                    "ContentType" => {
                        obj.content_type = Some(deserialize_primitive("ContentType", stack, Ok)?);
                    }
                    "Expires" => {
                        obj.expires = Some(deserialize_primitive("Expires", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "GrantFullControl" => {
                        obj.grant_full_control = Some(deserialize_primitive("GrantFullControl", stack, Ok)?);
                    }
                    "GrantRead" => {
                        obj.grant_read = Some(deserialize_primitive("GrantRead", stack, Ok)?);
                    }
                    "GrantReadACP" => {
                        obj.grant_read_acp = Some(deserialize_primitive("GrantReadACP", stack, Ok)?);
                    }
                    "GrantWriteACP" => {
                        obj.grant_write_acp = Some(deserialize_primitive("GrantWriteACP", stack, Ok)?);
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "Metadata" => {
                        obj.metadata = Some(Default::default());
                    }
                    "ServerSideEncryption" => {
                        obj.server_side_encryption = Some(deserialize_primitive("ServerSideEncryption", stack, Ok)?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    "WebsiteRedirectLocation" => {
                        obj.website_redirect_location = Some(deserialize_primitive("WebsiteRedirectLocation", stack, Ok)?);
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKey" => {
                        obj.sse_customer_key = Some(deserialize_primitive("SSECustomerKey", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "SSEKMSKeyId" => {
                        obj.ssekms_key_id = Some(deserialize_primitive("SSEKMSKeyId", stack, Ok)?);
                    }
                    "SSEKMSEncryptionContext" => {
                        obj.ssekms_encryption_context = Some(deserialize_primitive("SSEKMSEncryptionContext", stack, Ok)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "Tagging" => {
                        obj.tagging = Some(deserialize_primitive("Tagging", stack, Ok)?);
                    }
                    "ObjectLockMode" => {
                        obj.object_lock_mode = Some(deserialize_primitive("ObjectLockMode", stack, Ok)?);
                    }
                    "ObjectLockRetainUntilDate" => {
                        obj.object_lock_retain_until_date = Some(deserialize_primitive("ObjectLockRetainUntilDate", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "ObjectLockLegalHoldStatus" => {
                        obj.object_lock_legal_hold_status = Some(deserialize_primitive("ObjectLockLegalHoldStatus", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DefaultRetention {
    #[serde(rename = "Mode")]
    pub mode: Option<ObjectLockRetentionMode>,
    #[serde(rename = "Days")]
    pub days: Option<Days>,
    #[serde(rename = "Years")]
    pub years: Option<Years>,
}
#[allow(dead_code)]
pub struct DefaultRetentionDeserializer;
impl DefaultRetentionDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefaultRetention, XmlParseError> {
        deserialize_elements::<_, DefaultRetention, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Mode" => {
                        obj.mode = Some(deserialize_primitive("Mode", stack, Ok)?);
                    }
                    "Days" => {
                        obj.days = Some(deserialize_primitive("Days", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "Years" => {
                        obj.years = Some(deserialize_primitive("Years", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Delete {
    #[serde(rename = "Objects")]
    pub objects: ObjectIdentifierList,
    #[serde(rename = "Quiet")]
    pub quiet: Option<Quiet>,
}
#[allow(dead_code)]
pub struct DeleteDeserializer;
impl DeleteDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Delete, XmlParseError> {
        deserialize_elements::<_, Delete, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Objects" => {
                        obj.objects
                            .extend(ObjectIdentifierListDeserializer::deserialize("Objects", stack)?);
                    }
                    "Quiet" => {
                        obj.quiet = Some(deserialize_primitive("Quiet", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketAnalyticsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: AnalyticsId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketAnalyticsConfigurationRequestDeserializer;
impl DeleteBucketAnalyticsConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketAnalyticsConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketAnalyticsConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketCorsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketCorsRequestDeserializer;
impl DeleteBucketCorsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketCorsRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketCorsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketEncryptionRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketEncryptionRequestDeserializer;
impl DeleteBucketEncryptionRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketEncryptionRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketEncryptionRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketIntelligentTieringConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: IntelligentTieringId,
}
#[allow(dead_code)]
pub struct DeleteBucketIntelligentTieringConfigurationRequestDeserializer;
impl DeleteBucketIntelligentTieringConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketIntelligentTieringConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketIntelligentTieringConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketInventoryConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: InventoryId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketInventoryConfigurationRequestDeserializer;
impl DeleteBucketInventoryConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketInventoryConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketInventoryConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketLifecycleRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketLifecycleRequestDeserializer;
impl DeleteBucketLifecycleRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketLifecycleRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketLifecycleRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketMetricsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: MetricsId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketMetricsConfigurationRequestDeserializer;
impl DeleteBucketMetricsConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketMetricsConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketMetricsConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketOwnershipControlsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketOwnershipControlsRequestDeserializer;
impl DeleteBucketOwnershipControlsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketOwnershipControlsRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketOwnershipControlsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketPolicyRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketPolicyRequestDeserializer;
impl DeleteBucketPolicyRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketPolicyRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketPolicyRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketReplicationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketReplicationRequestDeserializer;
impl DeleteBucketReplicationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketReplicationRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketReplicationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketRequestDeserializer;
impl DeleteBucketRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketTaggingRequestDeserializer;
impl DeleteBucketTaggingRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketTaggingRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketTaggingRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBucketWebsiteRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteBucketWebsiteRequestDeserializer;
impl DeleteBucketWebsiteRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketWebsiteRequest, XmlParseError> {
        deserialize_elements::<_, DeleteBucketWebsiteRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct DeleteMarkerEntryDeserializer;
impl DeleteMarkerEntryDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteMarkerEntry, XmlParseError> {
        deserialize_elements::<_, DeleteMarkerEntry, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Owner" => {
                        obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                    }
                    "Key" => {
                        obj.key = Some(deserialize_primitive("Key", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "IsLatest" => {
                        obj.is_latest = Some(deserialize_primitive("IsLatest", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "LastModified" => {
                        obj.last_modified = Some(deserialize_primitive("LastModified", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteMarkerReplication {
    #[serde(rename = "Status")]
    pub status: Option<DeleteMarkerReplicationStatus>,
}
#[allow(dead_code)]
pub struct DeleteMarkerReplicationDeserializer;
impl DeleteMarkerReplicationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteMarkerReplication, XmlParseError> {
        deserialize_elements::<_, DeleteMarkerReplication, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = Some(deserialize_primitive("Status", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteObjectOutput {
    #[serde(rename = "DeleteMarker")]
    pub delete_marker: Option<DeleteMarker>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}
#[allow(dead_code)]
pub struct DeleteObjectOutputDeserializer;
impl DeleteObjectOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectOutput, XmlParseError> {
        deserialize_elements::<_, DeleteObjectOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DeleteMarker" => {
                        obj.delete_marker = Some(deserialize_primitive("DeleteMarker", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteObjectRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "MFA")]
    pub mfa: Option<MFA>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "BypassGovernanceRetention")]
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteObjectRequestDeserializer;
impl DeleteObjectRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectRequest, XmlParseError> {
        deserialize_elements::<_, DeleteObjectRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "MFA" => {
                        obj.mfa = Some(deserialize_primitive("MFA", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "BypassGovernanceRetention" => {
                        obj.bypass_governance_retention = Some(deserialize_primitive("BypassGovernanceRetention", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteObjectTaggingOutput {
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
}
#[allow(dead_code)]
pub struct DeleteObjectTaggingOutputDeserializer;
impl DeleteObjectTaggingOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectTaggingOutput, XmlParseError> {
        deserialize_elements::<_, DeleteObjectTaggingOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteObjectTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteObjectTaggingRequestDeserializer;
impl DeleteObjectTaggingRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectTaggingRequest, XmlParseError> {
        deserialize_elements::<_, DeleteObjectTaggingRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteObjectsOutput {
    #[serde(rename = "Deleted")]
    pub deleted: Option<DeletedObjects>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
    #[serde(rename = "Errors")]
    pub errors: Option<Errors>,
}
#[allow(dead_code)]
pub struct DeleteObjectsOutputDeserializer;
impl DeleteObjectsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectsOutput, XmlParseError> {
        deserialize_elements::<_, DeleteObjectsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Deleted" => {
                        obj.deleted
                            .get_or_insert(vec![])
                            .extend(DeletedObjectsDeserializer::deserialize("Deleted", stack)?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    "Errors" => {
                        obj.errors
                            .get_or_insert(vec![])
                            .extend(ErrorsDeserializer::deserialize("Errors", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteObjectsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Delete")]
    pub delete: Delete,
    #[serde(rename = "MFA")]
    pub mfa: Option<MFA>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "BypassGovernanceRetention")]
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeleteObjectsRequestDeserializer;
impl DeleteObjectsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectsRequest, XmlParseError> {
        deserialize_elements::<_, DeleteObjectsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Delete" => {
                        obj.delete = DeleteDeserializer::deserialize("Delete", stack)?;
                    }
                    "MFA" => {
                        obj.mfa = Some(deserialize_primitive("MFA", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "BypassGovernanceRetention" => {
                        obj.bypass_governance_retention = Some(deserialize_primitive("BypassGovernanceRetention", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeletePublicAccessBlockRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct DeletePublicAccessBlockRequestDeserializer;
impl DeletePublicAccessBlockRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeletePublicAccessBlockRequest, XmlParseError> {
        deserialize_elements::<_, DeletePublicAccessBlockRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct DeletedObjectDeserializer;
impl DeletedObjectDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeletedObject, XmlParseError> {
        deserialize_elements::<_, DeletedObject, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Key" => {
                        obj.key = Some(deserialize_primitive("Key", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "DeleteMarker" => {
                        obj.delete_marker = Some(deserialize_primitive("DeleteMarker", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "DeleteMarkerVersionId" => {
                        obj.delete_marker_version_id = Some(deserialize_primitive("DeleteMarkerVersionId", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Destination {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Account")]
    pub account: Option<AccountId>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    #[serde(rename = "AccessControlTranslation")]
    pub access_control_translation: Option<AccessControlTranslation>,
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "ReplicationTime")]
    pub replication_time: Option<ReplicationTime>,
    #[serde(rename = "Metrics")]
    pub metrics: Option<Metrics>,
}
#[allow(dead_code)]
pub struct DestinationDeserializer;
impl DestinationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Destination, XmlParseError> {
        deserialize_elements::<_, Destination, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Account" => {
                        obj.account = Some(deserialize_primitive("Account", stack, Ok)?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    "AccessControlTranslation" => {
                        obj.access_control_translation = Some(AccessControlTranslationDeserializer::deserialize("AccessControlTranslation", stack)?);
                    }
                    "EncryptionConfiguration" => {
                        obj.encryption_configuration = Some(EncryptionConfigurationDeserializer::deserialize("EncryptionConfiguration", stack)?);
                    }
                    "ReplicationTime" => {
                        obj.replication_time = Some(ReplicationTimeDeserializer::deserialize("ReplicationTime", stack)?);
                    }
                    "Metrics" => {
                        obj.metrics = Some(MetricsDeserializer::deserialize("Metrics", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Encryption {
    #[serde(rename = "EncryptionType")]
    pub encryption_type: ServerSideEncryption,
    #[serde(rename = "KMSKeyId")]
    pub kms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "KMSContext")]
    pub kms_context: Option<KMSContext>,
}
#[allow(dead_code)]
pub struct EncryptionDeserializer;
impl EncryptionDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Encryption, XmlParseError> {
        deserialize_elements::<_, Encryption, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EncryptionType" => {
                        obj.encryption_type = deserialize_primitive("EncryptionType", stack, Ok)?;
                    }
                    "KMSKeyId" => {
                        obj.kms_key_id = Some(deserialize_primitive("KMSKeyId", stack, Ok)?);
                    }
                    "KMSContext" => {
                        obj.kms_context = Some(deserialize_primitive("KMSContext", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "ReplicaKmsKeyID")]
    pub replica_kms_key_id: Option<ReplicaKmsKeyID>,
}
#[allow(dead_code)]
pub struct EncryptionConfigurationDeserializer;
impl EncryptionConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EncryptionConfiguration, XmlParseError> {
        deserialize_elements::<_, EncryptionConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ReplicaKmsKeyID" => {
                        obj.replica_kms_key_id = Some(deserialize_primitive("ReplicaKmsKeyID", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EndEvent {
}
#[allow(dead_code)]
pub struct EndEventDeserializer;
impl EndEventDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EndEvent, XmlParseError> {
        deserialize_elements::<_, EndEvent, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ErrorDeserializer;
impl ErrorDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Error, XmlParseError> {
        deserialize_elements::<_, Error, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Key" => {
                        obj.key = Some(deserialize_primitive("Key", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "Code" => {
                        obj.code = Some(deserialize_primitive("Code", stack, Ok)?);
                    }
                    "Message" => {
                        obj.message = Some(deserialize_primitive("Message", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ErrorDocument {
    #[serde(rename = "Key")]
    pub key: ObjectKey,
}
#[allow(dead_code)]
pub struct ErrorDocumentDeserializer;
impl ErrorDocumentDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ErrorDocument, XmlParseError> {
        deserialize_elements::<_, ErrorDocument, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExistingObjectReplication {
    #[serde(rename = "Status")]
    pub status: ExistingObjectReplicationStatus,
}
#[allow(dead_code)]
pub struct ExistingObjectReplicationDeserializer;
impl ExistingObjectReplicationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ExistingObjectReplication, XmlParseError> {
        deserialize_elements::<_, ExistingObjectReplication, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = deserialize_primitive("Status", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FilterRule {
    #[serde(rename = "Name")]
    pub name: Option<FilterRuleName>,
    #[serde(rename = "Value")]
    pub value: Option<FilterRuleValue>,
}
#[allow(dead_code)]
pub struct FilterRuleDeserializer;
impl FilterRuleDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FilterRule, XmlParseError> {
        deserialize_elements::<_, FilterRule, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Name" => {
                        obj.name = Some(deserialize_primitive("Name", stack, Ok)?);
                    }
                    "Value" => {
                        obj.value = Some(deserialize_primitive("Value", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketAccelerateConfigurationOutput {
    #[serde(rename = "Status")]
    pub status: Option<BucketAccelerateStatus>,
}
#[allow(dead_code)]
pub struct GetBucketAccelerateConfigurationOutputDeserializer;
impl GetBucketAccelerateConfigurationOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAccelerateConfigurationOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketAccelerateConfigurationOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = Some(deserialize_primitive("Status", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketAccelerateConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketAccelerateConfigurationRequestDeserializer;
impl GetBucketAccelerateConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAccelerateConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketAccelerateConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketAclOutput {
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    #[serde(rename = "Grants")]
    pub grants: Option<Grants>,
}
#[allow(dead_code)]
pub struct GetBucketAclOutputDeserializer;
impl GetBucketAclOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAclOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketAclOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Owner" => {
                        obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                    }
                    "Grants" => {
                        obj.grants
                            .get_or_insert(vec![])
                            .extend(GrantsDeserializer::deserialize("Grants", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketAclRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketAclRequestDeserializer;
impl GetBucketAclRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAclRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketAclRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketAnalyticsConfigurationOutput {
    #[serde(rename = "AnalyticsConfiguration")]
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}
#[allow(dead_code)]
pub struct GetBucketAnalyticsConfigurationOutputDeserializer;
impl GetBucketAnalyticsConfigurationOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAnalyticsConfigurationOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketAnalyticsConfigurationOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AnalyticsConfiguration" => {
                        obj.analytics_configuration = Some(AnalyticsConfigurationDeserializer::deserialize("AnalyticsConfiguration", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketAnalyticsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: AnalyticsId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketAnalyticsConfigurationRequestDeserializer;
impl GetBucketAnalyticsConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAnalyticsConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketAnalyticsConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketCorsOutput {
    #[serde(rename = "CORSRules")]
    pub cors_rules: Option<CORSRules>,
}
#[allow(dead_code)]
pub struct GetBucketCorsOutputDeserializer;
impl GetBucketCorsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketCorsOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketCorsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CORSRules" => {
                        obj.cors_rules
                            .get_or_insert(vec![])
                            .extend(CORSRulesDeserializer::deserialize("CORSRules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketCorsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketCorsRequestDeserializer;
impl GetBucketCorsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketCorsRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketCorsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketEncryptionOutput {
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}
#[allow(dead_code)]
pub struct GetBucketEncryptionOutputDeserializer;
impl GetBucketEncryptionOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketEncryptionOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketEncryptionOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ServerSideEncryptionConfiguration" => {
                        obj.server_side_encryption_configuration = Some(ServerSideEncryptionConfigurationDeserializer::deserialize("ServerSideEncryptionConfiguration", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketEncryptionRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketEncryptionRequestDeserializer;
impl GetBucketEncryptionRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketEncryptionRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketEncryptionRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketIntelligentTieringConfigurationOutput {
    #[serde(rename = "IntelligentTieringConfiguration")]
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
}
#[allow(dead_code)]
pub struct GetBucketIntelligentTieringConfigurationOutputDeserializer;
impl GetBucketIntelligentTieringConfigurationOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketIntelligentTieringConfigurationOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketIntelligentTieringConfigurationOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IntelligentTieringConfiguration" => {
                        obj.intelligent_tiering_configuration = Some(IntelligentTieringConfigurationDeserializer::deserialize("IntelligentTieringConfiguration", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketIntelligentTieringConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: IntelligentTieringId,
}
#[allow(dead_code)]
pub struct GetBucketIntelligentTieringConfigurationRequestDeserializer;
impl GetBucketIntelligentTieringConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketIntelligentTieringConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketIntelligentTieringConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketInventoryConfigurationOutput {
    #[serde(rename = "InventoryConfiguration")]
    pub inventory_configuration: Option<InventoryConfiguration>,
}
#[allow(dead_code)]
pub struct GetBucketInventoryConfigurationOutputDeserializer;
impl GetBucketInventoryConfigurationOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketInventoryConfigurationOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketInventoryConfigurationOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "InventoryConfiguration" => {
                        obj.inventory_configuration = Some(InventoryConfigurationDeserializer::deserialize("InventoryConfiguration", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketInventoryConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: InventoryId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketInventoryConfigurationRequestDeserializer;
impl GetBucketInventoryConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketInventoryConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketInventoryConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketLifecycleConfigurationOutput {
    #[serde(rename = "Rules")]
    pub rules: Option<LifecycleRules>,
}
#[allow(dead_code)]
pub struct GetBucketLifecycleConfigurationOutputDeserializer;
impl GetBucketLifecycleConfigurationOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLifecycleConfigurationOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketLifecycleConfigurationOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Rules" => {
                        obj.rules
                            .get_or_insert(vec![])
                            .extend(LifecycleRulesDeserializer::deserialize("Rules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketLifecycleConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketLifecycleConfigurationRequestDeserializer;
impl GetBucketLifecycleConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLifecycleConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketLifecycleConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketLifecycleOutput {
    #[serde(rename = "Rules")]
    pub rules: Option<Rules>,
}
#[allow(dead_code)]
pub struct GetBucketLifecycleOutputDeserializer;
impl GetBucketLifecycleOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLifecycleOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketLifecycleOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Rules" => {
                        obj.rules
                            .get_or_insert(vec![])
                            .extend(RulesDeserializer::deserialize("Rules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketLifecycleRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketLifecycleRequestDeserializer;
impl GetBucketLifecycleRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLifecycleRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketLifecycleRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketLocationOutput {
    #[serde(rename = "LocationConstraint")]
    pub location_constraint: Option<BucketLocationConstraint>,
}
#[allow(dead_code)]
pub struct GetBucketLocationOutputDeserializer;
impl GetBucketLocationOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLocationOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketLocationOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LocationConstraint" => {
                        obj.location_constraint = Some(deserialize_primitive("LocationConstraint", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketLocationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketLocationRequestDeserializer;
impl GetBucketLocationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLocationRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketLocationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketLoggingOutput {
    #[serde(rename = "LoggingEnabled")]
    pub logging_enabled: Option<LoggingEnabled>,
}
#[allow(dead_code)]
pub struct GetBucketLoggingOutputDeserializer;
impl GetBucketLoggingOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLoggingOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketLoggingOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LoggingEnabled" => {
                        obj.logging_enabled = Some(LoggingEnabledDeserializer::deserialize("LoggingEnabled", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketLoggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketLoggingRequestDeserializer;
impl GetBucketLoggingRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLoggingRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketLoggingRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketMetricsConfigurationOutput {
    #[serde(rename = "MetricsConfiguration")]
    pub metrics_configuration: Option<MetricsConfiguration>,
}
#[allow(dead_code)]
pub struct GetBucketMetricsConfigurationOutputDeserializer;
impl GetBucketMetricsConfigurationOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketMetricsConfigurationOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketMetricsConfigurationOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "MetricsConfiguration" => {
                        obj.metrics_configuration = Some(MetricsConfigurationDeserializer::deserialize("MetricsConfiguration", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketMetricsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: MetricsId,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketMetricsConfigurationRequestDeserializer;
impl GetBucketMetricsConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketMetricsConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketMetricsConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketNotificationConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketNotificationConfigurationRequestDeserializer;
impl GetBucketNotificationConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketNotificationConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketNotificationConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketOwnershipControlsOutput {
    #[serde(rename = "OwnershipControls")]
    pub ownership_controls: Option<OwnershipControls>,
}
#[allow(dead_code)]
pub struct GetBucketOwnershipControlsOutputDeserializer;
impl GetBucketOwnershipControlsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketOwnershipControlsOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketOwnershipControlsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "OwnershipControls" => {
                        obj.ownership_controls = Some(OwnershipControlsDeserializer::deserialize("OwnershipControls", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketOwnershipControlsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketOwnershipControlsRequestDeserializer;
impl GetBucketOwnershipControlsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketOwnershipControlsRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketOwnershipControlsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketPolicyOutput {
    #[serde(rename = "Policy")]
    pub policy: Option<Policy>,
}
#[allow(dead_code)]
pub struct GetBucketPolicyOutputDeserializer;
impl GetBucketPolicyOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketPolicyOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketPolicyOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Policy" => {
                        obj.policy = Some(deserialize_primitive("Policy", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketPolicyRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketPolicyRequestDeserializer;
impl GetBucketPolicyRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketPolicyRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketPolicyRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketPolicyStatusOutput {
    #[serde(rename = "PolicyStatus")]
    pub policy_status: Option<PolicyStatus>,
}
#[allow(dead_code)]
pub struct GetBucketPolicyStatusOutputDeserializer;
impl GetBucketPolicyStatusOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketPolicyStatusOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketPolicyStatusOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "PolicyStatus" => {
                        obj.policy_status = Some(PolicyStatusDeserializer::deserialize("PolicyStatus", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketPolicyStatusRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketPolicyStatusRequestDeserializer;
impl GetBucketPolicyStatusRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketPolicyStatusRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketPolicyStatusRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketReplicationOutput {
    #[serde(rename = "ReplicationConfiguration")]
    pub replication_configuration: Option<ReplicationConfiguration>,
}
#[allow(dead_code)]
pub struct GetBucketReplicationOutputDeserializer;
impl GetBucketReplicationOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketReplicationOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketReplicationOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ReplicationConfiguration" => {
                        obj.replication_configuration = Some(ReplicationConfigurationDeserializer::deserialize("ReplicationConfiguration", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketReplicationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketReplicationRequestDeserializer;
impl GetBucketReplicationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketReplicationRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketReplicationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketRequestPaymentOutput {
    #[serde(rename = "Payer")]
    pub payer: Option<Payer>,
}
#[allow(dead_code)]
pub struct GetBucketRequestPaymentOutputDeserializer;
impl GetBucketRequestPaymentOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketRequestPaymentOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketRequestPaymentOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Payer" => {
                        obj.payer = Some(deserialize_primitive("Payer", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketRequestPaymentRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketRequestPaymentRequestDeserializer;
impl GetBucketRequestPaymentRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketRequestPaymentRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketRequestPaymentRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketTaggingOutput {
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
}
#[allow(dead_code)]
pub struct GetBucketTaggingOutputDeserializer;
impl GetBucketTaggingOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketTaggingOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketTaggingOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TagSet" => {
                        obj.tag_set
                            .extend(TagSetDeserializer::deserialize("TagSet", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketTaggingRequestDeserializer;
impl GetBucketTaggingRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketTaggingRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketTaggingRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketVersioningOutput {
    #[serde(rename = "Status")]
    pub status: Option<BucketVersioningStatus>,
    #[serde(rename = "MFADelete")]
    pub mfa_delete: Option<MFADeleteStatus>,
}
#[allow(dead_code)]
pub struct GetBucketVersioningOutputDeserializer;
impl GetBucketVersioningOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketVersioningOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketVersioningOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = Some(deserialize_primitive("Status", stack, Ok)?);
                    }
                    "MFADelete" => {
                        obj.mfa_delete = Some(deserialize_primitive("MFADelete", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketVersioningRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketVersioningRequestDeserializer;
impl GetBucketVersioningRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketVersioningRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketVersioningRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct GetBucketWebsiteOutputDeserializer;
impl GetBucketWebsiteOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketWebsiteOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketWebsiteOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RedirectAllRequestsTo" => {
                        obj.redirect_all_requests_to = Some(RedirectAllRequestsToDeserializer::deserialize("RedirectAllRequestsTo", stack)?);
                    }
                    "IndexDocument" => {
                        obj.index_document = Some(IndexDocumentDeserializer::deserialize("IndexDocument", stack)?);
                    }
                    "ErrorDocument" => {
                        obj.error_document = Some(ErrorDocumentDeserializer::deserialize("ErrorDocument", stack)?);
                    }
                    "RoutingRules" => {
                        obj.routing_rules
                            .get_or_insert(vec![])
                            .extend(RoutingRulesDeserializer::deserialize("RoutingRules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetBucketWebsiteRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetBucketWebsiteRequestDeserializer;
impl GetBucketWebsiteRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketWebsiteRequest, XmlParseError> {
        deserialize_elements::<_, GetBucketWebsiteRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectAclOutput {
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    #[serde(rename = "Grants")]
    pub grants: Option<Grants>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}
#[allow(dead_code)]
pub struct GetObjectAclOutputDeserializer;
impl GetObjectAclOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectAclOutput, XmlParseError> {
        deserialize_elements::<_, GetObjectAclOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Owner" => {
                        obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                    }
                    "Grants" => {
                        obj.grants
                            .get_or_insert(vec![])
                            .extend(GrantsDeserializer::deserialize("Grants", stack)?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectAclRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetObjectAclRequestDeserializer;
impl GetObjectAclRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectAclRequest, XmlParseError> {
        deserialize_elements::<_, GetObjectAclRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectLegalHoldOutput {
    #[serde(rename = "LegalHold")]
    pub legal_hold: Option<ObjectLockLegalHold>,
}
#[allow(dead_code)]
pub struct GetObjectLegalHoldOutputDeserializer;
impl GetObjectLegalHoldOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectLegalHoldOutput, XmlParseError> {
        deserialize_elements::<_, GetObjectLegalHoldOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LegalHold" => {
                        obj.legal_hold = Some(ObjectLockLegalHoldDeserializer::deserialize("LegalHold", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectLegalHoldRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetObjectLegalHoldRequestDeserializer;
impl GetObjectLegalHoldRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectLegalHoldRequest, XmlParseError> {
        deserialize_elements::<_, GetObjectLegalHoldRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectLockConfigurationOutput {
    #[serde(rename = "ObjectLockConfiguration")]
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
}
#[allow(dead_code)]
pub struct GetObjectLockConfigurationOutputDeserializer;
impl GetObjectLockConfigurationOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectLockConfigurationOutput, XmlParseError> {
        deserialize_elements::<_, GetObjectLockConfigurationOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ObjectLockConfiguration" => {
                        obj.object_lock_configuration = Some(ObjectLockConfigurationDeserializer::deserialize("ObjectLockConfiguration", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectLockConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetObjectLockConfigurationRequestDeserializer;
impl GetObjectLockConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectLockConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, GetObjectLockConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct GetObjectOutputDeserializer;
impl GetObjectOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectOutput, XmlParseError> {
        deserialize_elements::<_, GetObjectOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Body" => {
                        obj.body = Some(Default::default());
                    }
                    "DeleteMarker" => {
                        obj.delete_marker = Some(deserialize_primitive("DeleteMarker", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "AcceptRanges" => {
                        obj.accept_ranges = Some(deserialize_primitive("AcceptRanges", stack, Ok)?);
                    }
                    "Expiration" => {
                        obj.expiration = Some(deserialize_primitive("Expiration", stack, Ok)?);
                    }
                    "Restore" => {
                        obj.restore = Some(deserialize_primitive("Restore", stack, Ok)?);
                    }
                    "LastModified" => {
                        obj.last_modified = Some(deserialize_primitive("LastModified", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "ContentLength" => {
                        obj.content_length = Some(deserialize_primitive("ContentLength", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    "ETag" => {
                        obj.e_tag = Some(deserialize_primitive("ETag", stack, Ok)?);
                    }
                    "MissingMeta" => {
                        obj.missing_meta = Some(deserialize_primitive("MissingMeta", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "CacheControl" => {
                        obj.cache_control = Some(deserialize_primitive("CacheControl", stack, Ok)?);
                    }
                    "ContentDisposition" => {
                        obj.content_disposition = Some(deserialize_primitive("ContentDisposition", stack, Ok)?);
                    }
                    "ContentEncoding" => {
                        obj.content_encoding = Some(deserialize_primitive("ContentEncoding", stack, Ok)?);
                    }
                    "ContentLanguage" => {
                        obj.content_language = Some(deserialize_primitive("ContentLanguage", stack, Ok)?);
                    }
                    "ContentRange" => {
                        obj.content_range = Some(deserialize_primitive("ContentRange", stack, Ok)?);
                    }
                    "ContentType" => {
                        obj.content_type = Some(deserialize_primitive("ContentType", stack, Ok)?);
                    }
                    "Expires" => {
                        obj.expires = Some(deserialize_primitive("Expires", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "WebsiteRedirectLocation" => {
                        obj.website_redirect_location = Some(deserialize_primitive("WebsiteRedirectLocation", stack, Ok)?);
                    }
                    "ServerSideEncryption" => {
                        obj.server_side_encryption = Some(deserialize_primitive("ServerSideEncryption", stack, Ok)?);
                    }
                    "Metadata" => {
                        obj.metadata = Some(Default::default());
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "SSEKMSKeyId" => {
                        obj.ssekms_key_id = Some(deserialize_primitive("SSEKMSKeyId", stack, Ok)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    "ReplicationStatus" => {
                        obj.replication_status = Some(deserialize_primitive("ReplicationStatus", stack, Ok)?);
                    }
                    "PartsCount" => {
                        obj.parts_count = Some(deserialize_primitive("PartsCount", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "TagCount" => {
                        obj.tag_count = Some(deserialize_primitive("TagCount", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "ObjectLockMode" => {
                        obj.object_lock_mode = Some(deserialize_primitive("ObjectLockMode", stack, Ok)?);
                    }
                    "ObjectLockRetainUntilDate" => {
                        obj.object_lock_retain_until_date = Some(deserialize_primitive("ObjectLockRetainUntilDate", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "ObjectLockLegalHoldStatus" => {
                        obj.object_lock_legal_hold_status = Some(deserialize_primitive("ObjectLockLegalHoldStatus", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "IfMatch")]
    pub if_match: Option<IfMatch>,
    #[serde(rename = "IfModifiedSince")]
    pub if_modified_since: Option<IfModifiedSince>,
    #[serde(rename = "IfNoneMatch")]
    pub if_none_match: Option<IfNoneMatch>,
    #[serde(rename = "IfUnmodifiedSince")]
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Range")]
    pub range: Option<Range>,
    #[serde(rename = "ResponseCacheControl")]
    pub response_cache_control: Option<ResponseCacheControl>,
    #[serde(rename = "ResponseContentDisposition")]
    pub response_content_disposition: Option<ResponseContentDisposition>,
    #[serde(rename = "ResponseContentEncoding")]
    pub response_content_encoding: Option<ResponseContentEncoding>,
    #[serde(rename = "ResponseContentLanguage")]
    pub response_content_language: Option<ResponseContentLanguage>,
    #[serde(rename = "ResponseContentType")]
    pub response_content_type: Option<ResponseContentType>,
    #[serde(rename = "ResponseExpires")]
    pub response_expires: Option<ResponseExpires>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: Option<SSECustomerKey>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "PartNumber")]
    pub part_number: Option<PartNumber>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetObjectRequestDeserializer;
impl GetObjectRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectRequest, XmlParseError> {
        deserialize_elements::<_, GetObjectRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "IfMatch" => {
                        obj.if_match = Some(deserialize_primitive("IfMatch", stack, Ok)?);
                    }
                    "IfModifiedSince" => {
                        obj.if_modified_since = Some(deserialize_primitive("IfModifiedSince", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "IfNoneMatch" => {
                        obj.if_none_match = Some(deserialize_primitive("IfNoneMatch", stack, Ok)?);
                    }
                    "IfUnmodifiedSince" => {
                        obj.if_unmodified_since = Some(deserialize_primitive("IfUnmodifiedSince", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "Range" => {
                        obj.range = Some(deserialize_primitive("Range", stack, Ok)?);
                    }
                    "ResponseCacheControl" => {
                        obj.response_cache_control = Some(deserialize_primitive("ResponseCacheControl", stack, Ok)?);
                    }
                    "ResponseContentDisposition" => {
                        obj.response_content_disposition = Some(deserialize_primitive("ResponseContentDisposition", stack, Ok)?);
                    }
                    "ResponseContentEncoding" => {
                        obj.response_content_encoding = Some(deserialize_primitive("ResponseContentEncoding", stack, Ok)?);
                    }
                    "ResponseContentLanguage" => {
                        obj.response_content_language = Some(deserialize_primitive("ResponseContentLanguage", stack, Ok)?);
                    }
                    "ResponseContentType" => {
                        obj.response_content_type = Some(deserialize_primitive("ResponseContentType", stack, Ok)?);
                    }
                    "ResponseExpires" => {
                        obj.response_expires = Some(deserialize_primitive("ResponseExpires", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKey" => {
                        obj.sse_customer_key = Some(deserialize_primitive("SSECustomerKey", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "PartNumber" => {
                        obj.part_number = Some(deserialize_primitive("PartNumber", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectRetentionOutput {
    #[serde(rename = "Retention")]
    pub retention: Option<ObjectLockRetention>,
}
#[allow(dead_code)]
pub struct GetObjectRetentionOutputDeserializer;
impl GetObjectRetentionOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectRetentionOutput, XmlParseError> {
        deserialize_elements::<_, GetObjectRetentionOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Retention" => {
                        obj.retention = Some(ObjectLockRetentionDeserializer::deserialize("Retention", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectRetentionRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetObjectRetentionRequestDeserializer;
impl GetObjectRetentionRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectRetentionRequest, XmlParseError> {
        deserialize_elements::<_, GetObjectRetentionRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectTaggingOutput {
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
}
#[allow(dead_code)]
pub struct GetObjectTaggingOutputDeserializer;
impl GetObjectTaggingOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectTaggingOutput, XmlParseError> {
        deserialize_elements::<_, GetObjectTaggingOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "TagSet" => {
                        obj.tag_set
                            .extend(TagSetDeserializer::deserialize("TagSet", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetObjectTaggingRequestDeserializer;
impl GetObjectTaggingRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectTaggingRequest, XmlParseError> {
        deserialize_elements::<_, GetObjectTaggingRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectTorrentOutput {
    #[serde(rename = "Body")]
    pub body: Option<Body>,
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}
#[allow(dead_code)]
pub struct GetObjectTorrentOutputDeserializer;
impl GetObjectTorrentOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectTorrentOutput, XmlParseError> {
        deserialize_elements::<_, GetObjectTorrentOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Body" => {
                        obj.body = Some(Default::default());
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetObjectTorrentRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetObjectTorrentRequestDeserializer;
impl GetObjectTorrentRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectTorrentRequest, XmlParseError> {
        deserialize_elements::<_, GetObjectTorrentRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetPublicAccessBlockOutput {
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}
#[allow(dead_code)]
pub struct GetPublicAccessBlockOutputDeserializer;
impl GetPublicAccessBlockOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetPublicAccessBlockOutput, XmlParseError> {
        deserialize_elements::<_, GetPublicAccessBlockOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "PublicAccessBlockConfiguration" => {
                        obj.public_access_block_configuration = Some(PublicAccessBlockConfigurationDeserializer::deserialize("PublicAccessBlockConfiguration", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetPublicAccessBlockRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct GetPublicAccessBlockRequestDeserializer;
impl GetPublicAccessBlockRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetPublicAccessBlockRequest, XmlParseError> {
        deserialize_elements::<_, GetPublicAccessBlockRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlacierJobParameters {
    #[serde(rename = "Tier")]
    pub tier: Tier,
}
#[allow(dead_code)]
pub struct GlacierJobParametersDeserializer;
impl GlacierJobParametersDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GlacierJobParameters, XmlParseError> {
        deserialize_elements::<_, GlacierJobParameters, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Tier" => {
                        obj.tier = deserialize_primitive("Tier", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Grant {
    #[serde(rename = "Grantee")]
    pub grantee: Option<Grantee>,
    #[serde(rename = "Permission")]
    pub permission: Option<Permission>,
}
#[allow(dead_code)]
pub struct GrantDeserializer;
impl GrantDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Grant, XmlParseError> {
        deserialize_elements::<_, Grant, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Grantee" => {
                        obj.grantee = Some(GranteeDeserializer::deserialize("Grantee", stack)?);
                    }
                    "Permission" => {
                        obj.permission = Some(deserialize_primitive("Permission", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Grantee {
    #[serde(rename = "DisplayName")]
    pub display_name: Option<DisplayName>,
    #[serde(rename = "EmailAddress")]
    pub email_address: Option<EmailAddress>,
    #[serde(rename = "ID")]
    pub id: Option<ID>,
    #[serde(rename = "Type")]
    pub r#type: Type,
    #[serde(rename = "URI")]
    pub uri: Option<URI>,
}
#[allow(dead_code)]
pub struct GranteeDeserializer;
impl GranteeDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Grantee, XmlParseError> {
        deserialize_elements::<_, Grantee, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DisplayName" => {
                        obj.display_name = Some(deserialize_primitive("DisplayName", stack, Ok)?);
                    }
                    "EmailAddress" => {
                        obj.email_address = Some(deserialize_primitive("EmailAddress", stack, Ok)?);
                    }
                    "ID" => {
                        obj.id = Some(deserialize_primitive("ID", stack, Ok)?);
                    }
                    "Type" => {
                        obj.r#type = deserialize_primitive("Type", stack, Ok)?;
                    }
                    "URI" => {
                        obj.uri = Some(deserialize_primitive("URI", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HeadBucketRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct HeadBucketRequestDeserializer;
impl HeadBucketRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HeadBucketRequest, XmlParseError> {
        deserialize_elements::<_, HeadBucketRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct HeadObjectOutputDeserializer;
impl HeadObjectOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HeadObjectOutput, XmlParseError> {
        deserialize_elements::<_, HeadObjectOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DeleteMarker" => {
                        obj.delete_marker = Some(deserialize_primitive("DeleteMarker", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "AcceptRanges" => {
                        obj.accept_ranges = Some(deserialize_primitive("AcceptRanges", stack, Ok)?);
                    }
                    "Expiration" => {
                        obj.expiration = Some(deserialize_primitive("Expiration", stack, Ok)?);
                    }
                    "Restore" => {
                        obj.restore = Some(deserialize_primitive("Restore", stack, Ok)?);
                    }
                    "ArchiveStatus" => {
                        obj.archive_status = Some(deserialize_primitive("ArchiveStatus", stack, Ok)?);
                    }
                    "LastModified" => {
                        obj.last_modified = Some(deserialize_primitive("LastModified", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "ContentLength" => {
                        obj.content_length = Some(deserialize_primitive("ContentLength", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    "ETag" => {
                        obj.e_tag = Some(deserialize_primitive("ETag", stack, Ok)?);
                    }
                    "MissingMeta" => {
                        obj.missing_meta = Some(deserialize_primitive("MissingMeta", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "CacheControl" => {
                        obj.cache_control = Some(deserialize_primitive("CacheControl", stack, Ok)?);
                    }
                    "ContentDisposition" => {
                        obj.content_disposition = Some(deserialize_primitive("ContentDisposition", stack, Ok)?);
                    }
                    "ContentEncoding" => {
                        obj.content_encoding = Some(deserialize_primitive("ContentEncoding", stack, Ok)?);
                    }
                    "ContentLanguage" => {
                        obj.content_language = Some(deserialize_primitive("ContentLanguage", stack, Ok)?);
                    }
                    "ContentType" => {
                        obj.content_type = Some(deserialize_primitive("ContentType", stack, Ok)?);
                    }
                    "Expires" => {
                        obj.expires = Some(deserialize_primitive("Expires", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "WebsiteRedirectLocation" => {
                        obj.website_redirect_location = Some(deserialize_primitive("WebsiteRedirectLocation", stack, Ok)?);
                    }
                    "ServerSideEncryption" => {
                        obj.server_side_encryption = Some(deserialize_primitive("ServerSideEncryption", stack, Ok)?);
                    }
                    "Metadata" => {
                        obj.metadata = Some(Default::default());
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "SSEKMSKeyId" => {
                        obj.ssekms_key_id = Some(deserialize_primitive("SSEKMSKeyId", stack, Ok)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    "ReplicationStatus" => {
                        obj.replication_status = Some(deserialize_primitive("ReplicationStatus", stack, Ok)?);
                    }
                    "PartsCount" => {
                        obj.parts_count = Some(deserialize_primitive("PartsCount", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "ObjectLockMode" => {
                        obj.object_lock_mode = Some(deserialize_primitive("ObjectLockMode", stack, Ok)?);
                    }
                    "ObjectLockRetainUntilDate" => {
                        obj.object_lock_retain_until_date = Some(deserialize_primitive("ObjectLockRetainUntilDate", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "ObjectLockLegalHoldStatus" => {
                        obj.object_lock_legal_hold_status = Some(deserialize_primitive("ObjectLockLegalHoldStatus", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HeadObjectRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "IfMatch")]
    pub if_match: Option<IfMatch>,
    #[serde(rename = "IfModifiedSince")]
    pub if_modified_since: Option<IfModifiedSince>,
    #[serde(rename = "IfNoneMatch")]
    pub if_none_match: Option<IfNoneMatch>,
    #[serde(rename = "IfUnmodifiedSince")]
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Range")]
    pub range: Option<Range>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: Option<SSECustomerKey>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "PartNumber")]
    pub part_number: Option<PartNumber>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct HeadObjectRequestDeserializer;
impl HeadObjectRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HeadObjectRequest, XmlParseError> {
        deserialize_elements::<_, HeadObjectRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "IfMatch" => {
                        obj.if_match = Some(deserialize_primitive("IfMatch", stack, Ok)?);
                    }
                    "IfModifiedSince" => {
                        obj.if_modified_since = Some(deserialize_primitive("IfModifiedSince", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "IfNoneMatch" => {
                        obj.if_none_match = Some(deserialize_primitive("IfNoneMatch", stack, Ok)?);
                    }
                    "IfUnmodifiedSince" => {
                        obj.if_unmodified_since = Some(deserialize_primitive("IfUnmodifiedSince", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "Range" => {
                        obj.range = Some(deserialize_primitive("Range", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKey" => {
                        obj.sse_customer_key = Some(deserialize_primitive("SSECustomerKey", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "PartNumber" => {
                        obj.part_number = Some(deserialize_primitive("PartNumber", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IndexDocument {
    #[serde(rename = "Suffix")]
    pub suffix: Suffix,
}
#[allow(dead_code)]
pub struct IndexDocumentDeserializer;
impl IndexDocumentDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IndexDocument, XmlParseError> {
        deserialize_elements::<_, IndexDocument, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Suffix" => {
                        obj.suffix = deserialize_primitive("Suffix", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Initiator {
    #[serde(rename = "ID")]
    pub id: Option<ID>,
    #[serde(rename = "DisplayName")]
    pub display_name: Option<DisplayName>,
}
#[allow(dead_code)]
pub struct InitiatorDeserializer;
impl InitiatorDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Initiator, XmlParseError> {
        deserialize_elements::<_, Initiator, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ID" => {
                        obj.id = Some(deserialize_primitive("ID", stack, Ok)?);
                    }
                    "DisplayName" => {
                        obj.display_name = Some(deserialize_primitive("DisplayName", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct InputSerializationDeserializer;
impl InputSerializationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InputSerialization, XmlParseError> {
        deserialize_elements::<_, InputSerialization, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CSV" => {
                        obj.csv = Some(CSVInputDeserializer::deserialize("CSV", stack)?);
                    }
                    "CompressionType" => {
                        obj.compression_type = Some(deserialize_primitive("CompressionType", stack, Ok)?);
                    }
                    "JSON" => {
                        obj.json = Some(JSONInputDeserializer::deserialize("JSON", stack)?);
                    }
                    "Parquet" => {
                        obj.parquet = Some(ParquetInputDeserializer::deserialize("Parquet", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IntelligentTieringAndOperator {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagSet>,
}
#[allow(dead_code)]
pub struct IntelligentTieringAndOperatorDeserializer;
impl IntelligentTieringAndOperatorDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IntelligentTieringAndOperator, XmlParseError> {
        deserialize_elements::<_, IntelligentTieringAndOperator, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Tags" => {
                        obj.tags
                            .get_or_insert(vec![])
                            .extend(TagSetDeserializer::deserialize("Tags", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IntelligentTieringConfiguration {
    #[serde(rename = "Id")]
    pub id: IntelligentTieringId,
    #[serde(rename = "Filter")]
    pub filter: Option<IntelligentTieringFilter>,
    #[serde(rename = "Status")]
    pub status: IntelligentTieringStatus,
    #[serde(rename = "Tierings")]
    pub tierings: TieringList,
}
#[allow(dead_code)]
pub struct IntelligentTieringConfigurationDeserializer;
impl IntelligentTieringConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IntelligentTieringConfiguration, XmlParseError> {
        deserialize_elements::<_, IntelligentTieringConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "Filter" => {
                        obj.filter = Some(IntelligentTieringFilterDeserializer::deserialize("Filter", stack)?);
                    }
                    "Status" => {
                        obj.status = deserialize_primitive("Status", stack, Ok)?;
                    }
                    "Tierings" => {
                        obj.tierings
                            .extend(TieringListDeserializer::deserialize("Tierings", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IntelligentTieringFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tag")]
    pub tag: Option<Tag>,
    #[serde(rename = "And")]
    pub and: Option<IntelligentTieringAndOperator>,
}
#[allow(dead_code)]
pub struct IntelligentTieringFilterDeserializer;
impl IntelligentTieringFilterDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IntelligentTieringFilter, XmlParseError> {
        deserialize_elements::<_, IntelligentTieringFilter, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Tag" => {
                        obj.tag = Some(TagDeserializer::deserialize("Tag", stack)?);
                    }
                    "And" => {
                        obj.and = Some(IntelligentTieringAndOperatorDeserializer::deserialize("And", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InvalidObjectState {
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    #[serde(rename = "AccessTier")]
    pub access_tier: Option<IntelligentTieringAccessTier>,
}
#[allow(dead_code)]
pub struct InvalidObjectStateDeserializer;
impl InvalidObjectStateDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InvalidObjectState, XmlParseError> {
        deserialize_elements::<_, InvalidObjectState, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    "AccessTier" => {
                        obj.access_tier = Some(deserialize_primitive("AccessTier", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InventoryConfiguration {
    #[serde(rename = "Destination")]
    pub destination: InventoryDestination,
    #[serde(rename = "IsEnabled")]
    pub is_enabled: IsEnabled,
    #[serde(rename = "Filter")]
    pub filter: Option<InventoryFilter>,
    #[serde(rename = "Id")]
    pub id: InventoryId,
    #[serde(rename = "IncludedObjectVersions")]
    pub included_object_versions: InventoryIncludedObjectVersions,
    #[serde(rename = "OptionalFields")]
    pub optional_fields: Option<InventoryOptionalFields>,
    #[serde(rename = "Schedule")]
    pub schedule: InventorySchedule,
}
#[allow(dead_code)]
pub struct InventoryConfigurationDeserializer;
impl InventoryConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryConfiguration, XmlParseError> {
        deserialize_elements::<_, InventoryConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Destination" => {
                        obj.destination = InventoryDestinationDeserializer::deserialize("Destination", stack)?;
                    }
                    "IsEnabled" => {
                        obj.is_enabled = deserialize_primitive("IsEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?;
                    }
                    "Filter" => {
                        obj.filter = Some(InventoryFilterDeserializer::deserialize("Filter", stack)?);
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "IncludedObjectVersions" => {
                        obj.included_object_versions = deserialize_primitive("IncludedObjectVersions", stack, Ok)?;
                    }
                    "OptionalFields" => {
                        obj.optional_fields
                            .get_or_insert(vec![])
                            .extend(InventoryOptionalFieldsDeserializer::deserialize("OptionalFields", stack)?);
                    }
                    "Schedule" => {
                        obj.schedule = InventoryScheduleDeserializer::deserialize("Schedule", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InventoryDestination {
    #[serde(rename = "S3BucketDestination")]
    pub s3_bucket_destination: InventoryS3BucketDestination,
}
#[allow(dead_code)]
pub struct InventoryDestinationDeserializer;
impl InventoryDestinationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryDestination, XmlParseError> {
        deserialize_elements::<_, InventoryDestination, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "S3BucketDestination" => {
                        obj.s3_bucket_destination = InventoryS3BucketDestinationDeserializer::deserialize("S3BucketDestination", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InventoryEncryption {
    #[serde(rename = "SSES3")]
    pub sses3: Option<SSES3>,
    #[serde(rename = "SSEKMS")]
    pub ssekms: Option<SSEKMS>,
}
#[allow(dead_code)]
pub struct InventoryEncryptionDeserializer;
impl InventoryEncryptionDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryEncryption, XmlParseError> {
        deserialize_elements::<_, InventoryEncryption, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "SSES3" => {
                        obj.sses3 = Some(SSES3Deserializer::deserialize("SSES3", stack)?);
                    }
                    "SSEKMS" => {
                        obj.ssekms = Some(SSEKMSDeserializer::deserialize("SSEKMS", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InventoryFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
}
#[allow(dead_code)]
pub struct InventoryFilterDeserializer;
impl InventoryFilterDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryFilter, XmlParseError> {
        deserialize_elements::<_, InventoryFilter, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = deserialize_primitive("Prefix", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InventoryS3BucketDestination {
    #[serde(rename = "AccountId")]
    pub account_id: Option<AccountId>,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Format")]
    pub format: InventoryFormat,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Encryption")]
    pub encryption: Option<InventoryEncryption>,
}
#[allow(dead_code)]
pub struct InventoryS3BucketDestinationDeserializer;
impl InventoryS3BucketDestinationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryS3BucketDestination, XmlParseError> {
        deserialize_elements::<_, InventoryS3BucketDestination, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AccountId" => {
                        obj.account_id = Some(deserialize_primitive("AccountId", stack, Ok)?);
                    }
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Format" => {
                        obj.format = deserialize_primitive("Format", stack, Ok)?;
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Encryption" => {
                        obj.encryption = Some(InventoryEncryptionDeserializer::deserialize("Encryption", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InventorySchedule {
    #[serde(rename = "Frequency")]
    pub frequency: InventoryFrequency,
}
#[allow(dead_code)]
pub struct InventoryScheduleDeserializer;
impl InventoryScheduleDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventorySchedule, XmlParseError> {
        deserialize_elements::<_, InventorySchedule, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Frequency" => {
                        obj.frequency = deserialize_primitive("Frequency", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JSONInput {
    #[serde(rename = "Type")]
    pub r#type: Option<JSONType>,
}
#[allow(dead_code)]
pub struct JSONInputDeserializer;
impl JSONInputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<JSONInput, XmlParseError> {
        deserialize_elements::<_, JSONInput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Type" => {
                        obj.r#type = Some(deserialize_primitive("Type", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JSONOutput {
    #[serde(rename = "RecordDelimiter")]
    pub record_delimiter: Option<RecordDelimiter>,
}
#[allow(dead_code)]
pub struct JSONOutputDeserializer;
impl JSONOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<JSONOutput, XmlParseError> {
        deserialize_elements::<_, JSONOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RecordDelimiter" => {
                        obj.record_delimiter = Some(deserialize_primitive("RecordDelimiter", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LambdaFunctionConfiguration {
    #[serde(rename = "Id")]
    pub id: Option<NotificationId>,
    #[serde(rename = "LambdaFunctionArn")]
    pub lambda_function_arn: LambdaFunctionArn,
    #[serde(rename = "Events")]
    pub events: EventList,
    #[serde(rename = "Filter")]
    pub filter: Option<NotificationConfigurationFilter>,
}
#[allow(dead_code)]
pub struct LambdaFunctionConfigurationDeserializer;
impl LambdaFunctionConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LambdaFunctionConfiguration, XmlParseError> {
        deserialize_elements::<_, LambdaFunctionConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Id" => {
                        obj.id = Some(deserialize_primitive("Id", stack, Ok)?);
                    }
                    "LambdaFunctionArn" => {
                        obj.lambda_function_arn = deserialize_primitive("LambdaFunctionArn", stack, Ok)?;
                    }
                    "Events" => {
                        obj.events
                            .extend(EventListDeserializer::deserialize("Events", stack)?);
                    }
                    "Filter" => {
                        obj.filter = Some(NotificationConfigurationFilterDeserializer::deserialize("Filter", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LifecycleConfiguration {
    #[serde(rename = "Rules")]
    pub rules: Rules,
}
#[allow(dead_code)]
pub struct LifecycleConfigurationDeserializer;
impl LifecycleConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleConfiguration, XmlParseError> {
        deserialize_elements::<_, LifecycleConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Rules" => {
                        obj.rules
                            .extend(RulesDeserializer::deserialize("Rules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LifecycleExpiration {
    #[serde(rename = "Date")]
    pub date: Option<Date>,
    #[serde(rename = "Days")]
    pub days: Option<Days>,
    #[serde(rename = "ExpiredObjectDeleteMarker")]
    pub expired_object_delete_marker: Option<ExpiredObjectDeleteMarker>,
}
#[allow(dead_code)]
pub struct LifecycleExpirationDeserializer;
impl LifecycleExpirationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleExpiration, XmlParseError> {
        deserialize_elements::<_, LifecycleExpiration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Date" => {
                        obj.date = Some(deserialize_primitive("Date", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "Days" => {
                        obj.days = Some(deserialize_primitive("Days", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "ExpiredObjectDeleteMarker" => {
                        obj.expired_object_delete_marker = Some(deserialize_primitive("ExpiredObjectDeleteMarker", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LifecycleRule {
    #[serde(rename = "Expiration")]
    pub expiration: Option<LifecycleExpiration>,
    #[serde(rename = "ID")]
    pub id: Option<ID>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Filter")]
    pub filter: Option<LifecycleRuleFilter>,
    #[serde(rename = "Status")]
    pub status: ExpirationStatus,
    #[serde(rename = "Transitions")]
    pub transitions: Option<TransitionList>,
    #[serde(rename = "NoncurrentVersionTransitions")]
    pub noncurrent_version_transitions: Option<NoncurrentVersionTransitionList>,
    #[serde(rename = "NoncurrentVersionExpiration")]
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    #[serde(rename = "AbortIncompleteMultipartUpload")]
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
}
#[allow(dead_code)]
pub struct LifecycleRuleDeserializer;
impl LifecycleRuleDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleRule, XmlParseError> {
        deserialize_elements::<_, LifecycleRule, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Expiration" => {
                        obj.expiration = Some(LifecycleExpirationDeserializer::deserialize("Expiration", stack)?);
                    }
                    "ID" => {
                        obj.id = Some(deserialize_primitive("ID", stack, Ok)?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Filter" => {
                        obj.filter = Some(LifecycleRuleFilterDeserializer::deserialize("Filter", stack)?);
                    }
                    "Status" => {
                        obj.status = deserialize_primitive("Status", stack, Ok)?;
                    }
                    "Transitions" => {
                        obj.transitions
                            .get_or_insert(vec![])
                            .extend(TransitionListDeserializer::deserialize("Transitions", stack)?);
                    }
                    "NoncurrentVersionTransitions" => {
                        obj.noncurrent_version_transitions
                            .get_or_insert(vec![])
                            .extend(NoncurrentVersionTransitionListDeserializer::deserialize("NoncurrentVersionTransitions", stack)?);
                    }
                    "NoncurrentVersionExpiration" => {
                        obj.noncurrent_version_expiration = Some(NoncurrentVersionExpirationDeserializer::deserialize("NoncurrentVersionExpiration", stack)?);
                    }
                    "AbortIncompleteMultipartUpload" => {
                        obj.abort_incomplete_multipart_upload = Some(AbortIncompleteMultipartUploadDeserializer::deserialize("AbortIncompleteMultipartUpload", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LifecycleRuleAndOperator {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagSet>,
}
#[allow(dead_code)]
pub struct LifecycleRuleAndOperatorDeserializer;
impl LifecycleRuleAndOperatorDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleRuleAndOperator, XmlParseError> {
        deserialize_elements::<_, LifecycleRuleAndOperator, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Tags" => {
                        obj.tags
                            .get_or_insert(vec![])
                            .extend(TagSetDeserializer::deserialize("Tags", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LifecycleRuleFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tag")]
    pub tag: Option<Tag>,
    #[serde(rename = "And")]
    pub and: Option<LifecycleRuleAndOperator>,
}
#[allow(dead_code)]
pub struct LifecycleRuleFilterDeserializer;
impl LifecycleRuleFilterDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleRuleFilter, XmlParseError> {
        deserialize_elements::<_, LifecycleRuleFilter, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Tag" => {
                        obj.tag = Some(TagDeserializer::deserialize("Tag", stack)?);
                    }
                    "And" => {
                        obj.and = Some(LifecycleRuleAndOperatorDeserializer::deserialize("And", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ListBucketAnalyticsConfigurationsOutputDeserializer;
impl ListBucketAnalyticsConfigurationsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketAnalyticsConfigurationsOutput, XmlParseError> {
        deserialize_elements::<_, ListBucketAnalyticsConfigurationsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated = Some(deserialize_primitive("IsTruncated", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "ContinuationToken" => {
                        obj.continuation_token = Some(deserialize_primitive("ContinuationToken", stack, Ok)?);
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(deserialize_primitive("NextContinuationToken", stack, Ok)?);
                    }
                    "AnalyticsConfigurationList" => {
                        obj.analytics_configuration_list
                            .get_or_insert(vec![])
                            .extend(AnalyticsConfigurationListDeserializer::deserialize("AnalyticsConfigurationList", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListBucketAnalyticsConfigurationsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<Token>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct ListBucketAnalyticsConfigurationsRequestDeserializer;
impl ListBucketAnalyticsConfigurationsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketAnalyticsConfigurationsRequest, XmlParseError> {
        deserialize_elements::<_, ListBucketAnalyticsConfigurationsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContinuationToken" => {
                        obj.continuation_token = Some(deserialize_primitive("ContinuationToken", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ListBucketIntelligentTieringConfigurationsOutputDeserializer;
impl ListBucketIntelligentTieringConfigurationsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketIntelligentTieringConfigurationsOutput, XmlParseError> {
        deserialize_elements::<_, ListBucketIntelligentTieringConfigurationsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated = Some(deserialize_primitive("IsTruncated", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "ContinuationToken" => {
                        obj.continuation_token = Some(deserialize_primitive("ContinuationToken", stack, Ok)?);
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(deserialize_primitive("NextContinuationToken", stack, Ok)?);
                    }
                    "IntelligentTieringConfigurationList" => {
                        obj.intelligent_tiering_configuration_list
                            .get_or_insert(vec![])
                            .extend(IntelligentTieringConfigurationListDeserializer::deserialize("IntelligentTieringConfigurationList", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListBucketIntelligentTieringConfigurationsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<Token>,
}
#[allow(dead_code)]
pub struct ListBucketIntelligentTieringConfigurationsRequestDeserializer;
impl ListBucketIntelligentTieringConfigurationsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketIntelligentTieringConfigurationsRequest, XmlParseError> {
        deserialize_elements::<_, ListBucketIntelligentTieringConfigurationsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContinuationToken" => {
                        obj.continuation_token = Some(deserialize_primitive("ContinuationToken", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ListBucketInventoryConfigurationsOutputDeserializer;
impl ListBucketInventoryConfigurationsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketInventoryConfigurationsOutput, XmlParseError> {
        deserialize_elements::<_, ListBucketInventoryConfigurationsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ContinuationToken" => {
                        obj.continuation_token = Some(deserialize_primitive("ContinuationToken", stack, Ok)?);
                    }
                    "InventoryConfigurationList" => {
                        obj.inventory_configuration_list
                            .get_or_insert(vec![])
                            .extend(InventoryConfigurationListDeserializer::deserialize("InventoryConfigurationList", stack)?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated = Some(deserialize_primitive("IsTruncated", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(deserialize_primitive("NextContinuationToken", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListBucketInventoryConfigurationsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<Token>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct ListBucketInventoryConfigurationsRequestDeserializer;
impl ListBucketInventoryConfigurationsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketInventoryConfigurationsRequest, XmlParseError> {
        deserialize_elements::<_, ListBucketInventoryConfigurationsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContinuationToken" => {
                        obj.continuation_token = Some(deserialize_primitive("ContinuationToken", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ListBucketMetricsConfigurationsOutputDeserializer;
impl ListBucketMetricsConfigurationsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketMetricsConfigurationsOutput, XmlParseError> {
        deserialize_elements::<_, ListBucketMetricsConfigurationsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated = Some(deserialize_primitive("IsTruncated", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "ContinuationToken" => {
                        obj.continuation_token = Some(deserialize_primitive("ContinuationToken", stack, Ok)?);
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(deserialize_primitive("NextContinuationToken", stack, Ok)?);
                    }
                    "MetricsConfigurationList" => {
                        obj.metrics_configuration_list
                            .get_or_insert(vec![])
                            .extend(MetricsConfigurationListDeserializer::deserialize("MetricsConfigurationList", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListBucketMetricsConfigurationsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<Token>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct ListBucketMetricsConfigurationsRequestDeserializer;
impl ListBucketMetricsConfigurationsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketMetricsConfigurationsRequest, XmlParseError> {
        deserialize_elements::<_, ListBucketMetricsConfigurationsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContinuationToken" => {
                        obj.continuation_token = Some(deserialize_primitive("ContinuationToken", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListBucketsOutput {
    #[serde(rename = "Buckets")]
    pub buckets: Option<Buckets>,
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
}
#[allow(dead_code)]
pub struct ListBucketsOutputDeserializer;
impl ListBucketsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketsOutput, XmlParseError> {
        deserialize_elements::<_, ListBucketsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Buckets" => {
                        obj.buckets
                            .get_or_insert(vec![])
                            .extend(BucketsDeserializer::deserialize("Buckets", stack)?);
                    }
                    "Owner" => {
                        obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ListMultipartUploadsOutputDeserializer;
impl ListMultipartUploadsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListMultipartUploadsOutput, XmlParseError> {
        deserialize_elements::<_, ListMultipartUploadsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = Some(deserialize_primitive("Bucket", stack, Ok)?);
                    }
                    "KeyMarker" => {
                        obj.key_marker = Some(deserialize_primitive("KeyMarker", stack, Ok)?);
                    }
                    "UploadIdMarker" => {
                        obj.upload_id_marker = Some(deserialize_primitive("UploadIdMarker", stack, Ok)?);
                    }
                    "NextKeyMarker" => {
                        obj.next_key_marker = Some(deserialize_primitive("NextKeyMarker", stack, Ok)?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Delimiter" => {
                        obj.delimiter = Some(deserialize_primitive("Delimiter", stack, Ok)?);
                    }
                    "NextUploadIdMarker" => {
                        obj.next_upload_id_marker = Some(deserialize_primitive("NextUploadIdMarker", stack, Ok)?);
                    }
                    "MaxUploads" => {
                        obj.max_uploads = Some(deserialize_primitive("MaxUploads", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated = Some(deserialize_primitive("IsTruncated", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "Uploads" => {
                        obj.uploads
                            .get_or_insert(vec![])
                            .extend(MultipartUploadListDeserializer::deserialize("Uploads", stack)?);
                    }
                    "CommonPrefixes" => {
                        obj.common_prefixes
                            .get_or_insert(vec![])
                            .extend(CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)?);
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(deserialize_primitive("EncodingType", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListMultipartUploadsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<Delimiter>,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<EncodingType>,
    #[serde(rename = "KeyMarker")]
    pub key_marker: Option<KeyMarker>,
    #[serde(rename = "MaxUploads")]
    pub max_uploads: Option<MaxUploads>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "UploadIdMarker")]
    pub upload_id_marker: Option<UploadIdMarker>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct ListMultipartUploadsRequestDeserializer;
impl ListMultipartUploadsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListMultipartUploadsRequest, XmlParseError> {
        deserialize_elements::<_, ListMultipartUploadsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Delimiter" => {
                        obj.delimiter = Some(deserialize_primitive("Delimiter", stack, Ok)?);
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(deserialize_primitive("EncodingType", stack, Ok)?);
                    }
                    "KeyMarker" => {
                        obj.key_marker = Some(deserialize_primitive("KeyMarker", stack, Ok)?);
                    }
                    "MaxUploads" => {
                        obj.max_uploads = Some(deserialize_primitive("MaxUploads", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "UploadIdMarker" => {
                        obj.upload_id_marker = Some(deserialize_primitive("UploadIdMarker", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ListObjectVersionsOutputDeserializer;
impl ListObjectVersionsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectVersionsOutput, XmlParseError> {
        deserialize_elements::<_, ListObjectVersionsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated = Some(deserialize_primitive("IsTruncated", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "KeyMarker" => {
                        obj.key_marker = Some(deserialize_primitive("KeyMarker", stack, Ok)?);
                    }
                    "VersionIdMarker" => {
                        obj.version_id_marker = Some(deserialize_primitive("VersionIdMarker", stack, Ok)?);
                    }
                    "NextKeyMarker" => {
                        obj.next_key_marker = Some(deserialize_primitive("NextKeyMarker", stack, Ok)?);
                    }
                    "NextVersionIdMarker" => {
                        obj.next_version_id_marker = Some(deserialize_primitive("NextVersionIdMarker", stack, Ok)?);
                    }
                    "Versions" => {
                        obj.versions
                            .get_or_insert(vec![])
                            .extend(ObjectVersionListDeserializer::deserialize("Versions", stack)?);
                    }
                    "DeleteMarkers" => {
                        obj.delete_markers
                            .get_or_insert(vec![])
                            .extend(DeleteMarkersDeserializer::deserialize("DeleteMarkers", stack)?);
                    }
                    "Name" => {
                        obj.name = Some(deserialize_primitive("Name", stack, Ok)?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Delimiter" => {
                        obj.delimiter = Some(deserialize_primitive("Delimiter", stack, Ok)?);
                    }
                    "MaxKeys" => {
                        obj.max_keys = Some(deserialize_primitive("MaxKeys", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "CommonPrefixes" => {
                        obj.common_prefixes
                            .get_or_insert(vec![])
                            .extend(CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)?);
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(deserialize_primitive("EncodingType", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListObjectVersionsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<Delimiter>,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<EncodingType>,
    #[serde(rename = "KeyMarker")]
    pub key_marker: Option<KeyMarker>,
    #[serde(rename = "MaxKeys")]
    pub max_keys: Option<MaxKeys>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "VersionIdMarker")]
    pub version_id_marker: Option<VersionIdMarker>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct ListObjectVersionsRequestDeserializer;
impl ListObjectVersionsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectVersionsRequest, XmlParseError> {
        deserialize_elements::<_, ListObjectVersionsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Delimiter" => {
                        obj.delimiter = Some(deserialize_primitive("Delimiter", stack, Ok)?);
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(deserialize_primitive("EncodingType", stack, Ok)?);
                    }
                    "KeyMarker" => {
                        obj.key_marker = Some(deserialize_primitive("KeyMarker", stack, Ok)?);
                    }
                    "MaxKeys" => {
                        obj.max_keys = Some(deserialize_primitive("MaxKeys", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "VersionIdMarker" => {
                        obj.version_id_marker = Some(deserialize_primitive("VersionIdMarker", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ListObjectsOutputDeserializer;
impl ListObjectsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectsOutput, XmlParseError> {
        deserialize_elements::<_, ListObjectsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated = Some(deserialize_primitive("IsTruncated", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "Marker" => {
                        obj.marker = Some(deserialize_primitive("Marker", stack, Ok)?);
                    }
                    "NextMarker" => {
                        obj.next_marker = Some(deserialize_primitive("NextMarker", stack, Ok)?);
                    }
                    "Contents" => {
                        obj.contents
                            .get_or_insert(vec![])
                            .extend(ObjectListDeserializer::deserialize("Contents", stack)?);
                    }
                    "Name" => {
                        obj.name = Some(deserialize_primitive("Name", stack, Ok)?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Delimiter" => {
                        obj.delimiter = Some(deserialize_primitive("Delimiter", stack, Ok)?);
                    }
                    "MaxKeys" => {
                        obj.max_keys = Some(deserialize_primitive("MaxKeys", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "CommonPrefixes" => {
                        obj.common_prefixes
                            .get_or_insert(vec![])
                            .extend(CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)?);
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(deserialize_primitive("EncodingType", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListObjectsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<Delimiter>,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<EncodingType>,
    #[serde(rename = "Marker")]
    pub marker: Option<Marker>,
    #[serde(rename = "MaxKeys")]
    pub max_keys: Option<MaxKeys>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct ListObjectsRequestDeserializer;
impl ListObjectsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectsRequest, XmlParseError> {
        deserialize_elements::<_, ListObjectsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Delimiter" => {
                        obj.delimiter = Some(deserialize_primitive("Delimiter", stack, Ok)?);
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(deserialize_primitive("EncodingType", stack, Ok)?);
                    }
                    "Marker" => {
                        obj.marker = Some(deserialize_primitive("Marker", stack, Ok)?);
                    }
                    "MaxKeys" => {
                        obj.max_keys = Some(deserialize_primitive("MaxKeys", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ListObjectsV2OutputDeserializer;
impl ListObjectsV2OutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectsV2Output, XmlParseError> {
        deserialize_elements::<_, ListObjectsV2Output, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsTruncated" => {
                        obj.is_truncated = Some(deserialize_primitive("IsTruncated", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "Contents" => {
                        obj.contents
                            .get_or_insert(vec![])
                            .extend(ObjectListDeserializer::deserialize("Contents", stack)?);
                    }
                    "Name" => {
                        obj.name = Some(deserialize_primitive("Name", stack, Ok)?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Delimiter" => {
                        obj.delimiter = Some(deserialize_primitive("Delimiter", stack, Ok)?);
                    }
                    "MaxKeys" => {
                        obj.max_keys = Some(deserialize_primitive("MaxKeys", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "CommonPrefixes" => {
                        obj.common_prefixes
                            .get_or_insert(vec![])
                            .extend(CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)?);
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(deserialize_primitive("EncodingType", stack, Ok)?);
                    }
                    "KeyCount" => {
                        obj.key_count = Some(deserialize_primitive("KeyCount", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "ContinuationToken" => {
                        obj.continuation_token = Some(deserialize_primitive("ContinuationToken", stack, Ok)?);
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(deserialize_primitive("NextContinuationToken", stack, Ok)?);
                    }
                    "StartAfter" => {
                        obj.start_after = Some(deserialize_primitive("StartAfter", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListObjectsV2Request {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<Delimiter>,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<EncodingType>,
    #[serde(rename = "MaxKeys")]
    pub max_keys: Option<MaxKeys>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "ContinuationToken")]
    pub continuation_token: Option<Token>,
    #[serde(rename = "FetchOwner")]
    pub fetch_owner: Option<FetchOwner>,
    #[serde(rename = "StartAfter")]
    pub start_after: Option<StartAfter>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct ListObjectsV2RequestDeserializer;
impl ListObjectsV2RequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectsV2Request, XmlParseError> {
        deserialize_elements::<_, ListObjectsV2Request, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Delimiter" => {
                        obj.delimiter = Some(deserialize_primitive("Delimiter", stack, Ok)?);
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(deserialize_primitive("EncodingType", stack, Ok)?);
                    }
                    "MaxKeys" => {
                        obj.max_keys = Some(deserialize_primitive("MaxKeys", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "ContinuationToken" => {
                        obj.continuation_token = Some(deserialize_primitive("ContinuationToken", stack, Ok)?);
                    }
                    "FetchOwner" => {
                        obj.fetch_owner = Some(deserialize_primitive("FetchOwner", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "StartAfter" => {
                        obj.start_after = Some(deserialize_primitive("StartAfter", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ListPartsOutputDeserializer;
impl ListPartsOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListPartsOutput, XmlParseError> {
        deserialize_elements::<_, ListPartsOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AbortDate" => {
                        obj.abort_date = Some(deserialize_primitive("AbortDate", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "AbortRuleId" => {
                        obj.abort_rule_id = Some(deserialize_primitive("AbortRuleId", stack, Ok)?);
                    }
                    "Bucket" => {
                        obj.bucket = Some(deserialize_primitive("Bucket", stack, Ok)?);
                    }
                    "Key" => {
                        obj.key = Some(deserialize_primitive("Key", stack, Ok)?);
                    }
                    "UploadId" => {
                        obj.upload_id = Some(deserialize_primitive("UploadId", stack, Ok)?);
                    }
                    "PartNumberMarker" => {
                        obj.part_number_marker = Some(deserialize_primitive("PartNumberMarker", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "NextPartNumberMarker" => {
                        obj.next_part_number_marker = Some(deserialize_primitive("NextPartNumberMarker", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "MaxParts" => {
                        obj.max_parts = Some(deserialize_primitive("MaxParts", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated = Some(deserialize_primitive("IsTruncated", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "Parts" => {
                        obj.parts
                            .get_or_insert(vec![])
                            .extend(PartsDeserializer::deserialize("Parts", stack)?);
                    }
                    "Initiator" => {
                        obj.initiator = Some(InitiatorDeserializer::deserialize("Initiator", stack)?);
                    }
                    "Owner" => {
                        obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListPartsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "MaxParts")]
    pub max_parts: Option<MaxParts>,
    #[serde(rename = "PartNumberMarker")]
    pub part_number_marker: Option<PartNumberMarker>,
    #[serde(rename = "UploadId")]
    pub upload_id: MultipartUploadId,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct ListPartsRequestDeserializer;
impl ListPartsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListPartsRequest, XmlParseError> {
        deserialize_elements::<_, ListPartsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "MaxParts" => {
                        obj.max_parts = Some(deserialize_primitive("MaxParts", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "PartNumberMarker" => {
                        obj.part_number_marker = Some(deserialize_primitive("PartNumberMarker", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "UploadId" => {
                        obj.upload_id = deserialize_primitive("UploadId", stack, Ok)?;
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoggingEnabled {
    #[serde(rename = "TargetBucket")]
    pub target_bucket: TargetBucket,
    #[serde(rename = "TargetGrants")]
    pub target_grants: Option<TargetGrants>,
    #[serde(rename = "TargetPrefix")]
    pub target_prefix: TargetPrefix,
}
#[allow(dead_code)]
pub struct LoggingEnabledDeserializer;
impl LoggingEnabledDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoggingEnabled, XmlParseError> {
        deserialize_elements::<_, LoggingEnabled, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TargetBucket" => {
                        obj.target_bucket = deserialize_primitive("TargetBucket", stack, Ok)?;
                    }
                    "TargetGrants" => {
                        obj.target_grants
                            .get_or_insert(vec![])
                            .extend(TargetGrantsDeserializer::deserialize("TargetGrants", stack)?);
                    }
                    "TargetPrefix" => {
                        obj.target_prefix = deserialize_primitive("TargetPrefix", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MetadataEntry {
    #[serde(rename = "Name")]
    pub name: Option<MetadataKey>,
    #[serde(rename = "Value")]
    pub value: Option<MetadataValue>,
}
#[allow(dead_code)]
pub struct MetadataEntryDeserializer;
impl MetadataEntryDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetadataEntry, XmlParseError> {
        deserialize_elements::<_, MetadataEntry, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Name" => {
                        obj.name = Some(deserialize_primitive("Name", stack, Ok)?);
                    }
                    "Value" => {
                        obj.value = Some(deserialize_primitive("Value", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Metrics {
    #[serde(rename = "Status")]
    pub status: MetricsStatus,
    #[serde(rename = "EventThreshold")]
    pub event_threshold: Option<ReplicationTimeValue>,
}
#[allow(dead_code)]
pub struct MetricsDeserializer;
impl MetricsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Metrics, XmlParseError> {
        deserialize_elements::<_, Metrics, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = deserialize_primitive("Status", stack, Ok)?;
                    }
                    "EventThreshold" => {
                        obj.event_threshold = Some(ReplicationTimeValueDeserializer::deserialize("EventThreshold", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MetricsAndOperator {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagSet>,
}
#[allow(dead_code)]
pub struct MetricsAndOperatorDeserializer;
impl MetricsAndOperatorDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricsAndOperator, XmlParseError> {
        deserialize_elements::<_, MetricsAndOperator, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Tags" => {
                        obj.tags
                            .get_or_insert(vec![])
                            .extend(TagSetDeserializer::deserialize("Tags", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MetricsConfiguration {
    #[serde(rename = "Id")]
    pub id: MetricsId,
    #[serde(rename = "Filter")]
    pub filter: Option<MetricsFilter>,
}
#[allow(dead_code)]
pub struct MetricsConfigurationDeserializer;
impl MetricsConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricsConfiguration, XmlParseError> {
        deserialize_elements::<_, MetricsConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "Filter" => {
                        obj.filter = Some(MetricsFilterDeserializer::deserialize("Filter", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MetricsFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tag")]
    pub tag: Option<Tag>,
    #[serde(rename = "And")]
    pub and: Option<MetricsAndOperator>,
}
#[allow(dead_code)]
pub struct MetricsFilterDeserializer;
impl MetricsFilterDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricsFilter, XmlParseError> {
        deserialize_elements::<_, MetricsFilter, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Tag" => {
                        obj.tag = Some(TagDeserializer::deserialize("Tag", stack)?);
                    }
                    "And" => {
                        obj.and = Some(MetricsAndOperatorDeserializer::deserialize("And", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct MultipartUploadDeserializer;
impl MultipartUploadDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MultipartUpload, XmlParseError> {
        deserialize_elements::<_, MultipartUpload, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "UploadId" => {
                        obj.upload_id = Some(deserialize_primitive("UploadId", stack, Ok)?);
                    }
                    "Key" => {
                        obj.key = Some(deserialize_primitive("Key", stack, Ok)?);
                    }
                    "Initiated" => {
                        obj.initiated = Some(deserialize_primitive("Initiated", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    "Owner" => {
                        obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                    }
                    "Initiator" => {
                        obj.initiator = Some(InitiatorDeserializer::deserialize("Initiator", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NoSuchBucket {
}
#[allow(dead_code)]
pub struct NoSuchBucketDeserializer;
impl NoSuchBucketDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NoSuchBucket, XmlParseError> {
        deserialize_elements::<_, NoSuchBucket, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NoSuchKey {
}
#[allow(dead_code)]
pub struct NoSuchKeyDeserializer;
impl NoSuchKeyDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NoSuchKey, XmlParseError> {
        deserialize_elements::<_, NoSuchKey, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NoSuchUpload {
}
#[allow(dead_code)]
pub struct NoSuchUploadDeserializer;
impl NoSuchUploadDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NoSuchUpload, XmlParseError> {
        deserialize_elements::<_, NoSuchUpload, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NoncurrentVersionExpiration {
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: Option<Days>,
}
#[allow(dead_code)]
pub struct NoncurrentVersionExpirationDeserializer;
impl NoncurrentVersionExpirationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NoncurrentVersionExpiration, XmlParseError> {
        deserialize_elements::<_, NoncurrentVersionExpiration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NoncurrentDays" => {
                        obj.noncurrent_days = Some(deserialize_primitive("NoncurrentDays", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NoncurrentVersionTransition {
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: Option<Days>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<TransitionStorageClass>,
}
#[allow(dead_code)]
pub struct NoncurrentVersionTransitionDeserializer;
impl NoncurrentVersionTransitionDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NoncurrentVersionTransition, XmlParseError> {
        deserialize_elements::<_, NoncurrentVersionTransition, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NoncurrentDays" => {
                        obj.noncurrent_days = Some(deserialize_primitive("NoncurrentDays", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NotificationConfiguration {
    #[serde(rename = "TopicConfigurations")]
    pub topic_configurations: Option<TopicConfigurationList>,
    #[serde(rename = "QueueConfigurations")]
    pub queue_configurations: Option<QueueConfigurationList>,
    #[serde(rename = "LambdaFunctionConfigurations")]
    pub lambda_function_configurations: Option<LambdaFunctionConfigurationList>,
}
#[allow(dead_code)]
pub struct NotificationConfigurationDeserializer;
impl NotificationConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfiguration, XmlParseError> {
        deserialize_elements::<_, NotificationConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TopicConfigurations" => {
                        obj.topic_configurations
                            .get_or_insert(vec![])
                            .extend(TopicConfigurationListDeserializer::deserialize("TopicConfigurations", stack)?);
                    }
                    "QueueConfigurations" => {
                        obj.queue_configurations
                            .get_or_insert(vec![])
                            .extend(QueueConfigurationListDeserializer::deserialize("QueueConfigurations", stack)?);
                    }
                    "LambdaFunctionConfigurations" => {
                        obj.lambda_function_configurations
                            .get_or_insert(vec![])
                            .extend(LambdaFunctionConfigurationListDeserializer::deserialize("LambdaFunctionConfigurations", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NotificationConfigurationDeprecated {
    #[serde(rename = "TopicConfiguration")]
    pub topic_configuration: Option<TopicConfigurationDeprecated>,
    #[serde(rename = "QueueConfiguration")]
    pub queue_configuration: Option<QueueConfigurationDeprecated>,
    #[serde(rename = "CloudFunctionConfiguration")]
    pub cloud_function_configuration: Option<CloudFunctionConfiguration>,
}
#[allow(dead_code)]
pub struct NotificationConfigurationDeprecatedDeserializer;
impl NotificationConfigurationDeprecatedDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfigurationDeprecated, XmlParseError> {
        deserialize_elements::<_, NotificationConfigurationDeprecated, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TopicConfiguration" => {
                        obj.topic_configuration = Some(TopicConfigurationDeprecatedDeserializer::deserialize("TopicConfiguration", stack)?);
                    }
                    "QueueConfiguration" => {
                        obj.queue_configuration = Some(QueueConfigurationDeprecatedDeserializer::deserialize("QueueConfiguration", stack)?);
                    }
                    "CloudFunctionConfiguration" => {
                        obj.cloud_function_configuration = Some(CloudFunctionConfigurationDeserializer::deserialize("CloudFunctionConfiguration", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NotificationConfigurationFilter {
    #[serde(rename = "Key")]
    pub key: Option<S3KeyFilter>,
}
#[allow(dead_code)]
pub struct NotificationConfigurationFilterDeserializer;
impl NotificationConfigurationFilterDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfigurationFilter, XmlParseError> {
        deserialize_elements::<_, NotificationConfigurationFilter, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Key" => {
                        obj.key = Some(S3KeyFilterDeserializer::deserialize("Key", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ObjectDeserializer;
impl ObjectDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Object, XmlParseError> {
        deserialize_elements::<_, Object, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Key" => {
                        obj.key = Some(deserialize_primitive("Key", stack, Ok)?);
                    }
                    "LastModified" => {
                        obj.last_modified = Some(deserialize_primitive("LastModified", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "ETag" => {
                        obj.e_tag = Some(deserialize_primitive("ETag", stack, Ok)?);
                    }
                    "Size" => {
                        obj.size = Some(deserialize_primitive("Size", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    "Owner" => {
                        obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ObjectAlreadyInActiveTierError {
}
#[allow(dead_code)]
pub struct ObjectAlreadyInActiveTierErrorDeserializer;
impl ObjectAlreadyInActiveTierErrorDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectAlreadyInActiveTierError, XmlParseError> {
        deserialize_elements::<_, ObjectAlreadyInActiveTierError, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ObjectIdentifier {
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
}
#[allow(dead_code)]
pub struct ObjectIdentifierDeserializer;
impl ObjectIdentifierDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectIdentifier, XmlParseError> {
        deserialize_elements::<_, ObjectIdentifier, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ObjectLockConfiguration {
    #[serde(rename = "ObjectLockEnabled")]
    pub object_lock_enabled: Option<ObjectLockEnabled>,
    #[serde(rename = "Rule")]
    pub rule: Option<ObjectLockRule>,
}
#[allow(dead_code)]
pub struct ObjectLockConfigurationDeserializer;
impl ObjectLockConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectLockConfiguration, XmlParseError> {
        deserialize_elements::<_, ObjectLockConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ObjectLockEnabled" => {
                        obj.object_lock_enabled = Some(deserialize_primitive("ObjectLockEnabled", stack, Ok)?);
                    }
                    "Rule" => {
                        obj.rule = Some(ObjectLockRuleDeserializer::deserialize("Rule", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ObjectLockLegalHold {
    #[serde(rename = "Status")]
    pub status: Option<ObjectLockLegalHoldStatus>,
}
#[allow(dead_code)]
pub struct ObjectLockLegalHoldDeserializer;
impl ObjectLockLegalHoldDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectLockLegalHold, XmlParseError> {
        deserialize_elements::<_, ObjectLockLegalHold, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = Some(deserialize_primitive("Status", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ObjectLockRetention {
    #[serde(rename = "Mode")]
    pub mode: Option<ObjectLockRetentionMode>,
    #[serde(rename = "RetainUntilDate")]
    pub retain_until_date: Option<Date>,
}
#[allow(dead_code)]
pub struct ObjectLockRetentionDeserializer;
impl ObjectLockRetentionDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectLockRetention, XmlParseError> {
        deserialize_elements::<_, ObjectLockRetention, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Mode" => {
                        obj.mode = Some(deserialize_primitive("Mode", stack, Ok)?);
                    }
                    "RetainUntilDate" => {
                        obj.retain_until_date = Some(deserialize_primitive("RetainUntilDate", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ObjectLockRule {
    #[serde(rename = "DefaultRetention")]
    pub default_retention: Option<DefaultRetention>,
}
#[allow(dead_code)]
pub struct ObjectLockRuleDeserializer;
impl ObjectLockRuleDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectLockRule, XmlParseError> {
        deserialize_elements::<_, ObjectLockRule, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DefaultRetention" => {
                        obj.default_retention = Some(DefaultRetentionDeserializer::deserialize("DefaultRetention", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ObjectNotInActiveTierError {
}
#[allow(dead_code)]
pub struct ObjectNotInActiveTierErrorDeserializer;
impl ObjectNotInActiveTierErrorDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectNotInActiveTierError, XmlParseError> {
        deserialize_elements::<_, ObjectNotInActiveTierError, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct ObjectVersionDeserializer;
impl ObjectVersionDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectVersion, XmlParseError> {
        deserialize_elements::<_, ObjectVersion, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ETag" => {
                        obj.e_tag = Some(deserialize_primitive("ETag", stack, Ok)?);
                    }
                    "Size" => {
                        obj.size = Some(deserialize_primitive("Size", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    "Key" => {
                        obj.key = Some(deserialize_primitive("Key", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "IsLatest" => {
                        obj.is_latest = Some(deserialize_primitive("IsLatest", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "LastModified" => {
                        obj.last_modified = Some(deserialize_primitive("LastModified", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "Owner" => {
                        obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OutputLocation {
    #[serde(rename = "S3")]
    pub s3: Option<S3Location>,
}
#[allow(dead_code)]
pub struct OutputLocationDeserializer;
impl OutputLocationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OutputLocation, XmlParseError> {
        deserialize_elements::<_, OutputLocation, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "S3" => {
                        obj.s3 = Some(S3LocationDeserializer::deserialize("S3", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OutputSerialization {
    #[serde(rename = "CSV")]
    pub csv: Option<CSVOutput>,
    #[serde(rename = "JSON")]
    pub json: Option<JSONOutput>,
}
#[allow(dead_code)]
pub struct OutputSerializationDeserializer;
impl OutputSerializationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OutputSerialization, XmlParseError> {
        deserialize_elements::<_, OutputSerialization, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CSV" => {
                        obj.csv = Some(CSVOutputDeserializer::deserialize("CSV", stack)?);
                    }
                    "JSON" => {
                        obj.json = Some(JSONOutputDeserializer::deserialize("JSON", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "DisplayName")]
    pub display_name: Option<DisplayName>,
    #[serde(rename = "ID")]
    pub id: Option<ID>,
}
#[allow(dead_code)]
pub struct OwnerDeserializer;
impl OwnerDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Owner, XmlParseError> {
        deserialize_elements::<_, Owner, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DisplayName" => {
                        obj.display_name = Some(deserialize_primitive("DisplayName", stack, Ok)?);
                    }
                    "ID" => {
                        obj.id = Some(deserialize_primitive("ID", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OwnershipControls {
    #[serde(rename = "Rules")]
    pub rules: OwnershipControlsRules,
}
#[allow(dead_code)]
pub struct OwnershipControlsDeserializer;
impl OwnershipControlsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OwnershipControls, XmlParseError> {
        deserialize_elements::<_, OwnershipControls, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Rules" => {
                        obj.rules
                            .extend(OwnershipControlsRulesDeserializer::deserialize("Rules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OwnershipControlsRule {
    #[serde(rename = "ObjectOwnership")]
    pub object_ownership: ObjectOwnership,
}
#[allow(dead_code)]
pub struct OwnershipControlsRuleDeserializer;
impl OwnershipControlsRuleDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OwnershipControlsRule, XmlParseError> {
        deserialize_elements::<_, OwnershipControlsRule, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ObjectOwnership" => {
                        obj.object_ownership = deserialize_primitive("ObjectOwnership", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParquetInput {
}
#[allow(dead_code)]
pub struct ParquetInputDeserializer;
impl ParquetInputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ParquetInput, XmlParseError> {
        deserialize_elements::<_, ParquetInput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct PartDeserializer;
impl PartDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Part, XmlParseError> {
        deserialize_elements::<_, Part, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "PartNumber" => {
                        obj.part_number = Some(deserialize_primitive("PartNumber", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "LastModified" => {
                        obj.last_modified = Some(deserialize_primitive("LastModified", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "ETag" => {
                        obj.e_tag = Some(deserialize_primitive("ETag", stack, Ok)?);
                    }
                    "Size" => {
                        obj.size = Some(deserialize_primitive("Size", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PolicyStatus {
    #[serde(rename = "IsPublic")]
    pub is_public: Option<IsPublic>,
}
#[allow(dead_code)]
pub struct PolicyStatusDeserializer;
impl PolicyStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyStatus, XmlParseError> {
        deserialize_elements::<_, PolicyStatus, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "IsPublic" => {
                        obj.is_public = Some(deserialize_primitive("IsPublic", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Progress {
    #[serde(rename = "BytesScanned")]
    pub bytes_scanned: Option<BytesScanned>,
    #[serde(rename = "BytesProcessed")]
    pub bytes_processed: Option<BytesProcessed>,
    #[serde(rename = "BytesReturned")]
    pub bytes_returned: Option<BytesReturned>,
}
#[allow(dead_code)]
pub struct ProgressDeserializer;
impl ProgressDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Progress, XmlParseError> {
        deserialize_elements::<_, Progress, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "BytesScanned" => {
                        obj.bytes_scanned = Some(deserialize_primitive("BytesScanned", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    "BytesProcessed" => {
                        obj.bytes_processed = Some(deserialize_primitive("BytesProcessed", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    "BytesReturned" => {
                        obj.bytes_returned = Some(deserialize_primitive("BytesReturned", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProgressEvent {
    #[serde(rename = "Details")]
    pub details: Option<Progress>,
}
#[allow(dead_code)]
pub struct ProgressEventDeserializer;
impl ProgressEventDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ProgressEvent, XmlParseError> {
        deserialize_elements::<_, ProgressEvent, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Details" => {
                        obj.details = Some(ProgressDeserializer::deserialize("Details", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct PublicAccessBlockConfigurationDeserializer;
impl PublicAccessBlockConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PublicAccessBlockConfiguration, XmlParseError> {
        deserialize_elements::<_, PublicAccessBlockConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "BlockPublicAcls" => {
                        obj.block_public_acls = Some(deserialize_primitive("BlockPublicAcls", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "IgnorePublicAcls" => {
                        obj.ignore_public_acls = Some(deserialize_primitive("IgnorePublicAcls", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "BlockPublicPolicy" => {
                        obj.block_public_policy = Some(deserialize_primitive("BlockPublicPolicy", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "RestrictPublicBuckets" => {
                        obj.restrict_public_buckets = Some(deserialize_primitive("RestrictPublicBuckets", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketAccelerateConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "AccelerateConfiguration")]
    pub accelerate_configuration: AccelerateConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketAccelerateConfigurationRequestDeserializer;
impl PutBucketAccelerateConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketAccelerateConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketAccelerateConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "AccelerateConfiguration" => {
                        obj.accelerate_configuration = AccelerateConfigurationDeserializer::deserialize("AccelerateConfiguration", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketAclRequest {
    #[serde(rename = "ACL")]
    pub acl: Option<BucketCannedACL>,
    #[serde(rename = "AccessControlPolicy")]
    pub access_control_policy: Option<AccessControlPolicy>,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: Option<GrantFullControl>,
    #[serde(rename = "GrantRead")]
    pub grant_read: Option<GrantRead>,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: Option<GrantReadACP>,
    #[serde(rename = "GrantWrite")]
    pub grant_write: Option<GrantWrite>,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: Option<GrantWriteACP>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketAclRequestDeserializer;
impl PutBucketAclRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketAclRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketAclRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ACL" => {
                        obj.acl = Some(deserialize_primitive("ACL", stack, Ok)?);
                    }
                    "AccessControlPolicy" => {
                        obj.access_control_policy = Some(AccessControlPolicyDeserializer::deserialize("AccessControlPolicy", stack)?);
                    }
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "GrantFullControl" => {
                        obj.grant_full_control = Some(deserialize_primitive("GrantFullControl", stack, Ok)?);
                    }
                    "GrantRead" => {
                        obj.grant_read = Some(deserialize_primitive("GrantRead", stack, Ok)?);
                    }
                    "GrantReadACP" => {
                        obj.grant_read_acp = Some(deserialize_primitive("GrantReadACP", stack, Ok)?);
                    }
                    "GrantWrite" => {
                        obj.grant_write = Some(deserialize_primitive("GrantWrite", stack, Ok)?);
                    }
                    "GrantWriteACP" => {
                        obj.grant_write_acp = Some(deserialize_primitive("GrantWriteACP", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketAnalyticsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: AnalyticsId,
    #[serde(rename = "AnalyticsConfiguration")]
    pub analytics_configuration: AnalyticsConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketAnalyticsConfigurationRequestDeserializer;
impl PutBucketAnalyticsConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketAnalyticsConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketAnalyticsConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "AnalyticsConfiguration" => {
                        obj.analytics_configuration = AnalyticsConfigurationDeserializer::deserialize("AnalyticsConfiguration", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketCorsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "CORSConfiguration")]
    pub cors_configuration: CORSConfiguration,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketCorsRequestDeserializer;
impl PutBucketCorsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketCorsRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketCorsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "CORSConfiguration" => {
                        obj.cors_configuration = CORSConfigurationDeserializer::deserialize("CORSConfiguration", stack)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketEncryptionRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    pub server_side_encryption_configuration: ServerSideEncryptionConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketEncryptionRequestDeserializer;
impl PutBucketEncryptionRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketEncryptionRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketEncryptionRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "ServerSideEncryptionConfiguration" => {
                        obj.server_side_encryption_configuration = ServerSideEncryptionConfigurationDeserializer::deserialize("ServerSideEncryptionConfiguration", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketIntelligentTieringConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: IntelligentTieringId,
    #[serde(rename = "IntelligentTieringConfiguration")]
    pub intelligent_tiering_configuration: IntelligentTieringConfiguration,
}
#[allow(dead_code)]
pub struct PutBucketIntelligentTieringConfigurationRequestDeserializer;
impl PutBucketIntelligentTieringConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketIntelligentTieringConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketIntelligentTieringConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "IntelligentTieringConfiguration" => {
                        obj.intelligent_tiering_configuration = IntelligentTieringConfigurationDeserializer::deserialize("IntelligentTieringConfiguration", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketInventoryConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: InventoryId,
    #[serde(rename = "InventoryConfiguration")]
    pub inventory_configuration: InventoryConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketInventoryConfigurationRequestDeserializer;
impl PutBucketInventoryConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketInventoryConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketInventoryConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "InventoryConfiguration" => {
                        obj.inventory_configuration = InventoryConfigurationDeserializer::deserialize("InventoryConfiguration", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketLifecycleConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "LifecycleConfiguration")]
    pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketLifecycleConfigurationRequestDeserializer;
impl PutBucketLifecycleConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketLifecycleConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketLifecycleConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "LifecycleConfiguration" => {
                        obj.lifecycle_configuration = Some(BucketLifecycleConfigurationDeserializer::deserialize("LifecycleConfiguration", stack)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketLifecycleRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "LifecycleConfiguration")]
    pub lifecycle_configuration: Option<LifecycleConfiguration>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketLifecycleRequestDeserializer;
impl PutBucketLifecycleRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketLifecycleRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketLifecycleRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "LifecycleConfiguration" => {
                        obj.lifecycle_configuration = Some(LifecycleConfigurationDeserializer::deserialize("LifecycleConfiguration", stack)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketLoggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "BucketLoggingStatus")]
    pub bucket_logging_status: BucketLoggingStatus,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketLoggingRequestDeserializer;
impl PutBucketLoggingRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketLoggingRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketLoggingRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "BucketLoggingStatus" => {
                        obj.bucket_logging_status = BucketLoggingStatusDeserializer::deserialize("BucketLoggingStatus", stack)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketMetricsConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Id")]
    pub id: MetricsId,
    #[serde(rename = "MetricsConfiguration")]
    pub metrics_configuration: MetricsConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketMetricsConfigurationRequestDeserializer;
impl PutBucketMetricsConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketMetricsConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketMetricsConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Id" => {
                        obj.id = deserialize_primitive("Id", stack, Ok)?;
                    }
                    "MetricsConfiguration" => {
                        obj.metrics_configuration = MetricsConfigurationDeserializer::deserialize("MetricsConfiguration", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketNotificationConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "NotificationConfiguration")]
    pub notification_configuration: NotificationConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketNotificationConfigurationRequestDeserializer;
impl PutBucketNotificationConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketNotificationConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketNotificationConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "NotificationConfiguration" => {
                        obj.notification_configuration = NotificationConfigurationDeserializer::deserialize("NotificationConfiguration", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketNotificationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "NotificationConfiguration")]
    pub notification_configuration: NotificationConfigurationDeprecated,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketNotificationRequestDeserializer;
impl PutBucketNotificationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketNotificationRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketNotificationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "NotificationConfiguration" => {
                        obj.notification_configuration = NotificationConfigurationDeprecatedDeserializer::deserialize("NotificationConfiguration", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketOwnershipControlsRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
    #[serde(rename = "OwnershipControls")]
    pub ownership_controls: OwnershipControls,
}
#[allow(dead_code)]
pub struct PutBucketOwnershipControlsRequestDeserializer;
impl PutBucketOwnershipControlsRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketOwnershipControlsRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketOwnershipControlsRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    "OwnershipControls" => {
                        obj.ownership_controls = OwnershipControlsDeserializer::deserialize("OwnershipControls", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketPolicyRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "ConfirmRemoveSelfBucketAccess")]
    pub confirm_remove_self_bucket_access: Option<ConfirmRemoveSelfBucketAccess>,
    #[serde(rename = "Policy")]
    pub policy: Policy,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketPolicyRequestDeserializer;
impl PutBucketPolicyRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketPolicyRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketPolicyRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "ConfirmRemoveSelfBucketAccess" => {
                        obj.confirm_remove_self_bucket_access = Some(deserialize_primitive("ConfirmRemoveSelfBucketAccess", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "Policy" => {
                        obj.policy = deserialize_primitive("Policy", stack, Ok)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketReplicationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "ReplicationConfiguration")]
    pub replication_configuration: ReplicationConfiguration,
    #[serde(rename = "Token")]
    pub token: Option<ObjectLockToken>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketReplicationRequestDeserializer;
impl PutBucketReplicationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketReplicationRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketReplicationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "ReplicationConfiguration" => {
                        obj.replication_configuration = ReplicationConfigurationDeserializer::deserialize("ReplicationConfiguration", stack)?;
                    }
                    "Token" => {
                        obj.token = Some(deserialize_primitive("Token", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketRequestPaymentRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "RequestPaymentConfiguration")]
    pub request_payment_configuration: RequestPaymentConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketRequestPaymentRequestDeserializer;
impl PutBucketRequestPaymentRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketRequestPaymentRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketRequestPaymentRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "RequestPaymentConfiguration" => {
                        obj.request_payment_configuration = RequestPaymentConfigurationDeserializer::deserialize("RequestPaymentConfiguration", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "Tagging")]
    pub tagging: Tagging,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketTaggingRequestDeserializer;
impl PutBucketTaggingRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketTaggingRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketTaggingRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "Tagging" => {
                        obj.tagging = TaggingDeserializer::deserialize("Tagging", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketVersioningRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "MFA")]
    pub mfa: Option<MFA>,
    #[serde(rename = "VersioningConfiguration")]
    pub versioning_configuration: VersioningConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketVersioningRequestDeserializer;
impl PutBucketVersioningRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketVersioningRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketVersioningRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "MFA" => {
                        obj.mfa = Some(deserialize_primitive("MFA", stack, Ok)?);
                    }
                    "VersioningConfiguration" => {
                        obj.versioning_configuration = VersioningConfigurationDeserializer::deserialize("VersioningConfiguration", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutBucketWebsiteRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "WebsiteConfiguration")]
    pub website_configuration: WebsiteConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutBucketWebsiteRequestDeserializer;
impl PutBucketWebsiteRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketWebsiteRequest, XmlParseError> {
        deserialize_elements::<_, PutBucketWebsiteRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "WebsiteConfiguration" => {
                        obj.website_configuration = WebsiteConfigurationDeserializer::deserialize("WebsiteConfiguration", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutObjectAclOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}
#[allow(dead_code)]
pub struct PutObjectAclOutputDeserializer;
impl PutObjectAclOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectAclOutput, XmlParseError> {
        deserialize_elements::<_, PutObjectAclOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutObjectAclRequest {
    #[serde(rename = "ACL")]
    pub acl: Option<ObjectCannedACL>,
    #[serde(rename = "AccessControlPolicy")]
    pub access_control_policy: Option<AccessControlPolicy>,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: Option<GrantFullControl>,
    #[serde(rename = "GrantRead")]
    pub grant_read: Option<GrantRead>,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: Option<GrantReadACP>,
    #[serde(rename = "GrantWrite")]
    pub grant_write: Option<GrantWrite>,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: Option<GrantWriteACP>,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutObjectAclRequestDeserializer;
impl PutObjectAclRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectAclRequest, XmlParseError> {
        deserialize_elements::<_, PutObjectAclRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ACL" => {
                        obj.acl = Some(deserialize_primitive("ACL", stack, Ok)?);
                    }
                    "AccessControlPolicy" => {
                        obj.access_control_policy = Some(AccessControlPolicyDeserializer::deserialize("AccessControlPolicy", stack)?);
                    }
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "GrantFullControl" => {
                        obj.grant_full_control = Some(deserialize_primitive("GrantFullControl", stack, Ok)?);
                    }
                    "GrantRead" => {
                        obj.grant_read = Some(deserialize_primitive("GrantRead", stack, Ok)?);
                    }
                    "GrantReadACP" => {
                        obj.grant_read_acp = Some(deserialize_primitive("GrantReadACP", stack, Ok)?);
                    }
                    "GrantWrite" => {
                        obj.grant_write = Some(deserialize_primitive("GrantWrite", stack, Ok)?);
                    }
                    "GrantWriteACP" => {
                        obj.grant_write_acp = Some(deserialize_primitive("GrantWriteACP", stack, Ok)?);
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutObjectLegalHoldOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}
#[allow(dead_code)]
pub struct PutObjectLegalHoldOutputDeserializer;
impl PutObjectLegalHoldOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectLegalHoldOutput, XmlParseError> {
        deserialize_elements::<_, PutObjectLegalHoldOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutObjectLegalHoldRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "LegalHold")]
    pub legal_hold: Option<ObjectLockLegalHold>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutObjectLegalHoldRequestDeserializer;
impl PutObjectLegalHoldRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectLegalHoldRequest, XmlParseError> {
        deserialize_elements::<_, PutObjectLegalHoldRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "LegalHold" => {
                        obj.legal_hold = Some(ObjectLockLegalHoldDeserializer::deserialize("LegalHold", stack)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutObjectLockConfigurationOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}
#[allow(dead_code)]
pub struct PutObjectLockConfigurationOutputDeserializer;
impl PutObjectLockConfigurationOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectLockConfigurationOutput, XmlParseError> {
        deserialize_elements::<_, PutObjectLockConfigurationOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutObjectLockConfigurationRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ObjectLockConfiguration")]
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "Token")]
    pub token: Option<ObjectLockToken>,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutObjectLockConfigurationRequestDeserializer;
impl PutObjectLockConfigurationRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectLockConfigurationRequest, XmlParseError> {
        deserialize_elements::<_, PutObjectLockConfigurationRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ObjectLockConfiguration" => {
                        obj.object_lock_configuration = Some(ObjectLockConfigurationDeserializer::deserialize("ObjectLockConfiguration", stack)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "Token" => {
                        obj.token = Some(deserialize_primitive("Token", stack, Ok)?);
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct PutObjectOutputDeserializer;
impl PutObjectOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectOutput, XmlParseError> {
        deserialize_elements::<_, PutObjectOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Expiration" => {
                        obj.expiration = Some(deserialize_primitive("Expiration", stack, Ok)?);
                    }
                    "ETag" => {
                        obj.e_tag = Some(deserialize_primitive("ETag", stack, Ok)?);
                    }
                    "ServerSideEncryption" => {
                        obj.server_side_encryption = Some(deserialize_primitive("ServerSideEncryption", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "SSEKMSKeyId" => {
                        obj.ssekms_key_id = Some(deserialize_primitive("SSEKMSKeyId", stack, Ok)?);
                    }
                    "SSEKMSEncryptionContext" => {
                        obj.ssekms_encryption_context = Some(deserialize_primitive("SSEKMSEncryptionContext", stack, Ok)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutObjectRequest {
    #[serde(rename = "ACL")]
    pub acl: Option<ObjectCannedACL>,
    #[serde(rename = "Body")]
    pub body: Option<Body>,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "CacheControl")]
    pub cache_control: Option<CacheControl>,
    #[serde(rename = "ContentDisposition")]
    pub content_disposition: Option<ContentDisposition>,
    #[serde(rename = "ContentEncoding")]
    pub content_encoding: Option<ContentEncoding>,
    #[serde(rename = "ContentLanguage")]
    pub content_language: Option<ContentLanguage>,
    #[serde(rename = "ContentLength")]
    pub content_length: Option<ContentLength>,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<ContentType>,
    #[serde(rename = "Expires")]
    pub expires: Option<Expires>,
    #[serde(rename = "GrantFullControl")]
    pub grant_full_control: Option<GrantFullControl>,
    #[serde(rename = "GrantRead")]
    pub grant_read: Option<GrantRead>,
    #[serde(rename = "GrantReadACP")]
    pub grant_read_acp: Option<GrantReadACP>,
    #[serde(rename = "GrantWriteACP")]
    pub grant_write_acp: Option<GrantWriteACP>,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "ServerSideEncryption")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
    #[serde(rename = "WebsiteRedirectLocation")]
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: Option<SSECustomerKey>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "SSEKMSKeyId")]
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    #[serde(rename = "SSEKMSEncryptionContext")]
    pub ssekms_encryption_context: Option<SSEKMSEncryptionContext>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "Tagging")]
    pub tagging: Option<TaggingHeader>,
    #[serde(rename = "ObjectLockMode")]
    pub object_lock_mode: Option<ObjectLockMode>,
    #[serde(rename = "ObjectLockRetainUntilDate")]
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    #[serde(rename = "ObjectLockLegalHoldStatus")]
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutObjectRequestDeserializer;
impl PutObjectRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectRequest, XmlParseError> {
        deserialize_elements::<_, PutObjectRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ACL" => {
                        obj.acl = Some(deserialize_primitive("ACL", stack, Ok)?);
                    }
                    "Body" => {
                        obj.body = Some(Default::default());
                    }
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "CacheControl" => {
                        obj.cache_control = Some(deserialize_primitive("CacheControl", stack, Ok)?);
                    }
                    "ContentDisposition" => {
                        obj.content_disposition = Some(deserialize_primitive("ContentDisposition", stack, Ok)?);
                    }
                    "ContentEncoding" => {
                        obj.content_encoding = Some(deserialize_primitive("ContentEncoding", stack, Ok)?);
                    }
                    "ContentLanguage" => {
                        obj.content_language = Some(deserialize_primitive("ContentLanguage", stack, Ok)?);
                    }
                    "ContentLength" => {
                        obj.content_length = Some(deserialize_primitive("ContentLength", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "ContentType" => {
                        obj.content_type = Some(deserialize_primitive("ContentType", stack, Ok)?);
                    }
                    "Expires" => {
                        obj.expires = Some(deserialize_primitive("Expires", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "GrantFullControl" => {
                        obj.grant_full_control = Some(deserialize_primitive("GrantFullControl", stack, Ok)?);
                    }
                    "GrantRead" => {
                        obj.grant_read = Some(deserialize_primitive("GrantRead", stack, Ok)?);
                    }
                    "GrantReadACP" => {
                        obj.grant_read_acp = Some(deserialize_primitive("GrantReadACP", stack, Ok)?);
                    }
                    "GrantWriteACP" => {
                        obj.grant_write_acp = Some(deserialize_primitive("GrantWriteACP", stack, Ok)?);
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "Metadata" => {
                        obj.metadata = Some(Default::default());
                    }
                    "ServerSideEncryption" => {
                        obj.server_side_encryption = Some(deserialize_primitive("ServerSideEncryption", stack, Ok)?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    "WebsiteRedirectLocation" => {
                        obj.website_redirect_location = Some(deserialize_primitive("WebsiteRedirectLocation", stack, Ok)?);
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKey" => {
                        obj.sse_customer_key = Some(deserialize_primitive("SSECustomerKey", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "SSEKMSKeyId" => {
                        obj.ssekms_key_id = Some(deserialize_primitive("SSEKMSKeyId", stack, Ok)?);
                    }
                    "SSEKMSEncryptionContext" => {
                        obj.ssekms_encryption_context = Some(deserialize_primitive("SSEKMSEncryptionContext", stack, Ok)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "Tagging" => {
                        obj.tagging = Some(deserialize_primitive("Tagging", stack, Ok)?);
                    }
                    "ObjectLockMode" => {
                        obj.object_lock_mode = Some(deserialize_primitive("ObjectLockMode", stack, Ok)?);
                    }
                    "ObjectLockRetainUntilDate" => {
                        obj.object_lock_retain_until_date = Some(deserialize_primitive("ObjectLockRetainUntilDate", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "ObjectLockLegalHoldStatus" => {
                        obj.object_lock_legal_hold_status = Some(deserialize_primitive("ObjectLockLegalHoldStatus", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutObjectRetentionOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
}
#[allow(dead_code)]
pub struct PutObjectRetentionOutputDeserializer;
impl PutObjectRetentionOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectRetentionOutput, XmlParseError> {
        deserialize_elements::<_, PutObjectRetentionOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutObjectRetentionRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Retention")]
    pub retention: Option<ObjectLockRetention>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "BypassGovernanceRetention")]
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutObjectRetentionRequestDeserializer;
impl PutObjectRetentionRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectRetentionRequest, XmlParseError> {
        deserialize_elements::<_, PutObjectRetentionRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "Retention" => {
                        obj.retention = Some(ObjectLockRetentionDeserializer::deserialize("Retention", stack)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "BypassGovernanceRetention" => {
                        obj.bypass_governance_retention = Some(deserialize_primitive("BypassGovernanceRetention", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutObjectTaggingOutput {
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
}
#[allow(dead_code)]
pub struct PutObjectTaggingOutputDeserializer;
impl PutObjectTaggingOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectTaggingOutput, XmlParseError> {
        deserialize_elements::<_, PutObjectTaggingOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutObjectTaggingRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "Tagging")]
    pub tagging: Tagging,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutObjectTaggingRequestDeserializer;
impl PutObjectTaggingRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectTaggingRequest, XmlParseError> {
        deserialize_elements::<_, PutObjectTaggingRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "Tagging" => {
                        obj.tagging = TaggingDeserializer::deserialize("Tagging", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PutPublicAccessBlockRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct PutPublicAccessBlockRequestDeserializer;
impl PutPublicAccessBlockRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutPublicAccessBlockRequest, XmlParseError> {
        deserialize_elements::<_, PutPublicAccessBlockRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "PublicAccessBlockConfiguration" => {
                        obj.public_access_block_configuration = PublicAccessBlockConfigurationDeserializer::deserialize("PublicAccessBlockConfiguration", stack)?;
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueueConfiguration {
    #[serde(rename = "Id")]
    pub id: Option<NotificationId>,
    #[serde(rename = "QueueArn")]
    pub queue_arn: QueueArn,
    #[serde(rename = "Events")]
    pub events: EventList,
    #[serde(rename = "Filter")]
    pub filter: Option<NotificationConfigurationFilter>,
}
#[allow(dead_code)]
pub struct QueueConfigurationDeserializer;
impl QueueConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueueConfiguration, XmlParseError> {
        deserialize_elements::<_, QueueConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Id" => {
                        obj.id = Some(deserialize_primitive("Id", stack, Ok)?);
                    }
                    "QueueArn" => {
                        obj.queue_arn = deserialize_primitive("QueueArn", stack, Ok)?;
                    }
                    "Events" => {
                        obj.events
                            .extend(EventListDeserializer::deserialize("Events", stack)?);
                    }
                    "Filter" => {
                        obj.filter = Some(NotificationConfigurationFilterDeserializer::deserialize("Filter", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct QueueConfigurationDeprecatedDeserializer;
impl QueueConfigurationDeprecatedDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueueConfigurationDeprecated, XmlParseError> {
        deserialize_elements::<_, QueueConfigurationDeprecated, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Id" => {
                        obj.id = Some(deserialize_primitive("Id", stack, Ok)?);
                    }
                    "Event" => {
                        obj.event = Some(deserialize_primitive("Event", stack, Ok)?);
                    }
                    "Events" => {
                        obj.events
                            .get_or_insert(vec![])
                            .extend(EventListDeserializer::deserialize("Events", stack)?);
                    }
                    "Queue" => {
                        obj.queue = Some(deserialize_primitive("Queue", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RecordsEvent {
    #[serde(rename = "Payload")]
    pub payload: Option<Body>,
}
#[allow(dead_code)]
pub struct RecordsEventDeserializer;
impl RecordsEventDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RecordsEvent, XmlParseError> {
        deserialize_elements::<_, RecordsEvent, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Payload" => {
                        obj.payload = Some(Default::default());
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct RedirectDeserializer;
impl RedirectDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Redirect, XmlParseError> {
        deserialize_elements::<_, Redirect, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HostName" => {
                        obj.host_name = Some(deserialize_primitive("HostName", stack, Ok)?);
                    }
                    "HttpRedirectCode" => {
                        obj.http_redirect_code = Some(deserialize_primitive("HttpRedirectCode", stack, Ok)?);
                    }
                    "Protocol" => {
                        obj.protocol = Some(deserialize_primitive("Protocol", stack, Ok)?);
                    }
                    "ReplaceKeyPrefixWith" => {
                        obj.replace_key_prefix_with = Some(deserialize_primitive("ReplaceKeyPrefixWith", stack, Ok)?);
                    }
                    "ReplaceKeyWith" => {
                        obj.replace_key_with = Some(deserialize_primitive("ReplaceKeyWith", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RedirectAllRequestsTo {
    #[serde(rename = "HostName")]
    pub host_name: HostName,
    #[serde(rename = "Protocol")]
    pub protocol: Option<Protocol>,
}
#[allow(dead_code)]
pub struct RedirectAllRequestsToDeserializer;
impl RedirectAllRequestsToDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RedirectAllRequestsTo, XmlParseError> {
        deserialize_elements::<_, RedirectAllRequestsTo, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "HostName" => {
                        obj.host_name = deserialize_primitive("HostName", stack, Ok)?;
                    }
                    "Protocol" => {
                        obj.protocol = Some(deserialize_primitive("Protocol", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicaModifications {
    #[serde(rename = "Status")]
    pub status: ReplicaModificationsStatus,
}
#[allow(dead_code)]
pub struct ReplicaModificationsDeserializer;
impl ReplicaModificationsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicaModifications, XmlParseError> {
        deserialize_elements::<_, ReplicaModifications, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = deserialize_primitive("Status", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicationConfiguration {
    #[serde(rename = "Role")]
    pub role: Role,
    #[serde(rename = "Rules")]
    pub rules: ReplicationRules,
}
#[allow(dead_code)]
pub struct ReplicationConfigurationDeserializer;
impl ReplicationConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationConfiguration, XmlParseError> {
        deserialize_elements::<_, ReplicationConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Role" => {
                        obj.role = deserialize_primitive("Role", stack, Ok)?;
                    }
                    "Rules" => {
                        obj.rules
                            .extend(ReplicationRulesDeserializer::deserialize("Rules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicationRule {
    #[serde(rename = "ID")]
    pub id: Option<ID>,
    #[serde(rename = "Priority")]
    pub priority: Option<Priority>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Filter")]
    pub filter: Option<ReplicationRuleFilter>,
    #[serde(rename = "Status")]
    pub status: ReplicationRuleStatus,
    #[serde(rename = "SourceSelectionCriteria")]
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    #[serde(rename = "ExistingObjectReplication")]
    pub existing_object_replication: Option<ExistingObjectReplication>,
    #[serde(rename = "Destination")]
    pub destination: Destination,
    #[serde(rename = "DeleteMarkerReplication")]
    pub delete_marker_replication: Option<DeleteMarkerReplication>,
}
#[allow(dead_code)]
pub struct ReplicationRuleDeserializer;
impl ReplicationRuleDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationRule, XmlParseError> {
        deserialize_elements::<_, ReplicationRule, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ID" => {
                        obj.id = Some(deserialize_primitive("ID", stack, Ok)?);
                    }
                    "Priority" => {
                        obj.priority = Some(deserialize_primitive("Priority", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Filter" => {
                        obj.filter = Some(ReplicationRuleFilterDeserializer::deserialize("Filter", stack)?);
                    }
                    "Status" => {
                        obj.status = deserialize_primitive("Status", stack, Ok)?;
                    }
                    "SourceSelectionCriteria" => {
                        obj.source_selection_criteria = Some(SourceSelectionCriteriaDeserializer::deserialize("SourceSelectionCriteria", stack)?);
                    }
                    "ExistingObjectReplication" => {
                        obj.existing_object_replication = Some(ExistingObjectReplicationDeserializer::deserialize("ExistingObjectReplication", stack)?);
                    }
                    "Destination" => {
                        obj.destination = DestinationDeserializer::deserialize("Destination", stack)?;
                    }
                    "DeleteMarkerReplication" => {
                        obj.delete_marker_replication = Some(DeleteMarkerReplicationDeserializer::deserialize("DeleteMarkerReplication", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicationRuleAndOperator {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tags")]
    pub tags: Option<TagSet>,
}
#[allow(dead_code)]
pub struct ReplicationRuleAndOperatorDeserializer;
impl ReplicationRuleAndOperatorDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationRuleAndOperator, XmlParseError> {
        deserialize_elements::<_, ReplicationRuleAndOperator, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Tags" => {
                        obj.tags
                            .get_or_insert(vec![])
                            .extend(TagSetDeserializer::deserialize("Tags", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicationRuleFilter {
    #[serde(rename = "Prefix")]
    pub prefix: Option<Prefix>,
    #[serde(rename = "Tag")]
    pub tag: Option<Tag>,
    #[serde(rename = "And")]
    pub and: Option<ReplicationRuleAndOperator>,
}
#[allow(dead_code)]
pub struct ReplicationRuleFilterDeserializer;
impl ReplicationRuleFilterDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationRuleFilter, XmlParseError> {
        deserialize_elements::<_, ReplicationRuleFilter, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(deserialize_primitive("Prefix", stack, Ok)?);
                    }
                    "Tag" => {
                        obj.tag = Some(TagDeserializer::deserialize("Tag", stack)?);
                    }
                    "And" => {
                        obj.and = Some(ReplicationRuleAndOperatorDeserializer::deserialize("And", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicationTime {
    #[serde(rename = "Status")]
    pub status: ReplicationTimeStatus,
    #[serde(rename = "Time")]
    pub time: ReplicationTimeValue,
}
#[allow(dead_code)]
pub struct ReplicationTimeDeserializer;
impl ReplicationTimeDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationTime, XmlParseError> {
        deserialize_elements::<_, ReplicationTime, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = deserialize_primitive("Status", stack, Ok)?;
                    }
                    "Time" => {
                        obj.time = ReplicationTimeValueDeserializer::deserialize("Time", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReplicationTimeValue {
    #[serde(rename = "Minutes")]
    pub minutes: Option<Minutes>,
}
#[allow(dead_code)]
pub struct ReplicationTimeValueDeserializer;
impl ReplicationTimeValueDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationTimeValue, XmlParseError> {
        deserialize_elements::<_, ReplicationTimeValue, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Minutes" => {
                        obj.minutes = Some(deserialize_primitive("Minutes", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestPaymentConfiguration {
    #[serde(rename = "Payer")]
    pub payer: Payer,
}
#[allow(dead_code)]
pub struct RequestPaymentConfigurationDeserializer;
impl RequestPaymentConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RequestPaymentConfiguration, XmlParseError> {
        deserialize_elements::<_, RequestPaymentConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Payer" => {
                        obj.payer = deserialize_primitive("Payer", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestProgress {
    #[serde(rename = "Enabled")]
    pub enabled: Option<EnableRequestProgress>,
}
#[allow(dead_code)]
pub struct RequestProgressDeserializer;
impl RequestProgressDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RequestProgress, XmlParseError> {
        deserialize_elements::<_, RequestProgress, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Enabled" => {
                        obj.enabled = Some(deserialize_primitive("Enabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreObjectOutput {
    #[serde(rename = "RequestCharged")]
    pub request_charged: Option<RequestCharged>,
    #[serde(rename = "RestoreOutputPath")]
    pub restore_output_path: Option<RestoreOutputPath>,
}
#[allow(dead_code)]
pub struct RestoreObjectOutputDeserializer;
impl RestoreObjectOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreObjectOutput, XmlParseError> {
        deserialize_elements::<_, RestoreObjectOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    "RestoreOutputPath" => {
                        obj.restore_output_path = Some(deserialize_primitive("RestoreOutputPath", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreObjectRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "VersionId")]
    pub version_id: Option<ObjectVersionId>,
    #[serde(rename = "RestoreRequest")]
    pub restore_request: Option<RestoreRequest>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct RestoreObjectRequestDeserializer;
impl RestoreObjectRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreObjectRequest, XmlParseError> {
        deserialize_elements::<_, RestoreObjectRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "VersionId" => {
                        obj.version_id = Some(deserialize_primitive("VersionId", stack, Ok)?);
                    }
                    "RestoreRequest" => {
                        obj.restore_request = Some(RestoreRequestDeserializer::deserialize("RestoreRequest", stack)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RestoreRequest {
    #[serde(rename = "Days")]
    pub days: Option<Days>,
    #[serde(rename = "GlacierJobParameters")]
    pub glacier_job_parameters: Option<GlacierJobParameters>,
    #[serde(rename = "Type")]
    pub r#type: Option<RestoreRequestType>,
    #[serde(rename = "Tier")]
    pub tier: Option<Tier>,
    #[serde(rename = "Description")]
    pub description: Option<Description>,
    #[serde(rename = "SelectParameters")]
    pub select_parameters: Option<SelectParameters>,
    #[serde(rename = "OutputLocation")]
    pub output_location: Option<OutputLocation>,
}
#[allow(dead_code)]
pub struct RestoreRequestDeserializer;
impl RestoreRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreRequest, XmlParseError> {
        deserialize_elements::<_, RestoreRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Days" => {
                        obj.days = Some(deserialize_primitive("Days", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "GlacierJobParameters" => {
                        obj.glacier_job_parameters = Some(GlacierJobParametersDeserializer::deserialize("GlacierJobParameters", stack)?);
                    }
                    "Type" => {
                        obj.r#type = Some(deserialize_primitive("Type", stack, Ok)?);
                    }
                    "Tier" => {
                        obj.tier = Some(deserialize_primitive("Tier", stack, Ok)?);
                    }
                    "Description" => {
                        obj.description = Some(deserialize_primitive("Description", stack, Ok)?);
                    }
                    "SelectParameters" => {
                        obj.select_parameters = Some(SelectParametersDeserializer::deserialize("SelectParameters", stack)?);
                    }
                    "OutputLocation" => {
                        obj.output_location = Some(OutputLocationDeserializer::deserialize("OutputLocation", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RoutingRule {
    #[serde(rename = "Condition")]
    pub condition: Option<Condition>,
    #[serde(rename = "Redirect")]
    pub redirect: Redirect,
}
#[allow(dead_code)]
pub struct RoutingRuleDeserializer;
impl RoutingRuleDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RoutingRule, XmlParseError> {
        deserialize_elements::<_, RoutingRule, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Condition" => {
                        obj.condition = Some(ConditionDeserializer::deserialize("Condition", stack)?);
                    }
                    "Redirect" => {
                        obj.redirect = RedirectDeserializer::deserialize("Redirect", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Rule {
    #[serde(rename = "Expiration")]
    pub expiration: Option<LifecycleExpiration>,
    #[serde(rename = "ID")]
    pub id: Option<ID>,
    #[serde(rename = "Prefix")]
    pub prefix: Prefix,
    #[serde(rename = "Status")]
    pub status: ExpirationStatus,
    #[serde(rename = "Transition")]
    pub transition: Option<Transition>,
    #[serde(rename = "NoncurrentVersionTransition")]
    pub noncurrent_version_transition: Option<NoncurrentVersionTransition>,
    #[serde(rename = "NoncurrentVersionExpiration")]
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    #[serde(rename = "AbortIncompleteMultipartUpload")]
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
}
#[allow(dead_code)]
pub struct RuleDeserializer;
impl RuleDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Rule, XmlParseError> {
        deserialize_elements::<_, Rule, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Expiration" => {
                        obj.expiration = Some(LifecycleExpirationDeserializer::deserialize("Expiration", stack)?);
                    }
                    "ID" => {
                        obj.id = Some(deserialize_primitive("ID", stack, Ok)?);
                    }
                    "Prefix" => {
                        obj.prefix = deserialize_primitive("Prefix", stack, Ok)?;
                    }
                    "Status" => {
                        obj.status = deserialize_primitive("Status", stack, Ok)?;
                    }
                    "Transition" => {
                        obj.transition = Some(TransitionDeserializer::deserialize("Transition", stack)?);
                    }
                    "NoncurrentVersionTransition" => {
                        obj.noncurrent_version_transition = Some(NoncurrentVersionTransitionDeserializer::deserialize("NoncurrentVersionTransition", stack)?);
                    }
                    "NoncurrentVersionExpiration" => {
                        obj.noncurrent_version_expiration = Some(NoncurrentVersionExpirationDeserializer::deserialize("NoncurrentVersionExpiration", stack)?);
                    }
                    "AbortIncompleteMultipartUpload" => {
                        obj.abort_incomplete_multipart_upload = Some(AbortIncompleteMultipartUploadDeserializer::deserialize("AbortIncompleteMultipartUpload", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct S3KeyFilter {
    #[serde(rename = "FilterRules")]
    pub filter_rules: Option<FilterRuleList>,
}
#[allow(dead_code)]
pub struct S3KeyFilterDeserializer;
impl S3KeyFilterDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3KeyFilter, XmlParseError> {
        deserialize_elements::<_, S3KeyFilter, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "FilterRules" => {
                        obj.filter_rules
                            .get_or_insert(vec![])
                            .extend(FilterRuleListDeserializer::deserialize("FilterRules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct S3Location {
    #[serde(rename = "BucketName")]
    pub bucket_name: BucketName,
    #[serde(rename = "Prefix")]
    pub prefix: LocationPrefix,
    #[serde(rename = "Encryption")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "CannedACL")]
    pub canned_acl: Option<ObjectCannedACL>,
    #[serde(rename = "AccessControlList")]
    pub access_control_list: Option<Grants>,
    #[serde(rename = "Tagging")]
    pub tagging: Option<Tagging>,
    #[serde(rename = "UserMetadata")]
    pub user_metadata: Option<UserMetadata>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<StorageClass>,
}
#[allow(dead_code)]
pub struct S3LocationDeserializer;
impl S3LocationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3Location, XmlParseError> {
        deserialize_elements::<_, S3Location, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "BucketName" => {
                        obj.bucket_name = deserialize_primitive("BucketName", stack, Ok)?;
                    }
                    "Prefix" => {
                        obj.prefix = deserialize_primitive("Prefix", stack, Ok)?;
                    }
                    "Encryption" => {
                        obj.encryption = Some(EncryptionDeserializer::deserialize("Encryption", stack)?);
                    }
                    "CannedACL" => {
                        obj.canned_acl = Some(deserialize_primitive("CannedACL", stack, Ok)?);
                    }
                    "AccessControlList" => {
                        obj.access_control_list
                            .get_or_insert(vec![])
                            .extend(GrantsDeserializer::deserialize("AccessControlList", stack)?);
                    }
                    "Tagging" => {
                        obj.tagging = Some(TaggingDeserializer::deserialize("Tagging", stack)?);
                    }
                    "UserMetadata" => {
                        obj.user_metadata
                            .get_or_insert(vec![])
                            .extend(UserMetadataDeserializer::deserialize("UserMetadata", stack)?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SSEKMS {
    #[serde(rename = "KeyId")]
    pub key_id: SSEKMSKeyId,
}
#[allow(dead_code)]
pub struct SSEKMSDeserializer;
impl SSEKMSDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SSEKMS, XmlParseError> {
        deserialize_elements::<_, SSEKMS, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "KeyId" => {
                        obj.key_id = deserialize_primitive("KeyId", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SSES3 {
}
#[allow(dead_code)]
pub struct SSES3Deserializer;
impl SSES3Deserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SSES3, XmlParseError> {
        deserialize_elements::<_, SSES3, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScanRange {
    #[serde(rename = "Start")]
    pub start: Option<Start>,
    #[serde(rename = "End")]
    pub end: Option<End>,
}
#[allow(dead_code)]
pub struct ScanRangeDeserializer;
impl ScanRangeDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ScanRange, XmlParseError> {
        deserialize_elements::<_, ScanRange, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Start" => {
                        obj.start = Some(deserialize_primitive("Start", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    "End" => {
                        obj.end = Some(deserialize_primitive("End", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct SelectObjectContentEventStreamDeserializer;
impl SelectObjectContentEventStreamDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SelectObjectContentEventStream, XmlParseError> {
        deserialize_elements::<_, SelectObjectContentEventStream, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Records" => {
                        obj.records = Some(RecordsEventDeserializer::deserialize("Records", stack)?);
                    }
                    "Stats" => {
                        obj.stats = Some(StatsEventDeserializer::deserialize("Stats", stack)?);
                    }
                    "Progress" => {
                        obj.progress = Some(ProgressEventDeserializer::deserialize("Progress", stack)?);
                    }
                    "Cont" => {
                        obj.cont = Some(ContinuationEventDeserializer::deserialize("Cont", stack)?);
                    }
                    "End" => {
                        obj.end = Some(EndEventDeserializer::deserialize("End", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SelectObjectContentOutput {
    #[serde(rename = "Payload")]
    pub payload: Option<SelectObjectContentEventStream>,
}
#[allow(dead_code)]
pub struct SelectObjectContentOutputDeserializer;
impl SelectObjectContentOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SelectObjectContentOutput, XmlParseError> {
        deserialize_elements::<_, SelectObjectContentOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Payload" => {
                        obj.payload = Some(SelectObjectContentEventStreamDeserializer::deserialize("Payload", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SelectObjectContentRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: Option<SSECustomerKey>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "Expression")]
    pub expression: Expression,
    #[serde(rename = "ExpressionType")]
    pub expression_type: ExpressionType,
    #[serde(rename = "RequestProgress")]
    pub request_progress: Option<RequestProgress>,
    #[serde(rename = "InputSerialization")]
    pub input_serialization: InputSerialization,
    #[serde(rename = "OutputSerialization")]
    pub output_serialization: OutputSerialization,
    #[serde(rename = "ScanRange")]
    pub scan_range: Option<ScanRange>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct SelectObjectContentRequestDeserializer;
impl SelectObjectContentRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SelectObjectContentRequest, XmlParseError> {
        deserialize_elements::<_, SelectObjectContentRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKey" => {
                        obj.sse_customer_key = Some(deserialize_primitive("SSECustomerKey", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "Expression" => {
                        obj.expression = deserialize_primitive("Expression", stack, Ok)?;
                    }
                    "ExpressionType" => {
                        obj.expression_type = deserialize_primitive("ExpressionType", stack, Ok)?;
                    }
                    "RequestProgress" => {
                        obj.request_progress = Some(RequestProgressDeserializer::deserialize("RequestProgress", stack)?);
                    }
                    "InputSerialization" => {
                        obj.input_serialization = InputSerializationDeserializer::deserialize("InputSerialization", stack)?;
                    }
                    "OutputSerialization" => {
                        obj.output_serialization = OutputSerializationDeserializer::deserialize("OutputSerialization", stack)?;
                    }
                    "ScanRange" => {
                        obj.scan_range = Some(ScanRangeDeserializer::deserialize("ScanRange", stack)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct SelectParametersDeserializer;
impl SelectParametersDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SelectParameters, XmlParseError> {
        deserialize_elements::<_, SelectParameters, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "InputSerialization" => {
                        obj.input_serialization = InputSerializationDeserializer::deserialize("InputSerialization", stack)?;
                    }
                    "ExpressionType" => {
                        obj.expression_type = deserialize_primitive("ExpressionType", stack, Ok)?;
                    }
                    "Expression" => {
                        obj.expression = deserialize_primitive("Expression", stack, Ok)?;
                    }
                    "OutputSerialization" => {
                        obj.output_serialization = OutputSerializationDeserializer::deserialize("OutputSerialization", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServerSideEncryptionByDefault {
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: ServerSideEncryption,
    #[serde(rename = "KMSMasterKeyID")]
    pub kms_master_key_id: Option<SSEKMSKeyId>,
}
#[allow(dead_code)]
pub struct ServerSideEncryptionByDefaultDeserializer;
impl ServerSideEncryptionByDefaultDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServerSideEncryptionByDefault, XmlParseError> {
        deserialize_elements::<_, ServerSideEncryptionByDefault, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "SSEAlgorithm" => {
                        obj.sse_algorithm = deserialize_primitive("SSEAlgorithm", stack, Ok)?;
                    }
                    "KMSMasterKeyID" => {
                        obj.kms_master_key_id = Some(deserialize_primitive("KMSMasterKeyID", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServerSideEncryptionConfiguration {
    #[serde(rename = "Rules")]
    pub rules: ServerSideEncryptionRules,
}
#[allow(dead_code)]
pub struct ServerSideEncryptionConfigurationDeserializer;
impl ServerSideEncryptionConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServerSideEncryptionConfiguration, XmlParseError> {
        deserialize_elements::<_, ServerSideEncryptionConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Rules" => {
                        obj.rules
                            .extend(ServerSideEncryptionRulesDeserializer::deserialize("Rules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServerSideEncryptionRule {
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
}
#[allow(dead_code)]
pub struct ServerSideEncryptionRuleDeserializer;
impl ServerSideEncryptionRuleDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServerSideEncryptionRule, XmlParseError> {
        deserialize_elements::<_, ServerSideEncryptionRule, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ApplyServerSideEncryptionByDefault" => {
                        obj.apply_server_side_encryption_by_default = Some(ServerSideEncryptionByDefaultDeserializer::deserialize("ApplyServerSideEncryptionByDefault", stack)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SourceSelectionCriteria {
    #[serde(rename = "SseKmsEncryptedObjects")]
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
    #[serde(rename = "ReplicaModifications")]
    pub replica_modifications: Option<ReplicaModifications>,
}
#[allow(dead_code)]
pub struct SourceSelectionCriteriaDeserializer;
impl SourceSelectionCriteriaDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SourceSelectionCriteria, XmlParseError> {
        deserialize_elements::<_, SourceSelectionCriteria, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "SseKmsEncryptedObjects" => {
                        obj.sse_kms_encrypted_objects = Some(SseKmsEncryptedObjectsDeserializer::deserialize("SseKmsEncryptedObjects", stack)?);
                    }
                    "ReplicaModifications" => {
                        obj.replica_modifications = Some(ReplicaModificationsDeserializer::deserialize("ReplicaModifications", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SseKmsEncryptedObjects {
    #[serde(rename = "Status")]
    pub status: SseKmsEncryptedObjectsStatus,
}
#[allow(dead_code)]
pub struct SseKmsEncryptedObjectsDeserializer;
impl SseKmsEncryptedObjectsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SseKmsEncryptedObjects, XmlParseError> {
        deserialize_elements::<_, SseKmsEncryptedObjects, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = deserialize_primitive("Status", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "BytesScanned")]
    pub bytes_scanned: Option<BytesScanned>,
    #[serde(rename = "BytesProcessed")]
    pub bytes_processed: Option<BytesProcessed>,
    #[serde(rename = "BytesReturned")]
    pub bytes_returned: Option<BytesReturned>,
}
#[allow(dead_code)]
pub struct StatsDeserializer;
impl StatsDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Stats, XmlParseError> {
        deserialize_elements::<_, Stats, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "BytesScanned" => {
                        obj.bytes_scanned = Some(deserialize_primitive("BytesScanned", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    "BytesProcessed" => {
                        obj.bytes_processed = Some(deserialize_primitive("BytesProcessed", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    "BytesReturned" => {
                        obj.bytes_returned = Some(deserialize_primitive("BytesReturned", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StatsEvent {
    #[serde(rename = "Details")]
    pub details: Option<Stats>,
}
#[allow(dead_code)]
pub struct StatsEventDeserializer;
impl StatsEventDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StatsEvent, XmlParseError> {
        deserialize_elements::<_, StatsEvent, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Details" => {
                        obj.details = Some(StatsDeserializer::deserialize("Details", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StorageClassAnalysis {
    #[serde(rename = "DataExport")]
    pub data_export: Option<StorageClassAnalysisDataExport>,
}
#[allow(dead_code)]
pub struct StorageClassAnalysisDeserializer;
impl StorageClassAnalysisDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StorageClassAnalysis, XmlParseError> {
        deserialize_elements::<_, StorageClassAnalysis, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DataExport" => {
                        obj.data_export = Some(StorageClassAnalysisDataExportDeserializer::deserialize("DataExport", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StorageClassAnalysisDataExport {
    #[serde(rename = "OutputSchemaVersion")]
    pub output_schema_version: StorageClassAnalysisSchemaVersion,
    #[serde(rename = "Destination")]
    pub destination: AnalyticsExportDestination,
}
#[allow(dead_code)]
pub struct StorageClassAnalysisDataExportDeserializer;
impl StorageClassAnalysisDataExportDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StorageClassAnalysisDataExport, XmlParseError> {
        deserialize_elements::<_, StorageClassAnalysisDataExport, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "OutputSchemaVersion" => {
                        obj.output_schema_version = deserialize_primitive("OutputSchemaVersion", stack, Ok)?;
                    }
                    "Destination" => {
                        obj.destination = AnalyticsExportDestinationDeserializer::deserialize("Destination", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "Value")]
    pub value: Value,
}
#[allow(dead_code)]
pub struct TagDeserializer;
impl TagDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Tag, XmlParseError> {
        deserialize_elements::<_, Tag, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "Value" => {
                        obj.value = deserialize_primitive("Value", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tagging {
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
}
#[allow(dead_code)]
pub struct TaggingDeserializer;
impl TaggingDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Tagging, XmlParseError> {
        deserialize_elements::<_, Tagging, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TagSet" => {
                        obj.tag_set
                            .extend(TagSetDeserializer::deserialize("TagSet", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TargetGrant {
    #[serde(rename = "Grantee")]
    pub grantee: Option<Grantee>,
    #[serde(rename = "Permission")]
    pub permission: Option<BucketLogsPermission>,
}
#[allow(dead_code)]
pub struct TargetGrantDeserializer;
impl TargetGrantDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetGrant, XmlParseError> {
        deserialize_elements::<_, TargetGrant, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Grantee" => {
                        obj.grantee = Some(GranteeDeserializer::deserialize("Grantee", stack)?);
                    }
                    "Permission" => {
                        obj.permission = Some(deserialize_primitive("Permission", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tiering {
    #[serde(rename = "Days")]
    pub days: IntelligentTieringDays,
    #[serde(rename = "AccessTier")]
    pub access_tier: IntelligentTieringAccessTier,
}
#[allow(dead_code)]
pub struct TieringDeserializer;
impl TieringDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Tiering, XmlParseError> {
        deserialize_elements::<_, Tiering, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Days" => {
                        obj.days = deserialize_primitive("Days", stack, |s| Ok(i64::from_str(&s).unwrap()))?;
                    }
                    "AccessTier" => {
                        obj.access_tier = deserialize_primitive("AccessTier", stack, Ok)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TopicConfiguration {
    #[serde(rename = "Id")]
    pub id: Option<NotificationId>,
    #[serde(rename = "TopicArn")]
    pub topic_arn: TopicArn,
    #[serde(rename = "Events")]
    pub events: EventList,
    #[serde(rename = "Filter")]
    pub filter: Option<NotificationConfigurationFilter>,
}
#[allow(dead_code)]
pub struct TopicConfigurationDeserializer;
impl TopicConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TopicConfiguration, XmlParseError> {
        deserialize_elements::<_, TopicConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Id" => {
                        obj.id = Some(deserialize_primitive("Id", stack, Ok)?);
                    }
                    "TopicArn" => {
                        obj.topic_arn = deserialize_primitive("TopicArn", stack, Ok)?;
                    }
                    "Events" => {
                        obj.events
                            .extend(EventListDeserializer::deserialize("Events", stack)?);
                    }
                    "Filter" => {
                        obj.filter = Some(NotificationConfigurationFilterDeserializer::deserialize("Filter", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct TopicConfigurationDeprecatedDeserializer;
impl TopicConfigurationDeprecatedDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TopicConfigurationDeprecated, XmlParseError> {
        deserialize_elements::<_, TopicConfigurationDeprecated, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Id" => {
                        obj.id = Some(deserialize_primitive("Id", stack, Ok)?);
                    }
                    "Events" => {
                        obj.events
                            .get_or_insert(vec![])
                            .extend(EventListDeserializer::deserialize("Events", stack)?);
                    }
                    "Event" => {
                        obj.event = Some(deserialize_primitive("Event", stack, Ok)?);
                    }
                    "Topic" => {
                        obj.topic = Some(deserialize_primitive("Topic", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Transition {
    #[serde(rename = "Date")]
    pub date: Option<Date>,
    #[serde(rename = "Days")]
    pub days: Option<Days>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<TransitionStorageClass>,
}
#[allow(dead_code)]
pub struct TransitionDeserializer;
impl TransitionDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Transition, XmlParseError> {
        deserialize_elements::<_, Transition, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Date" => {
                        obj.date = Some(deserialize_primitive("Date", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "Days" => {
                        obj.days = Some(deserialize_primitive("Days", stack, |s| Ok(i64::from_str(&s).unwrap()))?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(deserialize_primitive("StorageClass", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct UploadPartCopyOutputDeserializer;
impl UploadPartCopyOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UploadPartCopyOutput, XmlParseError> {
        deserialize_elements::<_, UploadPartCopyOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CopySourceVersionId" => {
                        obj.copy_source_version_id = Some(deserialize_primitive("CopySourceVersionId", stack, Ok)?);
                    }
                    "CopyPartResult" => {
                        obj.copy_part_result = Some(CopyPartResultDeserializer::deserialize("CopyPartResult", stack)?);
                    }
                    "ServerSideEncryption" => {
                        obj.server_side_encryption = Some(deserialize_primitive("ServerSideEncryption", stack, Ok)?);
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "SSEKMSKeyId" => {
                        obj.ssekms_key_id = Some(deserialize_primitive("SSEKMSKeyId", stack, Ok)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UploadPartCopyRequest {
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "CopySource")]
    pub copy_source: CopySource,
    #[serde(rename = "CopySourceIfMatch")]
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    #[serde(rename = "CopySourceIfModifiedSince")]
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    #[serde(rename = "CopySourceIfNoneMatch")]
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    #[serde(rename = "CopySourceIfUnmodifiedSince")]
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    #[serde(rename = "CopySourceRange")]
    pub copy_source_range: Option<CopySourceRange>,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "PartNumber")]
    pub part_number: PartNumber,
    #[serde(rename = "UploadId")]
    pub upload_id: MultipartUploadId,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: Option<SSECustomerKey>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "CopySourceSSECustomerAlgorithm")]
    pub copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm>,
    #[serde(rename = "CopySourceSSECustomerKey")]
    pub copy_source_sse_customer_key: Option<CopySourceSSECustomerKey>,
    #[serde(rename = "CopySourceSSECustomerKeyMD5")]
    pub copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
    #[serde(rename = "ExpectedSourceBucketOwner")]
    pub expected_source_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct UploadPartCopyRequestDeserializer;
impl UploadPartCopyRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UploadPartCopyRequest, XmlParseError> {
        deserialize_elements::<_, UploadPartCopyRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "CopySource" => {
                        obj.copy_source = deserialize_primitive("CopySource", stack, Ok)?;
                    }
                    "CopySourceIfMatch" => {
                        obj.copy_source_if_match = Some(deserialize_primitive("CopySourceIfMatch", stack, Ok)?);
                    }
                    "CopySourceIfModifiedSince" => {
                        obj.copy_source_if_modified_since = Some(deserialize_primitive("CopySourceIfModifiedSince", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "CopySourceIfNoneMatch" => {
                        obj.copy_source_if_none_match = Some(deserialize_primitive("CopySourceIfNoneMatch", stack, Ok)?);
                    }
                    "CopySourceIfUnmodifiedSince" => {
                        obj.copy_source_if_unmodified_since = Some(deserialize_primitive("CopySourceIfUnmodifiedSince", stack, |s| Ok(String::from_str(&s).unwrap()))?);
                    }
                    "CopySourceRange" => {
                        obj.copy_source_range = Some(deserialize_primitive("CopySourceRange", stack, Ok)?);
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "PartNumber" => {
                        obj.part_number = deserialize_primitive("PartNumber", stack, |s| Ok(i64::from_str(&s).unwrap()))?;
                    }
                    "UploadId" => {
                        obj.upload_id = deserialize_primitive("UploadId", stack, Ok)?;
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKey" => {
                        obj.sse_customer_key = Some(deserialize_primitive("SSECustomerKey", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "CopySourceSSECustomerAlgorithm" => {
                        obj.copy_source_sse_customer_algorithm = Some(deserialize_primitive("CopySourceSSECustomerAlgorithm", stack, Ok)?);
                    }
                    "CopySourceSSECustomerKey" => {
                        obj.copy_source_sse_customer_key = Some(deserialize_primitive("CopySourceSSECustomerKey", stack, Ok)?);
                    }
                    "CopySourceSSECustomerKeyMD5" => {
                        obj.copy_source_sse_customer_key_md5 = Some(deserialize_primitive("CopySourceSSECustomerKeyMD5", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    "ExpectedSourceBucketOwner" => {
                        obj.expected_source_bucket_owner = Some(deserialize_primitive("ExpectedSourceBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct UploadPartOutputDeserializer;
impl UploadPartOutputDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UploadPartOutput, XmlParseError> {
        deserialize_elements::<_, UploadPartOutput, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ServerSideEncryption" => {
                        obj.server_side_encryption = Some(deserialize_primitive("ServerSideEncryption", stack, Ok)?);
                    }
                    "ETag" => {
                        obj.e_tag = Some(deserialize_primitive("ETag", stack, Ok)?);
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "SSEKMSKeyId" => {
                        obj.ssekms_key_id = Some(deserialize_primitive("SSEKMSKeyId", stack, Ok)?);
                    }
                    "BucketKeyEnabled" => {
                        obj.bucket_key_enabled = Some(deserialize_primitive("BucketKeyEnabled", stack, |s| Ok(bool::from_str(&s).unwrap()))?);
                    }
                    "RequestCharged" => {
                        obj.request_charged = Some(deserialize_primitive("RequestCharged", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UploadPartRequest {
    #[serde(rename = "Body")]
    pub body: Option<Body>,
    #[serde(rename = "Bucket")]
    pub bucket: BucketName,
    #[serde(rename = "ContentLength")]
    pub content_length: Option<ContentLength>,
    #[serde(rename = "ContentMD5")]
    pub content_md5: Option<ContentMD5>,
    #[serde(rename = "Key")]
    pub key: ObjectKey,
    #[serde(rename = "PartNumber")]
    pub part_number: PartNumber,
    #[serde(rename = "UploadId")]
    pub upload_id: MultipartUploadId,
    #[serde(rename = "SSECustomerAlgorithm")]
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    #[serde(rename = "SSECustomerKey")]
    pub sse_customer_key: Option<SSECustomerKey>,
    #[serde(rename = "SSECustomerKeyMD5")]
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    #[serde(rename = "RequestPayer")]
    pub request_payer: Option<RequestPayer>,
    #[serde(rename = "ExpectedBucketOwner")]
    pub expected_bucket_owner: Option<AccountId>,
}
#[allow(dead_code)]
pub struct UploadPartRequestDeserializer;
impl UploadPartRequestDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UploadPartRequest, XmlParseError> {
        deserialize_elements::<_, UploadPartRequest, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Body" => {
                        obj.body = Some(Default::default());
                    }
                    "Bucket" => {
                        obj.bucket = deserialize_primitive("Bucket", stack, Ok)?;
                    }
                    "ContentLength" => {
                        obj.content_length = Some(deserialize_primitive("ContentLength", stack, |s| Ok(u64::from_str(&s).unwrap()))?);
                    }
                    "ContentMD5" => {
                        obj.content_md5 = Some(deserialize_primitive("ContentMD5", stack, Ok)?);
                    }
                    "Key" => {
                        obj.key = deserialize_primitive("Key", stack, Ok)?;
                    }
                    "PartNumber" => {
                        obj.part_number = deserialize_primitive("PartNumber", stack, |s| Ok(i64::from_str(&s).unwrap()))?;
                    }
                    "UploadId" => {
                        obj.upload_id = deserialize_primitive("UploadId", stack, Ok)?;
                    }
                    "SSECustomerAlgorithm" => {
                        obj.sse_customer_algorithm = Some(deserialize_primitive("SSECustomerAlgorithm", stack, Ok)?);
                    }
                    "SSECustomerKey" => {
                        obj.sse_customer_key = Some(deserialize_primitive("SSECustomerKey", stack, Ok)?);
                    }
                    "SSECustomerKeyMD5" => {
                        obj.sse_customer_key_md5 = Some(deserialize_primitive("SSECustomerKeyMD5", stack, Ok)?);
                    }
                    "RequestPayer" => {
                        obj.request_payer = Some(deserialize_primitive("RequestPayer", stack, Ok)?);
                    }
                    "ExpectedBucketOwner" => {
                        obj.expected_bucket_owner = Some(deserialize_primitive("ExpectedBucketOwner", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersioningConfiguration {
    #[serde(rename = "MFADelete")]
    pub mfa_delete: Option<MFADelete>,
    #[serde(rename = "Status")]
    pub status: Option<BucketVersioningStatus>,
}
#[allow(dead_code)]
pub struct VersioningConfigurationDeserializer;
impl VersioningConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<VersioningConfiguration, XmlParseError> {
        deserialize_elements::<_, VersioningConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "MFADelete" => {
                        obj.mfa_delete = Some(deserialize_primitive("MFADelete", stack, Ok)?);
                    }
                    "Status" => {
                        obj.status = Some(deserialize_primitive("Status", stack, Ok)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
#[allow(dead_code)]
pub struct WebsiteConfigurationDeserializer;
impl WebsiteConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    pub fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<WebsiteConfiguration, XmlParseError> {
        deserialize_elements::<_, WebsiteConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ErrorDocument" => {
                        obj.error_document = Some(ErrorDocumentDeserializer::deserialize("ErrorDocument", stack)?);
                    }
                    "IndexDocument" => {
                        obj.index_document = Some(IndexDocumentDeserializer::deserialize("IndexDocument", stack)?);
                    }
                    "RedirectAllRequestsTo" => {
                        obj.redirect_all_requests_to = Some(RedirectAllRequestsToDeserializer::deserialize("RedirectAllRequestsTo", stack)?);
                    }
                    "RoutingRules" => {
                        obj.routing_rules
                            .get_or_insert(vec![])
                            .extend(RoutingRulesDeserializer::deserialize("RoutingRules", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

