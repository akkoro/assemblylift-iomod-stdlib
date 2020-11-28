use assemblylift_core_iomod::iomod;
use futures::future::BoxFuture;
use once_cell::sync::Lazy;

mod client;

use guest::structs::*;

static CLIENT: Lazy<client::Client> = Lazy::new(|| {
    use std::env;
    let mut c = client::Client::new(
        String::from("s3"),
        env::var("AWS_REGION").unwrap_or(String::from("us-east-1")),
    );
    c.set_credentials(
        env::var("AWS_ACCESS_KEY_ID").unwrap(),
        env::var("AWS_SECRET_ACCESS_KEY").unwrap()
    );
    c
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
      delete_bucket_intelligent_tiering_configuration => delete_bucket_intelligent_tiering_configuration,
      delete_bucket_inventory_configuration => delete_bucket_inventory_configuration,
      delete_bucket_lifecycle => delete_bucket_lifecycle,
      delete_bucket_metrics_configuration => delete_bucket_metrics_configuration,
      delete_bucket_ownership_controls => delete_bucket_ownership_controls,
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
      get_bucket_intelligent_tiering_configuration => get_bucket_intelligent_tiering_configuration,
      get_bucket_inventory_configuration => get_bucket_inventory_configuration,
      get_bucket_lifecycle => get_bucket_lifecycle,
      get_bucket_lifecycle_configuration => get_bucket_lifecycle_configuration,
      get_bucket_location => get_bucket_location,
      get_bucket_logging => get_bucket_logging,
      get_bucket_metrics_configuration => get_bucket_metrics_configuration,
      get_bucket_notification => get_bucket_notification,
      get_bucket_notification_configuration => get_bucket_notification_configuration,
      get_bucket_ownership_controls => get_bucket_ownership_controls,
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
      list_bucket_intelligent_tiering_configurations => list_bucket_intelligent_tiering_configurations,
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
      put_bucket_intelligent_tiering_configuration => put_bucket_intelligent_tiering_configuration,
      put_bucket_inventory_configuration => put_bucket_inventory_configuration,
      put_bucket_lifecycle => put_bucket_lifecycle,
      put_bucket_lifecycle_configuration => put_bucket_lifecycle_configuration,
      put_bucket_logging => put_bucket_logging,
      put_bucket_metrics_configuration => put_bucket_metrics_configuration,
      put_bucket_notification => put_bucket_notification,
      put_bucket_notification_configuration => put_bucket_notification_configuration,
      put_bucket_ownership_controls => put_bucket_ownership_controls,
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
      upload_part_copy => upload_part_copy,
    });
}

pub fn abort_multipart_upload(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: AbortMultipartUploadRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __abort_multipart_upload(deserialized)
}

fn __abort_multipart_upload(input: AbortMultipartUploadRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<AbortMultipartUploadOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<AbortMultipartUploadOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn complete_multipart_upload(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CompleteMultipartUploadRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __complete_multipart_upload(deserialized)
}

fn __complete_multipart_upload(input: CompleteMultipartUploadRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("POST", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<CompleteMultipartUploadOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<CompleteMultipartUploadOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn copy_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CopyObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __copy_object(deserialized)
}

fn __copy_object(input: CopyObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<CopyObjectOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<CopyObjectOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn create_bucket(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateBucketRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_bucket(deserialized)
}

fn __create_bucket(input: CreateBucketRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<CreateBucketOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<CreateBucketOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn create_multipart_upload(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateMultipartUploadRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_multipart_upload(deserialized)
}

fn __create_multipart_upload(input: CreateMultipartUploadRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?uploads";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("POST", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<CreateMultipartUploadOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<CreateMultipartUploadOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket(deserialized)
}

fn __delete_bucket(input: DeleteBucketRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_analytics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketAnalyticsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_analytics_configuration(deserialized)
}

fn __delete_bucket_analytics_configuration(input: DeleteBucketAnalyticsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?analytics";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_cors(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketCorsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_cors(deserialized)
}

fn __delete_bucket_cors(input: DeleteBucketCorsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?cors";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_encryption(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketEncryptionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_encryption(deserialized)
}

fn __delete_bucket_encryption(input: DeleteBucketEncryptionRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?encryption";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_intelligent_tiering_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketIntelligentTieringConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_intelligent_tiering_configuration(deserialized)
}

fn __delete_bucket_intelligent_tiering_configuration(input: DeleteBucketIntelligentTieringConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?intelligent-tiering";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_inventory_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketInventoryConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_inventory_configuration(deserialized)
}

fn __delete_bucket_inventory_configuration(input: DeleteBucketInventoryConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?inventory";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_lifecycle(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketLifecycleRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_lifecycle(deserialized)
}

fn __delete_bucket_lifecycle(input: DeleteBucketLifecycleRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?lifecycle";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_metrics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketMetricsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_metrics_configuration(deserialized)
}

fn __delete_bucket_metrics_configuration(input: DeleteBucketMetricsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?metrics";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_ownership_controls(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketOwnershipControlsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_ownership_controls(deserialized)
}

fn __delete_bucket_ownership_controls(input: DeleteBucketOwnershipControlsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?ownershipControls";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketPolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_policy(deserialized)
}

fn __delete_bucket_policy(input: DeleteBucketPolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?policy";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_replication(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketReplicationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_replication(deserialized)
}

fn __delete_bucket_replication(input: DeleteBucketReplicationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?replication";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_tagging(deserialized)
}

fn __delete_bucket_tagging(input: DeleteBucketTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?tagging";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_bucket_website(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketWebsiteRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_website(deserialized)
}

fn __delete_bucket_website(input: DeleteBucketWebsiteRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?website";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_object(deserialized)
}

fn __delete_object(input: DeleteObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<DeleteObjectOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DeleteObjectOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_object_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteObjectTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_object_tagging(deserialized)
}

fn __delete_object_tagging(input: DeleteObjectTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?tagging";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<DeleteObjectTaggingOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DeleteObjectTaggingOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_objects(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteObjectsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_objects(deserialized)
}

fn __delete_objects(input: DeleteObjectsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?delete";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("POST", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<DeleteObjectsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<DeleteObjectsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn delete_public_access_block(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeletePublicAccessBlockRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_public_access_block(deserialized)
}

fn __delete_public_access_block(input: DeletePublicAccessBlockRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?publicAccessBlock";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_accelerate_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketAccelerateConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_accelerate_configuration(deserialized)
}

fn __get_bucket_accelerate_configuration(input: GetBucketAccelerateConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?accelerate";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketAccelerateConfigurationOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketAccelerateConfigurationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_acl(deserialized)
}

fn __get_bucket_acl(input: GetBucketAclRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?acl";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketAclOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketAclOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_analytics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketAnalyticsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_analytics_configuration(deserialized)
}

fn __get_bucket_analytics_configuration(input: GetBucketAnalyticsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?analytics";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketAnalyticsConfigurationOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketAnalyticsConfigurationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_cors(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketCorsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_cors(deserialized)
}

fn __get_bucket_cors(input: GetBucketCorsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?cors";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketCorsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketCorsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_encryption(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketEncryptionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_encryption(deserialized)
}

fn __get_bucket_encryption(input: GetBucketEncryptionRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?encryption";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketEncryptionOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketEncryptionOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_intelligent_tiering_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketIntelligentTieringConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_intelligent_tiering_configuration(deserialized)
}

fn __get_bucket_intelligent_tiering_configuration(input: GetBucketIntelligentTieringConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?intelligent-tiering";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketIntelligentTieringConfigurationOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketIntelligentTieringConfigurationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_inventory_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketInventoryConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_inventory_configuration(deserialized)
}

fn __get_bucket_inventory_configuration(input: GetBucketInventoryConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?inventory";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketInventoryConfigurationOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketInventoryConfigurationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_lifecycle(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLifecycleRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_lifecycle(deserialized)
}

fn __get_bucket_lifecycle(input: GetBucketLifecycleRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?lifecycle";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketLifecycleOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketLifecycleOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_lifecycle_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLifecycleConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_lifecycle_configuration(deserialized)
}

fn __get_bucket_lifecycle_configuration(input: GetBucketLifecycleConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?lifecycle";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketLifecycleConfigurationOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketLifecycleConfigurationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_location(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLocationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_location(deserialized)
}

fn __get_bucket_location(input: GetBucketLocationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?location";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketLocationOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketLocationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_logging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLoggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_logging(deserialized)
}

fn __get_bucket_logging(input: GetBucketLoggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?logging";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketLoggingOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketLoggingOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_metrics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketMetricsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_metrics_configuration(deserialized)
}

fn __get_bucket_metrics_configuration(input: GetBucketMetricsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?metrics";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketMetricsConfigurationOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketMetricsConfigurationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_notification(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketNotificationConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_notification(deserialized)
}

fn __get_bucket_notification(input: GetBucketNotificationConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?notification";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<NotificationConfigurationDeprecated, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<NotificationConfigurationDeprecated, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_notification_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketNotificationConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_notification_configuration(deserialized)
}

fn __get_bucket_notification_configuration(input: GetBucketNotificationConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?notification";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<NotificationConfiguration, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<NotificationConfiguration, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_ownership_controls(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketOwnershipControlsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_ownership_controls(deserialized)
}

fn __get_bucket_ownership_controls(input: GetBucketOwnershipControlsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?ownershipControls";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketOwnershipControlsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketOwnershipControlsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketPolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_policy(deserialized)
}

fn __get_bucket_policy(input: GetBucketPolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?policy";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketPolicyOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketPolicyOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_policy_status(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketPolicyStatusRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_policy_status(deserialized)
}

fn __get_bucket_policy_status(input: GetBucketPolicyStatusRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?policyStatus";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketPolicyStatusOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketPolicyStatusOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_replication(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketReplicationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_replication(deserialized)
}

fn __get_bucket_replication(input: GetBucketReplicationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?replication";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketReplicationOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketReplicationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_request_payment(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketRequestPaymentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_request_payment(deserialized)
}

fn __get_bucket_request_payment(input: GetBucketRequestPaymentRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?requestPayment";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketRequestPaymentOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketRequestPaymentOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_tagging(deserialized)
}

fn __get_bucket_tagging(input: GetBucketTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?tagging";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketTaggingOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketTaggingOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_versioning(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketVersioningRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_versioning(deserialized)
}

fn __get_bucket_versioning(input: GetBucketVersioningRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?versioning";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketVersioningOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketVersioningOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_bucket_website(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketWebsiteRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_website(deserialized)
}

fn __get_bucket_website(input: GetBucketWebsiteRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?website";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetBucketWebsiteOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetBucketWebsiteOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object(deserialized)
}

fn __get_object(input: GetObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetObjectOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetObjectOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_object_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_acl(deserialized)
}

fn __get_object_acl(input: GetObjectAclRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?acl";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetObjectAclOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetObjectAclOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_object_legal_hold(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectLegalHoldRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_legal_hold(deserialized)
}

fn __get_object_legal_hold(input: GetObjectLegalHoldRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?legal-hold";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetObjectLegalHoldOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetObjectLegalHoldOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_object_lock_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectLockConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_lock_configuration(deserialized)
}

fn __get_object_lock_configuration(input: GetObjectLockConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?object-lock";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetObjectLockConfigurationOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetObjectLockConfigurationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_object_retention(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectRetentionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_retention(deserialized)
}

fn __get_object_retention(input: GetObjectRetentionRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?retention";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetObjectRetentionOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetObjectRetentionOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_object_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_tagging(deserialized)
}

fn __get_object_tagging(input: GetObjectTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?tagging";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetObjectTaggingOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetObjectTaggingOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_object_torrent(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectTorrentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_torrent(deserialized)
}

fn __get_object_torrent(input: GetObjectTorrentRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?torrent";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetObjectTorrentOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetObjectTorrentOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn get_public_access_block(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetPublicAccessBlockRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_public_access_block(deserialized)
}

fn __get_public_access_block(input: GetPublicAccessBlockRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?publicAccessBlock";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<GetPublicAccessBlockOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<GetPublicAccessBlockOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn head_bucket(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: HeadBucketRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __head_bucket(deserialized)
}

fn __head_bucket(input: HeadBucketRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("HEAD", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn head_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: HeadObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __head_object(deserialized)
}

fn __head_object(input: HeadObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("HEAD", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<HeadObjectOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<HeadObjectOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn list_bucket_analytics_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketAnalyticsConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_analytics_configurations(deserialized)
}

fn __list_bucket_analytics_configurations(input: ListBucketAnalyticsConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?analytics";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<ListBucketAnalyticsConfigurationsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListBucketAnalyticsConfigurationsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn list_bucket_intelligent_tiering_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketIntelligentTieringConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_intelligent_tiering_configurations(deserialized)
}

fn __list_bucket_intelligent_tiering_configurations(input: ListBucketIntelligentTieringConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?intelligent-tiering";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<ListBucketIntelligentTieringConfigurationsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListBucketIntelligentTieringConfigurationsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn list_bucket_inventory_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketInventoryConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_inventory_configurations(deserialized)
}

fn __list_bucket_inventory_configurations(input: ListBucketInventoryConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?inventory";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<ListBucketInventoryConfigurationsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListBucketInventoryConfigurationsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn list_bucket_metrics_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketMetricsConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_metrics_configurations(deserialized)
}

fn __list_bucket_metrics_configurations(input: ListBucketMetricsConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?metrics";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<ListBucketMetricsConfigurationsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListBucketMetricsConfigurationsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn list_buckets(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: () = serde_json::from_slice(input.as_slice()).unwrap();
    __list_buckets(deserialized)
}

fn __list_buckets(input: ()) -> BoxFuture<'static, Vec<u8>> {
    let path = "/";

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                // FIXME this should be serde_xml_rs::from_slice -- converted to json by to_vec
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<ListBucketsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListBucketsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn list_multipart_uploads(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListMultipartUploadsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_multipart_uploads(deserialized)
}

fn __list_multipart_uploads(input: ListMultipartUploadsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?uploads";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<ListMultipartUploadsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListMultipartUploadsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn list_object_versions(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListObjectVersionsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_object_versions(deserialized)
}

fn __list_object_versions(input: ListObjectVersionsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?versions";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<ListObjectVersionsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListObjectVersionsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn list_objects(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListObjectsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_objects(deserialized)
}

fn __list_objects(input: ListObjectsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<ListObjectsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListObjectsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn list_objects_v2(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListObjectsV2Request = serde_json::from_slice(input.as_slice()).unwrap();
    __list_objects_v2(deserialized)
}

fn __list_objects_v2(input: ListObjectsV2Request) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?list-type=2";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<ListObjectsV2Output, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListObjectsV2Output, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn list_parts(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListPartsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_parts(deserialized)
}

fn __list_parts(input: ListPartsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<ListPartsOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<ListPartsOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_accelerate_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketAccelerateConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_accelerate_configuration(deserialized)
}

fn __put_bucket_accelerate_configuration(input: PutBucketAccelerateConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?accelerate";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_acl(deserialized)
}

fn __put_bucket_acl(input: PutBucketAclRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?acl";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_analytics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketAnalyticsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_analytics_configuration(deserialized)
}

fn __put_bucket_analytics_configuration(input: PutBucketAnalyticsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?analytics";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_cors(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketCorsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_cors(deserialized)
}

fn __put_bucket_cors(input: PutBucketCorsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?cors";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_encryption(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketEncryptionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_encryption(deserialized)
}

fn __put_bucket_encryption(input: PutBucketEncryptionRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?encryption";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_intelligent_tiering_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketIntelligentTieringConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_intelligent_tiering_configuration(deserialized)
}

fn __put_bucket_intelligent_tiering_configuration(input: PutBucketIntelligentTieringConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?intelligent-tiering";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_inventory_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketInventoryConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_inventory_configuration(deserialized)
}

fn __put_bucket_inventory_configuration(input: PutBucketInventoryConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?inventory";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_lifecycle(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketLifecycleRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_lifecycle(deserialized)
}

fn __put_bucket_lifecycle(input: PutBucketLifecycleRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?lifecycle";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_lifecycle_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketLifecycleConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_lifecycle_configuration(deserialized)
}

fn __put_bucket_lifecycle_configuration(input: PutBucketLifecycleConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?lifecycle";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_logging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketLoggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_logging(deserialized)
}

fn __put_bucket_logging(input: PutBucketLoggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?logging";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_metrics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketMetricsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_metrics_configuration(deserialized)
}

fn __put_bucket_metrics_configuration(input: PutBucketMetricsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?metrics";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_notification(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketNotificationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_notification(deserialized)
}

fn __put_bucket_notification(input: PutBucketNotificationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?notification";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_notification_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketNotificationConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_notification_configuration(deserialized)
}

fn __put_bucket_notification_configuration(input: PutBucketNotificationConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?notification";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_ownership_controls(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketOwnershipControlsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_ownership_controls(deserialized)
}

fn __put_bucket_ownership_controls(input: PutBucketOwnershipControlsRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?ownershipControls";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketPolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_policy(deserialized)
}

fn __put_bucket_policy(input: PutBucketPolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?policy";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_replication(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketReplicationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_replication(deserialized)
}

fn __put_bucket_replication(input: PutBucketReplicationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?replication";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_request_payment(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketRequestPaymentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_request_payment(deserialized)
}

fn __put_bucket_request_payment(input: PutBucketRequestPaymentRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?requestPayment";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_tagging(deserialized)
}

fn __put_bucket_tagging(input: PutBucketTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?tagging";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_versioning(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketVersioningRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_versioning(deserialized)
}

fn __put_bucket_versioning(input: PutBucketVersioningRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?versioning";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_bucket_website(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketWebsiteRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_website(deserialized)
}

fn __put_bucket_website(input: PutBucketWebsiteRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?website";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object(deserialized)
}

fn __put_object(input: PutObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<PutObjectOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PutObjectOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_object_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_acl(deserialized)
}

fn __put_object_acl(input: PutObjectAclRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?acl";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<PutObjectAclOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PutObjectAclOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_object_legal_hold(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectLegalHoldRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_legal_hold(deserialized)
}

fn __put_object_legal_hold(input: PutObjectLegalHoldRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?legal-hold";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<PutObjectLegalHoldOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PutObjectLegalHoldOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_object_lock_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectLockConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_lock_configuration(deserialized)
}

fn __put_object_lock_configuration(input: PutObjectLockConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?object-lock";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<PutObjectLockConfigurationOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PutObjectLockConfigurationOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_object_retention(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectRetentionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_retention(deserialized)
}

fn __put_object_retention(input: PutObjectRetentionRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?retention";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<PutObjectRetentionOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PutObjectRetentionOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_object_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_tagging(deserialized)
}

fn __put_object_tagging(input: PutObjectTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?tagging";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<PutObjectTaggingOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<PutObjectTaggingOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn put_public_access_block(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutPublicAccessBlockRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_public_access_block(deserialized)
}

fn __put_public_access_block(input: PutPublicAccessBlockRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}?publicAccessBlock";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = ();
                serde_json::to_vec(&Result::<(), guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn restore_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: RestoreObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __restore_object(deserialized)
}

fn __restore_object(input: RestoreObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?restore";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("POST", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<RestoreObjectOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<RestoreObjectOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn select_object_content(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: SelectObjectContentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __select_object_content(deserialized)
}

fn __select_object_content(input: SelectObjectContentRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}?select&amp;select-type=2";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("POST", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<SelectObjectContentOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<SelectObjectContentOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn upload_part(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UploadPartRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __upload_part(deserialized)
}

fn __upload_part(input: UploadPartRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<UploadPartOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UploadPartOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}

pub fn upload_part_copy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UploadPartCopyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __upload_part_copy(deserialized)
}

fn __upload_part_copy(input: UploadPartCopyRequest) -> BoxFuture<'static, Vec<u8>> {
    let path = "/{Bucket}/{Key+}";
    let path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    let path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", input).await {
            Ok(response) => {
                let response = serde_json::from_slice(response.as_slice()).unwrap();
                serde_json::to_vec(&Result::<UploadPartCopyOutput, guest::Error>::Ok(response)).unwrap()
            },
            Err(why) => {
                serde_json::to_vec(&Result::<UploadPartCopyOutput, guest::Error>::Err(guest::Error {
                    why: why.to_string(),
                }))
                    .unwrap()
            },
        }
    })
}
