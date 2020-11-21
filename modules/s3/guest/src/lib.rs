use assemblylift_core_iomod_guest::{call, export_iomod_guest};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::structs::*;

pub mod structs;

export_iomod_guest!(akkoro, aws, s3);

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

call!(abort_multipart_upload, AbortMultipartUploadRequest => Result<AbortMultipartUploadOutput, Error>);
call!(complete_multipart_upload, CompleteMultipartUploadRequest => Result<CompleteMultipartUploadOutput, Error>);
call!(copy_object, CopyObjectRequest => Result<CopyObjectOutput, Error>);
call!(create_bucket, CreateBucketRequest => Result<CreateBucketOutput, Error>);
call!(create_multipart_upload, CreateMultipartUploadRequest => Result<CreateMultipartUploadOutput, Error>);
call!(delete_bucket, DeleteBucketRequest => Result<(), Error>);
call!(delete_bucket_analytics_configuration, DeleteBucketAnalyticsConfigurationRequest => Result<(), Error>);
call!(delete_bucket_cors, DeleteBucketCorsRequest => Result<(), Error>);
call!(delete_bucket_encryption, DeleteBucketEncryptionRequest => Result<(), Error>);
call!(delete_bucket_intelligent_tiering_configuration, DeleteBucketIntelligentTieringConfigurationRequest => Result<(), Error>);
call!(delete_bucket_inventory_configuration, DeleteBucketInventoryConfigurationRequest => Result<(), Error>);
call!(delete_bucket_lifecycle, DeleteBucketLifecycleRequest => Result<(), Error>);
call!(delete_bucket_metrics_configuration, DeleteBucketMetricsConfigurationRequest => Result<(), Error>);
call!(delete_bucket_ownership_controls, DeleteBucketOwnershipControlsRequest => Result<(), Error>);
call!(delete_bucket_policy, DeleteBucketPolicyRequest => Result<(), Error>);
call!(delete_bucket_replication, DeleteBucketReplicationRequest => Result<(), Error>);
call!(delete_bucket_tagging, DeleteBucketTaggingRequest => Result<(), Error>);
call!(delete_bucket_website, DeleteBucketWebsiteRequest => Result<(), Error>);
call!(delete_object, DeleteObjectRequest => Result<DeleteObjectOutput, Error>);
call!(delete_object_tagging, DeleteObjectTaggingRequest => Result<DeleteObjectTaggingOutput, Error>);
call!(delete_objects, DeleteObjectsRequest => Result<DeleteObjectsOutput, Error>);
call!(delete_public_access_block, DeletePublicAccessBlockRequest => Result<(), Error>);
call!(get_bucket_accelerate_configuration, GetBucketAccelerateConfigurationRequest => Result<GetBucketAccelerateConfigurationOutput, Error>);
call!(get_bucket_acl, GetBucketAclRequest => Result<GetBucketAclOutput, Error>);
call!(get_bucket_analytics_configuration, GetBucketAnalyticsConfigurationRequest => Result<GetBucketAnalyticsConfigurationOutput, Error>);
call!(get_bucket_cors, GetBucketCorsRequest => Result<GetBucketCorsOutput, Error>);
call!(get_bucket_encryption, GetBucketEncryptionRequest => Result<GetBucketEncryptionOutput, Error>);
call!(get_bucket_intelligent_tiering_configuration, GetBucketIntelligentTieringConfigurationRequest => Result<GetBucketIntelligentTieringConfigurationOutput, Error>);
call!(get_bucket_inventory_configuration, GetBucketInventoryConfigurationRequest => Result<GetBucketInventoryConfigurationOutput, Error>);
call!(get_bucket_lifecycle, GetBucketLifecycleRequest => Result<GetBucketLifecycleOutput, Error>);
call!(get_bucket_lifecycle_configuration, GetBucketLifecycleConfigurationRequest => Result<GetBucketLifecycleConfigurationOutput, Error>);
call!(get_bucket_location, GetBucketLocationRequest => Result<GetBucketLocationOutput, Error>);
call!(get_bucket_logging, GetBucketLoggingRequest => Result<GetBucketLoggingOutput, Error>);
call!(get_bucket_metrics_configuration, GetBucketMetricsConfigurationRequest => Result<GetBucketMetricsConfigurationOutput, Error>);
call!(get_bucket_notification, GetBucketNotificationConfigurationRequest => Result<NotificationConfigurationDeprecated, Error>);
call!(get_bucket_notification_configuration, GetBucketNotificationConfigurationRequest => Result<NotificationConfiguration, Error>);
call!(get_bucket_ownership_controls, GetBucketOwnershipControlsRequest => Result<GetBucketOwnershipControlsOutput, Error>);
call!(get_bucket_policy, GetBucketPolicyRequest => Result<GetBucketPolicyOutput, Error>);
call!(get_bucket_policy_status, GetBucketPolicyStatusRequest => Result<GetBucketPolicyStatusOutput, Error>);
call!(get_bucket_replication, GetBucketReplicationRequest => Result<GetBucketReplicationOutput, Error>);
call!(get_bucket_request_payment, GetBucketRequestPaymentRequest => Result<GetBucketRequestPaymentOutput, Error>);
call!(get_bucket_tagging, GetBucketTaggingRequest => Result<GetBucketTaggingOutput, Error>);
call!(get_bucket_versioning, GetBucketVersioningRequest => Result<GetBucketVersioningOutput, Error>);
call!(get_bucket_website, GetBucketWebsiteRequest => Result<GetBucketWebsiteOutput, Error>);
call!(get_object, GetObjectRequest => Result<GetObjectOutput, Error>);
call!(get_object_acl, GetObjectAclRequest => Result<GetObjectAclOutput, Error>);
call!(get_object_legal_hold, GetObjectLegalHoldRequest => Result<GetObjectLegalHoldOutput, Error>);
call!(get_object_lock_configuration, GetObjectLockConfigurationRequest => Result<GetObjectLockConfigurationOutput, Error>);
call!(get_object_retention, GetObjectRetentionRequest => Result<GetObjectRetentionOutput, Error>);
call!(get_object_tagging, GetObjectTaggingRequest => Result<GetObjectTaggingOutput, Error>);
call!(get_object_torrent, GetObjectTorrentRequest => Result<GetObjectTorrentOutput, Error>);
call!(get_public_access_block, GetPublicAccessBlockRequest => Result<GetPublicAccessBlockOutput, Error>);
call!(head_bucket, HeadBucketRequest => Result<(), Error>);
call!(head_object, HeadObjectRequest => Result<HeadObjectOutput, Error>);
call!(list_bucket_analytics_configurations, ListBucketAnalyticsConfigurationsRequest => Result<ListBucketAnalyticsConfigurationsOutput, Error>);
call!(list_bucket_intelligent_tiering_configurations, ListBucketIntelligentTieringConfigurationsRequest => Result<ListBucketIntelligentTieringConfigurationsOutput, Error>);
call!(list_bucket_inventory_configurations, ListBucketInventoryConfigurationsRequest => Result<ListBucketInventoryConfigurationsOutput, Error>);
call!(list_bucket_metrics_configurations, ListBucketMetricsConfigurationsRequest => Result<ListBucketMetricsConfigurationsOutput, Error>);
call!(list_buckets, () => Result<ListBucketsOutput, Error>);
call!(list_multipart_uploads, ListMultipartUploadsRequest => Result<ListMultipartUploadsOutput, Error>);
call!(list_object_versions, ListObjectVersionsRequest => Result<ListObjectVersionsOutput, Error>);
call!(list_objects, ListObjectsRequest => Result<ListObjectsOutput, Error>);
call!(list_objects_v2, ListObjectsV2Request => Result<ListObjectsV2Output, Error>);
call!(list_parts, ListPartsRequest => Result<ListPartsOutput, Error>);
call!(put_bucket_accelerate_configuration, PutBucketAccelerateConfigurationRequest => Result<(), Error>);
call!(put_bucket_acl, PutBucketAclRequest => Result<(), Error>);
call!(put_bucket_analytics_configuration, PutBucketAnalyticsConfigurationRequest => Result<(), Error>);
call!(put_bucket_cors, PutBucketCorsRequest => Result<(), Error>);
call!(put_bucket_encryption, PutBucketEncryptionRequest => Result<(), Error>);
call!(put_bucket_intelligent_tiering_configuration, PutBucketIntelligentTieringConfigurationRequest => Result<(), Error>);
call!(put_bucket_inventory_configuration, PutBucketInventoryConfigurationRequest => Result<(), Error>);
call!(put_bucket_lifecycle, PutBucketLifecycleRequest => Result<(), Error>);
call!(put_bucket_lifecycle_configuration, PutBucketLifecycleConfigurationRequest => Result<(), Error>);
call!(put_bucket_logging, PutBucketLoggingRequest => Result<(), Error>);
call!(put_bucket_metrics_configuration, PutBucketMetricsConfigurationRequest => Result<(), Error>);
call!(put_bucket_notification, PutBucketNotificationRequest => Result<(), Error>);
call!(put_bucket_notification_configuration, PutBucketNotificationConfigurationRequest => Result<(), Error>);
call!(put_bucket_ownership_controls, PutBucketOwnershipControlsRequest => Result<(), Error>);
call!(put_bucket_policy, PutBucketPolicyRequest => Result<(), Error>);
call!(put_bucket_replication, PutBucketReplicationRequest => Result<(), Error>);
call!(put_bucket_request_payment, PutBucketRequestPaymentRequest => Result<(), Error>);
call!(put_bucket_tagging, PutBucketTaggingRequest => Result<(), Error>);
call!(put_bucket_versioning, PutBucketVersioningRequest => Result<(), Error>);
call!(put_bucket_website, PutBucketWebsiteRequest => Result<(), Error>);
call!(put_object, PutObjectRequest => Result<PutObjectOutput, Error>);
call!(put_object_acl, PutObjectAclRequest => Result<PutObjectAclOutput, Error>);
call!(put_object_legal_hold, PutObjectLegalHoldRequest => Result<PutObjectLegalHoldOutput, Error>);
call!(put_object_lock_configuration, PutObjectLockConfigurationRequest => Result<PutObjectLockConfigurationOutput, Error>);
call!(put_object_retention, PutObjectRetentionRequest => Result<PutObjectRetentionOutput, Error>);
call!(put_object_tagging, PutObjectTaggingRequest => Result<PutObjectTaggingOutput, Error>);
call!(put_public_access_block, PutPublicAccessBlockRequest => Result<(), Error>);
call!(restore_object, RestoreObjectRequest => Result<RestoreObjectOutput, Error>);
call!(select_object_content, SelectObjectContentRequest => Result<SelectObjectContentOutput, Error>);
call!(upload_part, UploadPartRequest => Result<UploadPartOutput, Error>);
call!(upload_part_copy, UploadPartCopyRequest => Result<UploadPartCopyOutput, Error>);
