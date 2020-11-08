use assemblylift_core_iomod::iomod;
use assemblylift_core_iomod::iomod_capnp::*;
use capnp::capability::Promise;
use capnp::{Error, ErrorKind};
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::future::BoxFuture;
use futures::{AsyncReadExt, FutureExt};
use futures_util::TryFutureExt;
use once_cell::sync::Lazy;
use rusoto_s3::S3Client;

use guest;

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
    iomod!(akkoro.aws.s3 => {
        abort_multipart_upload => abort_multipart_upload,
        complete_multipart_upload => complete_multipart_upload,
        copy_object => copy_object,
        create_bucket => create_bucket,
        create_multipart_upload => create_multipart_upload,
        delete_bucket => delete_bucket,
        delete_bucket_analytics_configuration => delete_bucket_analytics_configuration,
        delete_bucket_cors => delete_bucket_cors,
        delete_bucket_encryption => delete_bucket_encryption,
        delete_bucket_inventory_configuration => delete_bucket_inventory_configuration,
        delete_bucket_lifecycle => delete_bucket_lifecycle,
        delete_bucket_metrics_configuration => delete_bucket_metrics_configuration,
        delete_bucket_policy => delete_bucket_policy,
        delete_bucket_replication => delete_bucket_replication,
        delete_bucket_tagging => delete_bucket_tagging,
        delete_bucket_website => delete_bucket_website,
        delete_object => delete_object,
        delete_object_tagging => delete_object_tagging,
        delete_objects => delete_objects,
        delete_public_access_block => delete_public_access_block,
        get_bucket_accelerate_configuration => get_bucket_accelerate_configuration,
        get_bucket_acl => get_bucket_acl,
        get_bucket_analytics_configuration => get_bucket_analytics_configuration,
        get_bucket_cors => get_bucket_cors,
        get_bucket_encryption => get_bucket_encryption,
        get_bucket_inventory_configuration => get_bucket_inventory_configuration,
        get_bucket_lifecycle => get_bucket_lifecycle,
        get_bucket_lifecycle_configuration => get_bucket_lifecycle_configuration,
        get_bucket_location => get_bucket_location,
        get_bucket_logging => get_bucket_logging,
        get_bucket_metrics_configuration => get_bucket_metrics_configuration,
        get_bucket_notification => get_bucket_notification,
        get_bucket_notification_configuration => get_bucket_notification_configuration,
        get_bucket_policy => get_bucket_policy,
        get_bucket_policy_status => get_bucket_policy_status,
        get_bucket_replication => get_bucket_replication,
        get_bucket_request_payment => get_bucket_request_payment,
        get_bucket_tagging => get_bucket_tagging,
        get_bucket_versioning => get_bucket_versioning,
        get_bucket_website => get_bucket_website,
        get_object => get_object,
        get_object_acl => get_object_acl,
        get_object_legal_hold => get_object_legal_hold,
        get_object_lock_configuration => get_object_lock_configuration,
        get_object_retention => get_object_retention,
        get_object_tagging => get_object_tagging,
        get_object_torrent => get_object_torrent,
        get_public_access_block => get_public_access_block,
        head_bucket => head_bucket,
        head_object => head_object,
        list_bucket_analytics_configurations => list_bucket_analytics_configurations,
        list_bucket_inventory_configurations => list_bucket_inventory_configurations,
        list_bucket_metrics_configurations => list_bucket_metrics_configurations,
        list_buckets => list_buckets,
        list_multipart_uploads => list_multipart_uploads,
        list_object_versions => list_object_versions,
        list_objects => list_objects,
        list_objects_v2 => list_objects_v2,
        list_parts => list_parts,
        put_bucket_accelerate_configuration => put_bucket_accelerate_configuration,
        put_bucket_acl => put_bucket_acl,
        put_bucket_analytics_configuration => put_bucket_analytics_configuration,
        put_bucket_cors => put_bucket_cors,
        put_bucket_encryption => put_bucket_encryption,
        put_bucket_inventory_configuration => put_bucket_inventory_configuration,
        put_bucket_lifecycle => put_bucket_lifecycle,
        put_bucket_lifecycle_configuration => put_bucket_lifecycle_configuration,
        put_bucket_logging => put_bucket_logging,
        put_bucket_metrics_configuration => put_bucket_metrics_configuration,
        put_bucket_notification => put_bucket_notification,
        put_bucket_notification_configuration => put_bucket_notification_configuration,
        put_bucket_policy => put_bucket_policy,
        put_bucket_replication => put_bucket_replication,
        put_bucket_request_payment => put_bucket_request_payment,
        put_bucket_tagging => put_bucket_tagging,
        put_bucket_versioning => put_bucket_versioning,
        put_bucket_website => put_bucket_website,
        put_object => put_object,
        put_object_acl => put_object_acl,
        put_object_legal_hold => put_object_legal_hold,
        put_object_lock_configuration => put_object_lock_configuration,
        put_object_retention => put_object_retention,
        put_object_tagging => put_object_tagging,
        put_public_access_block => put_public_access_block,
        restore_object => restore_object,
        select_object_content => select_object_content,
        upload_part => upload_part,
        upload_part_copy => upload_part_copy
    });
}

s3!(abort_multipart_upload, AbortMultipartUploadOutput);
s3!(complete_multipart_upload, CompleteMultipartUploadOutput);
s3!(copy_object, CopyObjectOutput);
s3!(create_bucket, CreateBucketOutput);
s3!(create_multipart_upload, CreateMultipartUploadOutput);
s3!(delete_bucket, ());
s3!(delete_bucket_analytics_configuration, ());
s3!(delete_bucket_cors, ());
s3!(delete_bucket_encryption, ());
s3!(delete_bucket_inventory_configuration, ());
s3!(delete_bucket_lifecycle, ());
s3!(delete_bucket_metrics_configuration, ());
s3!(delete_bucket_policy, ());
s3!(delete_bucket_replication, ());
s3!(delete_bucket_tagging, ());
s3!(delete_bucket_website, ());
s3!(delete_object, DeleteObjectOutput);
s3!(delete_object_tagging, DeleteObjectTaggingOutput);
s3!(delete_objects, DeleteObjectsOutput);
s3!(delete_public_access_block, ());
s3!(get_bucket_accelerate_configuration, GetBucketAccelerateConfigurationOutput);
s3!(get_bucket_acl, GetBucketAclOutput);
s3!(get_bucket_analytics_configuration, GetBucketAnalyticsConfigurationOutput);
s3!(get_bucket_cors, GetBucketCorsOutput);
s3!(get_bucket_encryption, GetBucketEncryptionOutput);
s3!(get_bucket_inventory_configuration, GetBucketInventoryConfigurationOutput);
s3!(get_bucket_lifecycle, GetBucketLifecycleOutput);
s3!(get_bucket_lifecycle_configuration, GetBucketLifecycleConfigurationOutput);
s3!(get_bucket_location, GetBucketLocationOutput);
s3!(get_bucket_logging, GetBucketLoggingOutput);
s3!(get_bucket_metrics_configuration, GetBucketMetricsConfigurationOutput);
s3!(get_bucket_notification, NotificationConfigurationDeprecated);
s3!(get_bucket_notification_configuration, NotificationConfiguration);
// s3!(get_bucket_policy, GetBucketPolicyOutput);
s3!(get_bucket_policy_status, GetBucketPolicyStatusOutput);
s3!(get_bucket_replication, GetBucketReplicationOutput);
s3!(get_bucket_request_payment, GetBucketRequestPaymentOutput);
s3!(get_bucket_tagging, GetBucketTaggingOutput);
s3!(get_bucket_versioning, GetBucketVersioningOutput);
s3!(get_bucket_website, GetBucketWebsiteOutput);
s3!(get_object, guest::structs::GetObjectRequest => GetObjectOutput);
s3!(get_object_acl, GetObjectAclOutput);
s3!(get_object_legal_hold, GetObjectLegalHoldOutput);
s3!(get_object_lock_configuration, GetObjectLockConfigurationOutput);
s3!(get_object_retention, GetObjectRetentionOutput);
s3!(get_object_tagging, GetObjectTaggingOutput);
// s3!(get_object_torrent, GetObjectTorrentOutput);
s3!(get_public_access_block, GetPublicAccessBlockOutput);
s3!(head_bucket, ());
s3!(head_object, HeadObjectOutput);
s3!(list_bucket_analytics_configurations, ListBucketAnalyticsConfigurationsOutput);
s3!(list_bucket_inventory_configurations, ListBucketInventoryConfigurationsOutput);
s3!(list_bucket_metrics_configurations, ListBucketMetricsConfigurationsOutput);
s3_noinput!(list_buckets, ListBucketsOutput);
s3!(list_multipart_uploads, ListMultipartUploadsOutput);
s3!(list_object_versions, ListObjectVersionsOutput);
s3!(list_objects, ListObjectsOutput);
s3!(list_objects_v2, ListObjectsV2Output);
s3!(list_parts, ListPartsOutput);
s3!(put_bucket_accelerate_configuration, ());
s3!(put_bucket_acl, ());
s3!(put_bucket_analytics_configuration, ());
s3!(put_bucket_cors, ());
s3!(put_bucket_encryption, ());
s3!(put_bucket_inventory_configuration, ());
s3!(put_bucket_lifecycle, ());
s3!(put_bucket_lifecycle_configuration, ());
s3!(put_bucket_logging, ());
s3!(put_bucket_metrics_configuration, ());
s3!(put_bucket_notification, ());
s3!(put_bucket_notification_configuration, ());
s3!(put_bucket_policy, ());
s3!(put_bucket_replication, ());
s3!(put_bucket_request_payment, ());
s3!(put_bucket_tagging, ());
s3!(put_bucket_versioning, ());
s3!(put_bucket_website, ());
s3!(put_object, PutObjectOutput);
s3!(put_object_acl, PutObjectAclOutput);
s3!(put_object_legal_hold, PutObjectLegalHoldOutput);
s3!(put_object_lock_configuration, PutObjectLockConfigurationOutput);
s3!(put_object_retention, PutObjectRetentionOutput);
s3!(put_object_tagging, PutObjectTaggingOutput);
s3!(put_public_access_block, ());
s3!(restore_object, RestoreObjectOutput);
s3!(select_object_content, SelectObjectContentOutput);
s3!(upload_part, UploadPartOutput);
s3!(upload_part_copy, UploadPartCopyOutput);
