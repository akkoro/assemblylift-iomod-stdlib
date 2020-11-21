use assemblylift_core_iomod::iomod;
use assemblylift_core_iomod::iomod_capnp::*;
use capnp::capability::Promise;
use capnp::{Error, ErrorKind};
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::future::BoxFuture;
use futures::{AsyncReadExt, FutureExt};
use futures_util::TryFutureExt;
use http::header::{HeaderName, HeaderValue, HeaderMap};
use hyper;
use hyper_tls::HttpsConnector;
use once_cell::sync::Lazy;
use rusoto_s3::{S3Client, *};
use rusoto_signature::{Region, SignedRequest};
use rusoto_signature::credential::AwsCredentials;

mod macros;

static S3: Lazy<S3Client> = Lazy::new(|| {
    S3Client::new(
        std::env::var("AWS_REGION")
            .unwrap_or(String::from("us-east-1"))
            .parse()
            .unwrap(),
    )
});

#[tokio::main]
async fn main() {
    let https = HttpsConnector::new();
    let client = hyper::Client::builder()
        .build::<_, hyper::Body>(https);
    let mut req = SignedRequest::new("GET", "s3", &Region::UsEast1, "/");
    req.sign(&AwsCredentials::default());

    // iomod!(akkoro.aws.s3 => {
    //     abort_multipart_upload => abort_multipart_upload,
    //     complete_multipart_upload => complete_multipart_upload,
    //     copy_object => copy_object,
    //     create_bucket => create_bucket,
    //     create_multipart_upload => create_multipart_upload,
    //     delete_bucket => delete_bucket,
    //     delete_bucket_analytics_configuration => delete_bucket_analytics_configuration,
    //     delete_bucket_cors => delete_bucket_cors,
    //     delete_bucket_encryption => delete_bucket_encryption,
    //     delete_bucket_inventory_configuration => delete_bucket_inventory_configuration,
    //     delete_bucket_lifecycle => delete_bucket_lifecycle,
    //     delete_bucket_metrics_configuration => delete_bucket_metrics_configuration,
    //     delete_bucket_policy => delete_bucket_policy,
    //     delete_bucket_replication => delete_bucket_replication,
    //     delete_bucket_tagging => delete_bucket_tagging,
    //     delete_bucket_website => delete_bucket_website,
    //     delete_object => delete_object,
    //     delete_object_tagging => delete_object_tagging,
    //     delete_objects => delete_objects,
    //     delete_public_access_block => delete_public_access_block,
    //     get_bucket_accelerate_configuration => get_bucket_accelerate_configuration,
    //     get_bucket_acl => get_bucket_acl,
    //     get_bucket_analytics_configuration => get_bucket_analytics_configuration,
    //     get_bucket_cors => get_bucket_cors,
    //     get_bucket_encryption => get_bucket_encryption,
    //     get_bucket_inventory_configuration => get_bucket_inventory_configuration,
    //     get_bucket_lifecycle => get_bucket_lifecycle,
    //     get_bucket_lifecycle_configuration => get_bucket_lifecycle_configuration,
    //     get_bucket_location => get_bucket_location,
    //     get_bucket_logging => get_bucket_logging,
    //     get_bucket_metrics_configuration => get_bucket_metrics_configuration,
    //     get_bucket_notification => get_bucket_notification,
    //     get_bucket_notification_configuration => get_bucket_notification_configuration,
    //     get_bucket_policy => get_bucket_policy,
    //     get_bucket_policy_status => get_bucket_policy_status,
    //     get_bucket_replication => get_bucket_replication,
    //     get_bucket_request_payment => get_bucket_request_payment,
    //     get_bucket_tagging => get_bucket_tagging,
    //     get_bucket_versioning => get_bucket_versioning,
    //     get_bucket_website => get_bucket_website,
    //     get_object => get_object,
    //     get_object_acl => get_object_acl,
    //     get_object_legal_hold => get_object_legal_hold,
    //     get_object_lock_configuration => get_object_lock_configuration,
    //     get_object_retention => get_object_retention,
    //     get_object_tagging => get_object_tagging,
    //     get_object_torrent => get_object_torrent,
    //     get_public_access_block => get_public_access_block,
    //     head_bucket => head_bucket,
    //     head_object => head_object,
    //     list_bucket_analytics_configurations => list_bucket_analytics_configurations,
    //     list_bucket_inventory_configurations => list_bucket_inventory_configurations,
    //     list_bucket_metrics_configurations => list_bucket_metrics_configurations,
    //     list_buckets => list_buckets,
    //     list_multipart_uploads => list_multipart_uploads,
    //     list_object_versions => list_object_versions,
    //     list_objects => list_objects,
    //     list_objects_v2 => list_objects_v2,
    //     list_parts => list_parts,
    //     put_bucket_accelerate_configuration => put_bucket_accelerate_configuration,
    //     put_bucket_acl => put_bucket_acl,
    //     put_bucket_analytics_configuration => put_bucket_analytics_configuration,
    //     put_bucket_cors => put_bucket_cors,
    //     put_bucket_encryption => put_bucket_encryption,
    //     put_bucket_inventory_configuration => put_bucket_inventory_configuration,
    //     put_bucket_lifecycle => put_bucket_lifecycle,
    //     put_bucket_lifecycle_configuration => put_bucket_lifecycle_configuration,
    //     put_bucket_logging => put_bucket_logging,
    //     put_bucket_metrics_configuration => put_bucket_metrics_configuration,
    //     put_bucket_notification => put_bucket_notification,
    //     put_bucket_notification_configuration => put_bucket_notification_configuration,
    //     put_bucket_policy => put_bucket_policy,
    //     put_bucket_replication => put_bucket_replication,
    //     put_bucket_request_payment => put_bucket_request_payment,
    //     put_bucket_tagging => put_bucket_tagging,
    //     put_bucket_versioning => put_bucket_versioning,
    //     put_bucket_website => put_bucket_website,
    //     put_object => put_object,
    //     put_object_acl => put_object_acl,
    //     put_object_legal_hold => put_object_legal_hold,
    //     put_object_lock_configuration => put_object_lock_configuration,
    //     put_object_retention => put_object_retention,
    //     put_object_tagging => put_object_tagging,
    //     put_public_access_block => put_public_access_block,
    //     restore_object => restore_object,
    //     select_object_content => select_object_content,
    //     upload_part => upload_part,
    //     upload_part_copy => upload_part_copy
    // });
}

// s3!(abort_multipart_upload, AbortMultipartUploadRequest => AbortMultipartUploadOutput);
// s3!(complete_multipart_upload, CompleteMultipartUploadRequest => CompleteMultipartUploadOutput);
// s3!(copy_object, CopyObjectRequest => CopyObjectOutput);
// s3!(create_bucket, CreateBucketRequest => CreateBucketOutput);
// s3!(create_multipart_upload, CreateMultipartUploadRequest => CreateMultipartUploadOutput);
// s3!(delete_bucket, DeleteBucketRequest => ());
// s3!(delete_bucket_analytics_configuration, DeleteBucketAnalyticsConfigurationRequest => ());
// s3!(delete_bucket_cors, DeleteBucketCorsRequest => ());
// s3!(delete_bucket_encryption, DeleteBucketEncryptionRequest => ());
// // s3!(delete_bucket_intelligent_tiering_configuration, DeleteBucketIntelligentTieringConfigurationRequest => ());
// s3!(delete_bucket_inventory_configuration, DeleteBucketInventoryConfigurationRequest => ());
// s3!(delete_bucket_lifecycle, DeleteBucketLifecycleRequest => ());
// s3!(delete_bucket_metrics_configuration, DeleteBucketMetricsConfigurationRequest => ());
// // s3!(delete_bucket_ownership_controls, DeleteBucketOwnershipControlsRequest => ());
// s3!(delete_bucket_policy, DeleteBucketPolicyRequest => ());
// s3!(delete_bucket_replication, DeleteBucketReplicationRequest => ());
// s3!(delete_bucket_tagging, DeleteBucketTaggingRequest => ());
// s3!(delete_bucket_website, DeleteBucketWebsiteRequest => ());
// s3!(delete_object, DeleteObjectRequest => DeleteObjectOutput);
// s3!(delete_object_tagging, DeleteObjectTaggingRequest => DeleteObjectTaggingOutput);
// s3!(delete_objects, DeleteObjectsRequest => DeleteObjectsOutput);
// s3!(delete_public_access_block, DeletePublicAccessBlockRequest => ());
// s3!(get_bucket_accelerate_configuration, GetBucketAccelerateConfigurationRequest => GetBucketAccelerateConfigurationOutput);
// s3!(get_bucket_acl, GetBucketAclRequest => GetBucketAclOutput);
// s3!(get_bucket_analytics_configuration, GetBucketAnalyticsConfigurationRequest => GetBucketAnalyticsConfigurationOutput);
// s3!(get_bucket_cors, GetBucketCorsRequest => GetBucketCorsOutput);
// s3!(get_bucket_encryption, GetBucketEncryptionRequest => GetBucketEncryptionOutput);
// // s3!(get_bucket_intelligent_tiering_configuration, GetBucketIntelligentTieringConfigurationRequest => GetBucketIntelligentTieringConfigurationOutput);
// s3!(get_bucket_inventory_configuration, GetBucketInventoryConfigurationRequest => GetBucketInventoryConfigurationOutput);
// s3!(get_bucket_lifecycle, GetBucketLifecycleRequest => GetBucketLifecycleOutput);
// s3!(get_bucket_lifecycle_configuration, GetBucketLifecycleConfigurationRequest => GetBucketLifecycleConfigurationOutput);
// s3!(get_bucket_location, GetBucketLocationRequest => GetBucketLocationOutput);
// s3!(get_bucket_logging, GetBucketLoggingRequest => GetBucketLoggingOutput);
// s3!(get_bucket_metrics_configuration, GetBucketMetricsConfigurationRequest => GetBucketMetricsConfigurationOutput);
// s3!(get_bucket_notification, GetBucketNotificationConfigurationRequest => NotificationConfigurationDeprecated);
// s3!(get_bucket_notification_configuration, GetBucketNotificationConfigurationRequest => NotificationConfiguration);
// // s3!(get_bucket_ownership_controls, GetBucketOwnershipControlsRequest => GetBucketOwnershipControlsOutput);
// s3!(get_bucket_policy, GetBucketPolicyRequest => GetBucketPolicyOutput);
// s3!(get_bucket_policy_status, GetBucketPolicyStatusRequest => GetBucketPolicyStatusOutput);
// s3!(get_bucket_replication, GetBucketReplicationRequest => GetBucketReplicationOutput);
// s3!(get_bucket_request_payment, GetBucketRequestPaymentRequest => GetBucketRequestPaymentOutput);
// s3!(get_bucket_tagging, GetBucketTaggingRequest => GetBucketTaggingOutput);
// s3!(get_bucket_versioning, GetBucketVersioningRequest => GetBucketVersioningOutput);
// s3!(get_bucket_website, GetBucketWebsiteRequest => GetBucketWebsiteOutput);
// s3!(get_object, GetObjectRequest => GetObjectOutput);
// s3!(get_object_acl, GetObjectAclRequest => GetObjectAclOutput);
// s3!(get_object_legal_hold, GetObjectLegalHoldRequest => GetObjectLegalHoldOutput);
// s3!(get_object_lock_configuration, GetObjectLockConfigurationRequest => GetObjectLockConfigurationOutput);
// s3!(get_object_retention, GetObjectRetentionRequest => GetObjectRetentionOutput);
// s3!(get_object_tagging, GetObjectTaggingRequest => GetObjectTaggingOutput);
// s3!(get_object_torrent, GetObjectTorrentRequest => GetObjectTorrentOutput);
// s3!(get_public_access_block, GetPublicAccessBlockRequest => GetPublicAccessBlockOutput);
// s3!(head_bucket, HeadBucketRequest => ());
// s3!(head_object, HeadObjectRequest => HeadObjectOutput);
// s3!(list_bucket_analytics_configurations, ListBucketAnalyticsConfigurationsRequest => ListBucketAnalyticsConfigurationsOutput);
// // s3!(list_bucket_intelligent_tiering_configurations, ListBucketIntelligentTieringConfigurationsRequest => ListBucketIntelligentTieringConfigurationsOutput);
// s3!(list_bucket_inventory_configurations, ListBucketInventoryConfigurationsRequest => ListBucketInventoryConfigurationsOutput);
// s3!(list_bucket_metrics_configurations, ListBucketMetricsConfigurationsRequest => ListBucketMetricsConfigurationsOutput);
// s3!(list_buckets, ListBucketsOutput);
// s3!(list_multipart_uploads, ListMultipartUploadsRequest => ListMultipartUploadsOutput);
// s3!(list_object_versions, ListObjectVersionsRequest => ListObjectVersionsOutput);
// s3!(list_objects, ListObjectsRequest => ListObjectsOutput);
// s3!(list_objects_v2, ListObjectsV2Request => ListObjectsV2Output);
// s3!(list_parts, ListPartsRequest => ListPartsOutput);
// s3!(put_bucket_accelerate_configuration, PutBucketAccelerateConfigurationRequest => ());
// s3!(put_bucket_acl, PutBucketAclRequest => ());
// s3!(put_bucket_analytics_configuration, PutBucketAnalyticsConfigurationRequest => ());
// s3!(put_bucket_cors, PutBucketCorsRequest => ());
// s3!(put_bucket_encryption, PutBucketEncryptionRequest => ());
// // s3!(put_bucket_intelligent_tiering_configuration, PutBucketIntelligentTieringConfigurationRequest => ());
// s3!(put_bucket_inventory_configuration, PutBucketInventoryConfigurationRequest => ());
// s3!(put_bucket_lifecycle, PutBucketLifecycleRequest => ());
// s3!(put_bucket_lifecycle_configuration, PutBucketLifecycleConfigurationRequest => ());
// s3!(put_bucket_logging, PutBucketLoggingRequest => ());
// s3!(put_bucket_metrics_configuration, PutBucketMetricsConfigurationRequest => ());
// s3!(put_bucket_notification, PutBucketNotificationRequest => ());
// s3!(put_bucket_notification_configuration, PutBucketNotificationConfigurationRequest => ());
// s3!(put_bucket_ownership_controls, PutBucketOwnershipControlsRequest => ());
// s3!(put_bucket_policy, PutBucketPolicyRequest => ());
// s3!(put_bucket_replication, PutBucketReplicationRequest => ());
// s3!(put_bucket_request_payment, PutBucketRequestPaymentRequest => ());
// s3!(put_bucket_tagging, PutBucketTaggingRequest => ());
// s3!(put_bucket_versioning, PutBucketVersioningRequest => ());
// s3!(put_bucket_website, PutBucketWebsiteRequest => ());
// s3!(put_object, PutObjectRequest => PutObjectOutput);
// s3!(put_object_acl, PutObjectAclRequest => PutObjectAclOutput);
// s3!(put_object_legal_hold, PutObjectLegalHoldRequest => PutObjectLegalHoldOutput);
// s3!(put_object_lock_configuration, PutObjectLockConfigurationRequest => PutObjectLockConfigurationOutput);
// s3!(put_object_retention, PutObjectRetentionRequest => PutObjectRetentionOutput);
// s3!(put_object_tagging, PutObjectTaggingRequest => PutObjectTaggingOutput);
// s3!(put_public_access_block, PutPublicAccessBlockRequest => ());
// s3!(restore_object, RestoreObjectRequest => RestoreObjectOutput);
// s3!(select_object_content, SelectObjectContentRequest => SelectObjectContentOutput);
// s3!(upload_part, UploadPartRequest => UploadPartOutput);
// s3!(upload_part_copy, UploadPartCopyRequest => UploadPartCopyOutput);
