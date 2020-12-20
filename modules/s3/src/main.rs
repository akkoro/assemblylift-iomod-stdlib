// This file is generated!
// See https://github.com/akkoro/asml-aws-codegen

mod client;

use std::str::FromStr;

use assemblylift_core_iomod::iomod;
use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use hyper::StatusCode;
use rusoto_signature::{Region, SignedRequest};

use xml;
use xml::reader::{EventReader, ParserConfig};

use guest::xml_util;
use guest::xml_util::util::Next;
use guest::structs::*;

static CLIENT: Lazy<client::Client> = Lazy::new(|| {
    use std::env;
    let mut c = client::Client::new();
    c.set_credentials(
        env::var("AWS_ACCESS_KEY_ID").unwrap(), 
        env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
        match env::var("AWS_SESSION_TOKEN") {
            Ok(token) => Some(token),
            Err(_) => None,
        },
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

#[allow(dead_code)]
pub fn abort_multipart_upload(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: AbortMultipartUploadRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __abort_multipart_upload(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __abort_multipart_upload(input: AbortMultipartUploadRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("uploadId", &serde_json::to_string(&input.upload_id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: AbortMultipartUploadOutput = Default::default();
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<AbortMultipartUploadOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<AbortMultipartUploadOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn complete_multipart_upload(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CompleteMultipartUploadRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __complete_multipart_upload(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __complete_multipart_upload(input: CompleteMultipartUploadRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(multipart_upload) = input.multipart_upload {
        body.insert("MultipartUpload", serde_json::to_string(&multipart_upload).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("uploadId", &serde_json::to_string(&input.upload_id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: CompleteMultipartUploadOutput = Default::default();
                        output.expiration = match response.headers().get("x-amz-expiration") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.server_side_encryption = match response.headers().get("x-amz-server-side-encryption") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.version_id = match response.headers().get("x-amz-version-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.ssekms_key_id = match response.headers().get("x-amz-server-side-encryption-aws-kms-key-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.bucket_key_enabled = match response.headers().get("x-amz-server-side-encryption-bucket-key-enabled") {
                            Some(v) => Some(bool::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match CompleteMultipartUploadOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<CompleteMultipartUploadOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<CompleteMultipartUploadOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn copy_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CopyObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __copy_object(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __copy_object(input: CopyObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(acl) = input.acl {
        http_request.add_header("x-amz-acl", &serde_json::to_string(&acl).unwrap());
    };
    if let Some(cache_control) = input.cache_control {
        http_request.add_header("Cache-Control", &serde_json::to_string(&cache_control).unwrap());
    };
    if let Some(content_disposition) = input.content_disposition {
        http_request.add_header("Content-Disposition", &serde_json::to_string(&content_disposition).unwrap());
    };
    if let Some(content_encoding) = input.content_encoding {
        http_request.add_header("Content-Encoding", &serde_json::to_string(&content_encoding).unwrap());
    };
    if let Some(content_language) = input.content_language {
        http_request.add_header("Content-Language", &serde_json::to_string(&content_language).unwrap());
    };
    if let Some(content_type) = input.content_type {
        http_request.add_header("Content-Type", &serde_json::to_string(&content_type).unwrap());
    };
    http_request.add_header("x-amz-copy-source", &serde_json::to_string(&input.copy_source).unwrap());
    if let Some(copy_source_if_match) = input.copy_source_if_match {
        http_request.add_header("x-amz-copy-source-if-match", &serde_json::to_string(&copy_source_if_match).unwrap());
    };
    if let Some(copy_source_if_modified_since) = input.copy_source_if_modified_since {
        http_request.add_header("x-amz-copy-source-if-modified-since", &serde_json::to_string(&copy_source_if_modified_since).unwrap());
    };
    if let Some(copy_source_if_none_match) = input.copy_source_if_none_match {
        http_request.add_header("x-amz-copy-source-if-none-match", &serde_json::to_string(&copy_source_if_none_match).unwrap());
    };
    if let Some(copy_source_if_unmodified_since) = input.copy_source_if_unmodified_since {
        http_request.add_header("x-amz-copy-source-if-unmodified-since", &serde_json::to_string(&copy_source_if_unmodified_since).unwrap());
    };
    if let Some(expires) = input.expires {
        http_request.add_header("Expires", &serde_json::to_string(&expires).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        http_request.add_header("x-amz-grant-full-control", &serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        http_request.add_header("x-amz-grant-read", &serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        http_request.add_header("x-amz-grant-read-acp", &serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        http_request.add_header("x-amz-grant-write-acp", &serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(metadata_directive) = input.metadata_directive {
        http_request.add_header("x-amz-metadata-directive", &serde_json::to_string(&metadata_directive).unwrap());
    };
    if let Some(tagging_directive) = input.tagging_directive {
        http_request.add_header("x-amz-tagging-directive", &serde_json::to_string(&tagging_directive).unwrap());
    };
    if let Some(server_side_encryption) = input.server_side_encryption {
        http_request.add_header("x-amz-server-side-encryption", &serde_json::to_string(&server_side_encryption).unwrap());
    };
    if let Some(storage_class) = input.storage_class {
        http_request.add_header("x-amz-storage-class", &serde_json::to_string(&storage_class).unwrap());
    };
    if let Some(website_redirect_location) = input.website_redirect_location {
        http_request.add_header("x-amz-website-redirect-location", &serde_json::to_string(&website_redirect_location).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        http_request.add_header("x-amz-server-side-encryption-customer-algorithm", &serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        http_request.add_header("x-amz-server-side-encryption-customer-key", &serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        http_request.add_header("x-amz-server-side-encryption-customer-key-MD5", &serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(ssekms_key_id) = input.ssekms_key_id {
        http_request.add_header("x-amz-server-side-encryption-aws-kms-key-id", &serde_json::to_string(&ssekms_key_id).unwrap());
    };
    if let Some(ssekms_encryption_context) = input.ssekms_encryption_context {
        http_request.add_header("x-amz-server-side-encryption-context", &serde_json::to_string(&ssekms_encryption_context).unwrap());
    };
    if let Some(bucket_key_enabled) = input.bucket_key_enabled {
        http_request.add_header("x-amz-server-side-encryption-bucket-key-enabled", &serde_json::to_string(&bucket_key_enabled).unwrap());
    };
    if let Some(copy_source_sse_customer_algorithm) = input.copy_source_sse_customer_algorithm {
        http_request.add_header("x-amz-copy-source-server-side-encryption-customer-algorithm", &serde_json::to_string(&copy_source_sse_customer_algorithm).unwrap());
    };
    if let Some(copy_source_sse_customer_key) = input.copy_source_sse_customer_key {
        http_request.add_header("x-amz-copy-source-server-side-encryption-customer-key", &serde_json::to_string(&copy_source_sse_customer_key).unwrap());
    };
    if let Some(copy_source_sse_customer_key_md5) = input.copy_source_sse_customer_key_md5 {
        http_request.add_header("x-amz-copy-source-server-side-encryption-customer-key-MD5", &serde_json::to_string(&copy_source_sse_customer_key_md5).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(tagging) = input.tagging {
        http_request.add_header("x-amz-tagging", &serde_json::to_string(&tagging).unwrap());
    };
    if let Some(object_lock_mode) = input.object_lock_mode {
        http_request.add_header("x-amz-object-lock-mode", &serde_json::to_string(&object_lock_mode).unwrap());
    };
    if let Some(object_lock_retain_until_date) = input.object_lock_retain_until_date {
        http_request.add_header("x-amz-object-lock-retain-until-date", &serde_json::to_string(&object_lock_retain_until_date).unwrap());
    };
    if let Some(object_lock_legal_hold_status) = input.object_lock_legal_hold_status {
        http_request.add_header("x-amz-object-lock-legal-hold", &serde_json::to_string(&object_lock_legal_hold_status).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(expected_source_bucket_owner) = input.expected_source_bucket_owner {
        http_request.add_header("x-amz-source-expected-bucket-owner", &serde_json::to_string(&expected_source_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: CopyObjectOutput = Default::default();
                        output.expiration = match response.headers().get("x-amz-expiration") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.copy_source_version_id = match response.headers().get("x-amz-copy-source-version-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.version_id = match response.headers().get("x-amz-version-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.server_side_encryption = match response.headers().get("x-amz-server-side-encryption") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_algorithm = match response.headers().get("x-amz-server-side-encryption-customer-algorithm") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_key_md5 = match response.headers().get("x-amz-server-side-encryption-customer-key-MD5") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.ssekms_key_id = match response.headers().get("x-amz-server-side-encryption-aws-kms-key-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.ssekms_encryption_context = match response.headers().get("x-amz-server-side-encryption-context") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.bucket_key_enabled = match response.headers().get("x-amz-server-side-encryption-bucket-key-enabled") {
                            Some(v) => Some(bool::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match CopyObjectOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<CopyObjectOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<CopyObjectOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn create_bucket(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateBucketRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_bucket(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __create_bucket(input: CreateBucketRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(create_bucket_configuration) = input.create_bucket_configuration {
        body.insert("CreateBucketConfiguration", serde_json::to_string(&create_bucket_configuration).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(acl) = input.acl {
        http_request.add_header("x-amz-acl", &serde_json::to_string(&acl).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        http_request.add_header("x-amz-grant-full-control", &serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        http_request.add_header("x-amz-grant-read", &serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        http_request.add_header("x-amz-grant-read-acp", &serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write) = input.grant_write {
        http_request.add_header("x-amz-grant-write", &serde_json::to_string(&grant_write).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        http_request.add_header("x-amz-grant-write-acp", &serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(object_lock_enabled_for_bucket) = input.object_lock_enabled_for_bucket {
        http_request.add_header("x-amz-bucket-object-lock-enabled", &serde_json::to_string(&object_lock_enabled_for_bucket).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: CreateBucketOutput = Default::default();
                        output.location = match response.headers().get("Location") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<CreateBucketOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<CreateBucketOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn create_multipart_upload(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateMultipartUploadRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_multipart_upload(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __create_multipart_upload(input: CreateMultipartUploadRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?uploads");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(acl) = input.acl {
        http_request.add_header("x-amz-acl", &serde_json::to_string(&acl).unwrap());
    };
    if let Some(cache_control) = input.cache_control {
        http_request.add_header("Cache-Control", &serde_json::to_string(&cache_control).unwrap());
    };
    if let Some(content_disposition) = input.content_disposition {
        http_request.add_header("Content-Disposition", &serde_json::to_string(&content_disposition).unwrap());
    };
    if let Some(content_encoding) = input.content_encoding {
        http_request.add_header("Content-Encoding", &serde_json::to_string(&content_encoding).unwrap());
    };
    if let Some(content_language) = input.content_language {
        http_request.add_header("Content-Language", &serde_json::to_string(&content_language).unwrap());
    };
    if let Some(content_type) = input.content_type {
        http_request.add_header("Content-Type", &serde_json::to_string(&content_type).unwrap());
    };
    if let Some(expires) = input.expires {
        http_request.add_header("Expires", &serde_json::to_string(&expires).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        http_request.add_header("x-amz-grant-full-control", &serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        http_request.add_header("x-amz-grant-read", &serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        http_request.add_header("x-amz-grant-read-acp", &serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        http_request.add_header("x-amz-grant-write-acp", &serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(server_side_encryption) = input.server_side_encryption {
        http_request.add_header("x-amz-server-side-encryption", &serde_json::to_string(&server_side_encryption).unwrap());
    };
    if let Some(storage_class) = input.storage_class {
        http_request.add_header("x-amz-storage-class", &serde_json::to_string(&storage_class).unwrap());
    };
    if let Some(website_redirect_location) = input.website_redirect_location {
        http_request.add_header("x-amz-website-redirect-location", &serde_json::to_string(&website_redirect_location).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        http_request.add_header("x-amz-server-side-encryption-customer-algorithm", &serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        http_request.add_header("x-amz-server-side-encryption-customer-key", &serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        http_request.add_header("x-amz-server-side-encryption-customer-key-MD5", &serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(ssekms_key_id) = input.ssekms_key_id {
        http_request.add_header("x-amz-server-side-encryption-aws-kms-key-id", &serde_json::to_string(&ssekms_key_id).unwrap());
    };
    if let Some(ssekms_encryption_context) = input.ssekms_encryption_context {
        http_request.add_header("x-amz-server-side-encryption-context", &serde_json::to_string(&ssekms_encryption_context).unwrap());
    };
    if let Some(bucket_key_enabled) = input.bucket_key_enabled {
        http_request.add_header("x-amz-server-side-encryption-bucket-key-enabled", &serde_json::to_string(&bucket_key_enabled).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(tagging) = input.tagging {
        http_request.add_header("x-amz-tagging", &serde_json::to_string(&tagging).unwrap());
    };
    if let Some(object_lock_mode) = input.object_lock_mode {
        http_request.add_header("x-amz-object-lock-mode", &serde_json::to_string(&object_lock_mode).unwrap());
    };
    if let Some(object_lock_retain_until_date) = input.object_lock_retain_until_date {
        http_request.add_header("x-amz-object-lock-retain-until-date", &serde_json::to_string(&object_lock_retain_until_date).unwrap());
    };
    if let Some(object_lock_legal_hold_status) = input.object_lock_legal_hold_status {
        http_request.add_header("x-amz-object-lock-legal-hold", &serde_json::to_string(&object_lock_legal_hold_status).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: CreateMultipartUploadOutput = Default::default();
                        output.abort_date = match response.headers().get("x-amz-abort-date") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.abort_rule_id = match response.headers().get("x-amz-abort-rule-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.server_side_encryption = match response.headers().get("x-amz-server-side-encryption") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_algorithm = match response.headers().get("x-amz-server-side-encryption-customer-algorithm") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_key_md5 = match response.headers().get("x-amz-server-side-encryption-customer-key-MD5") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.ssekms_key_id = match response.headers().get("x-amz-server-side-encryption-aws-kms-key-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.ssekms_encryption_context = match response.headers().get("x-amz-server-side-encryption-context") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.bucket_key_enabled = match response.headers().get("x-amz-server-side-encryption-bucket-key-enabled") {
                            Some(v) => Some(bool::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match CreateMultipartUploadOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<CreateMultipartUploadOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<CreateMultipartUploadOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket(input: DeleteBucketRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_analytics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketAnalyticsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_analytics_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_analytics_configuration(input: DeleteBucketAnalyticsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?analytics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_cors(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketCorsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_cors(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_cors(input: DeleteBucketCorsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?cors");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_encryption(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketEncryptionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_encryption(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_encryption(input: DeleteBucketEncryptionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?encryption");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_intelligent_tiering_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketIntelligentTieringConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_intelligent_tiering_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_intelligent_tiering_configuration(input: DeleteBucketIntelligentTieringConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?intelligent-tiering");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_inventory_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketInventoryConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_inventory_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_inventory_configuration(input: DeleteBucketInventoryConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?inventory");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_lifecycle(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketLifecycleRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_lifecycle(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_lifecycle(input: DeleteBucketLifecycleRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?lifecycle");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_metrics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketMetricsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_metrics_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_metrics_configuration(input: DeleteBucketMetricsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?metrics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_ownership_controls(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketOwnershipControlsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_ownership_controls(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_ownership_controls(input: DeleteBucketOwnershipControlsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?ownershipControls");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketPolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_policy(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_policy(input: DeleteBucketPolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?policy");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_replication(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketReplicationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_replication(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_replication(input: DeleteBucketReplicationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?replication");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_tagging(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_tagging(input: DeleteBucketTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?tagging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_bucket_website(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketWebsiteRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_website(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_bucket_website(input: DeleteBucketWebsiteRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?website");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_object(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_object(input: DeleteObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(mfa) = input.mfa {
        http_request.add_header("x-amz-mfa", &serde_json::to_string(&mfa).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(bypass_governance_retention) = input.bypass_governance_retention {
        http_request.add_header("x-amz-bypass-governance-retention", &serde_json::to_string(&bypass_governance_retention).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: DeleteObjectOutput = Default::default();
                        output.delete_marker = match response.headers().get("x-amz-delete-marker") {
                            Some(v) => Some(bool::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.version_id = match response.headers().get("x-amz-version-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<DeleteObjectOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DeleteObjectOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_object_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteObjectTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_object_tagging(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_object_tagging(input: DeleteObjectTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?tagging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: DeleteObjectTaggingOutput = Default::default();
                        output.version_id = match response.headers().get("x-amz-version-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<DeleteObjectTaggingOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DeleteObjectTaggingOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_objects(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteObjectsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_objects(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_objects(input: DeleteObjectsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?delete");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("Delete", serde_xml_rs::to_string(&input.delete).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(mfa) = input.mfa {
        http_request.add_header("x-amz-mfa", &serde_json::to_string(&mfa).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(bypass_governance_retention) = input.bypass_governance_retention {
        http_request.add_header("x-amz-bypass-governance-retention", &serde_json::to_string(&bypass_governance_retention).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: DeleteObjectsOutput = Default::default();
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match DeleteObjectsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<DeleteObjectsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<DeleteObjectsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn delete_public_access_block(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeletePublicAccessBlockRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_public_access_block(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __delete_public_access_block(input: DeletePublicAccessBlockRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?publicAccessBlock");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "DELETE",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_accelerate_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketAccelerateConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_accelerate_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_accelerate_configuration(input: GetBucketAccelerateConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?accelerate");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketAccelerateConfigurationOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketAccelerateConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketAccelerateConfigurationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketAccelerateConfigurationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_acl(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_acl(input: GetBucketAclRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?acl");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketAclOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketAclOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketAclOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketAclOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_analytics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketAnalyticsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_analytics_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_analytics_configuration(input: GetBucketAnalyticsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?analytics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketAnalyticsConfigurationOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketAnalyticsConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketAnalyticsConfigurationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketAnalyticsConfigurationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_cors(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketCorsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_cors(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_cors(input: GetBucketCorsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?cors");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketCorsOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketCorsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketCorsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketCorsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_encryption(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketEncryptionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_encryption(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_encryption(input: GetBucketEncryptionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?encryption");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketEncryptionOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketEncryptionOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketEncryptionOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketEncryptionOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_intelligent_tiering_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketIntelligentTieringConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_intelligent_tiering_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_intelligent_tiering_configuration(input: GetBucketIntelligentTieringConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?intelligent-tiering");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketIntelligentTieringConfigurationOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketIntelligentTieringConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketIntelligentTieringConfigurationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketIntelligentTieringConfigurationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_inventory_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketInventoryConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_inventory_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_inventory_configuration(input: GetBucketInventoryConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?inventory");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketInventoryConfigurationOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketInventoryConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketInventoryConfigurationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketInventoryConfigurationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_lifecycle(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLifecycleRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_lifecycle(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_lifecycle(input: GetBucketLifecycleRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?lifecycle");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketLifecycleOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketLifecycleOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketLifecycleOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketLifecycleOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_lifecycle_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLifecycleConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_lifecycle_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_lifecycle_configuration(input: GetBucketLifecycleConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?lifecycle");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketLifecycleConfigurationOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketLifecycleConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketLifecycleConfigurationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketLifecycleConfigurationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_location(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLocationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_location(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_location(input: GetBucketLocationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?location");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketLocationOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketLocationOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketLocationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketLocationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_logging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLoggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_logging(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_logging(input: GetBucketLoggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?logging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketLoggingOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketLoggingOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketLoggingOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketLoggingOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_metrics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketMetricsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_metrics_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_metrics_configuration(input: GetBucketMetricsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?metrics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketMetricsConfigurationOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketMetricsConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketMetricsConfigurationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketMetricsConfigurationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_notification(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketNotificationConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_notification(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_notification(input: GetBucketNotificationConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?notification");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: NotificationConfigurationDeprecated = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match NotificationConfigurationDeprecatedDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<NotificationConfigurationDeprecated, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<NotificationConfigurationDeprecated, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_notification_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketNotificationConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_notification_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_notification_configuration(input: GetBucketNotificationConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?notification");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: NotificationConfiguration = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match NotificationConfigurationDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<NotificationConfiguration, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<NotificationConfiguration, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_ownership_controls(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketOwnershipControlsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_ownership_controls(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_ownership_controls(input: GetBucketOwnershipControlsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?ownershipControls");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketOwnershipControlsOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketOwnershipControlsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketOwnershipControlsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketOwnershipControlsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketPolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_policy(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_policy(input: GetBucketPolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?policy");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketPolicyOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketPolicyOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketPolicyOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketPolicyOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_policy_status(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketPolicyStatusRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_policy_status(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_policy_status(input: GetBucketPolicyStatusRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?policyStatus");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketPolicyStatusOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketPolicyStatusOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketPolicyStatusOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketPolicyStatusOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_replication(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketReplicationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_replication(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_replication(input: GetBucketReplicationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?replication");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketReplicationOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketReplicationOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketReplicationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketReplicationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_request_payment(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketRequestPaymentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_request_payment(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_request_payment(input: GetBucketRequestPaymentRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?requestPayment");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketRequestPaymentOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketRequestPaymentOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketRequestPaymentOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketRequestPaymentOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_tagging(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_tagging(input: GetBucketTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?tagging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketTaggingOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketTaggingOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketTaggingOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketTaggingOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_versioning(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketVersioningRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_versioning(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_versioning(input: GetBucketVersioningRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?versioning");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketVersioningOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketVersioningOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketVersioningOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketVersioningOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_bucket_website(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketWebsiteRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_website(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_bucket_website(input: GetBucketWebsiteRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?website");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketWebsiteOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetBucketWebsiteOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetBucketWebsiteOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetBucketWebsiteOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_object(input: GetObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(if_match) = input.if_match {
        http_request.add_header("If-Match", &serde_json::to_string(&if_match).unwrap());
    };
    if let Some(if_modified_since) = input.if_modified_since {
        http_request.add_header("If-Modified-Since", &serde_json::to_string(&if_modified_since).unwrap());
    };
    if let Some(if_none_match) = input.if_none_match {
        http_request.add_header("If-None-Match", &serde_json::to_string(&if_none_match).unwrap());
    };
    if let Some(if_unmodified_since) = input.if_unmodified_since {
        http_request.add_header("If-Unmodified-Since", &serde_json::to_string(&if_unmodified_since).unwrap());
    };
    if let Some(range) = input.range {
        http_request.add_header("Range", &serde_json::to_string(&range).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        http_request.add_header("x-amz-server-side-encryption-customer-algorithm", &serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        http_request.add_header("x-amz-server-side-encryption-customer-key", &serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        http_request.add_header("x-amz-server-side-encryption-customer-key-MD5", &serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(response_cache_control) = input.response_cache_control {
        http_request.add_param("response-cache-control", &serde_json::to_string(&response_cache_control).unwrap());
    };
    if let Some(response_content_disposition) = input.response_content_disposition {
        http_request.add_param("response-content-disposition", &serde_json::to_string(&response_content_disposition).unwrap());
    };
    if let Some(response_content_encoding) = input.response_content_encoding {
        http_request.add_param("response-content-encoding", &serde_json::to_string(&response_content_encoding).unwrap());
    };
    if let Some(response_content_language) = input.response_content_language {
        http_request.add_param("response-content-language", &serde_json::to_string(&response_content_language).unwrap());
    };
    if let Some(response_content_type) = input.response_content_type {
        http_request.add_param("response-content-type", &serde_json::to_string(&response_content_type).unwrap());
    };
    if let Some(response_expires) = input.response_expires {
        http_request.add_param("response-expires", &serde_json::to_string(&response_expires).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };
    if let Some(part_number) = input.part_number {
        http_request.add_param("partNumber", &serde_json::to_string(&part_number).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectOutput = Default::default();
                        output.delete_marker = match response.headers().get("x-amz-delete-marker") {
                            Some(v) => Some(bool::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.accept_ranges = match response.headers().get("accept-ranges") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.expiration = match response.headers().get("x-amz-expiration") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.restore = match response.headers().get("x-amz-restore") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.last_modified = match response.headers().get("Last-Modified") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.content_length = match response.headers().get("Content-Length") {
                            Some(v) => Some(u64::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.e_tag = match response.headers().get("ETag") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.missing_meta = match response.headers().get("x-amz-missing-meta") {
                            Some(v) => Some(i64::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.version_id = match response.headers().get("x-amz-version-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.cache_control = match response.headers().get("Cache-Control") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.content_disposition = match response.headers().get("Content-Disposition") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.content_encoding = match response.headers().get("Content-Encoding") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.content_language = match response.headers().get("Content-Language") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.content_range = match response.headers().get("Content-Range") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.content_type = match response.headers().get("Content-Type") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.expires = match response.headers().get("Expires") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.website_redirect_location = match response.headers().get("x-amz-website-redirect-location") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.server_side_encryption = match response.headers().get("x-amz-server-side-encryption") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_algorithm = match response.headers().get("x-amz-server-side-encryption-customer-algorithm") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_key_md5 = match response.headers().get("x-amz-server-side-encryption-customer-key-MD5") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.ssekms_key_id = match response.headers().get("x-amz-server-side-encryption-aws-kms-key-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.bucket_key_enabled = match response.headers().get("x-amz-server-side-encryption-bucket-key-enabled") {
                            Some(v) => Some(bool::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.storage_class = match response.headers().get("x-amz-storage-class") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.replication_status = match response.headers().get("x-amz-replication-status") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.parts_count = match response.headers().get("x-amz-mp-parts-count") {
                            Some(v) => Some(i64::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.tag_count = match response.headers().get("x-amz-tagging-count") {
                            Some(v) => Some(i64::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.object_lock_mode = match response.headers().get("x-amz-object-lock-mode") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.object_lock_retain_until_date = match response.headers().get("x-amz-object-lock-retain-until-date") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.object_lock_legal_hold_status = match response.headers().get("x-amz-object-lock-legal-hold") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        output.body = Some(Vec::from(body));

                        serde_json::to_vec(&Result::<GetObjectOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetObjectOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_object_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_acl(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_object_acl(input: GetObjectAclRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?acl");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectAclOutput = Default::default();
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetObjectAclOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetObjectAclOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetObjectAclOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_object_legal_hold(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectLegalHoldRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_legal_hold(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_object_legal_hold(input: GetObjectLegalHoldRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?legal-hold");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectLegalHoldOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetObjectLegalHoldOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetObjectLegalHoldOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetObjectLegalHoldOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_object_lock_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectLockConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_lock_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_object_lock_configuration(input: GetObjectLockConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?object-lock");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectLockConfigurationOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetObjectLockConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetObjectLockConfigurationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetObjectLockConfigurationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_object_retention(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectRetentionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_retention(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_object_retention(input: GetObjectRetentionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?retention");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectRetentionOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetObjectRetentionOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetObjectRetentionOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetObjectRetentionOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_object_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_tagging(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_object_tagging(input: GetObjectTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?tagging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectTaggingOutput = Default::default();
                        output.version_id = match response.headers().get("x-amz-version-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetObjectTaggingOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetObjectTaggingOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetObjectTaggingOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_object_torrent(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectTorrentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_torrent(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_object_torrent(input: GetObjectTorrentRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?torrent");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectTorrentOutput = Default::default();
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        output.body = Some(Vec::from(body));

                        serde_json::to_vec(&Result::<GetObjectTorrentOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetObjectTorrentOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn get_public_access_block(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetPublicAccessBlockRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_public_access_block(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __get_public_access_block(input: GetPublicAccessBlockRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?publicAccessBlock");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: GetPublicAccessBlockOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetPublicAccessBlockOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<GetPublicAccessBlockOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<GetPublicAccessBlockOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn head_bucket(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: HeadBucketRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __head_bucket(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __head_bucket(input: HeadBucketRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "HEAD",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn head_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: HeadObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __head_object(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __head_object(input: HeadObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "HEAD",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(if_match) = input.if_match {
        http_request.add_header("If-Match", &serde_json::to_string(&if_match).unwrap());
    };
    if let Some(if_modified_since) = input.if_modified_since {
        http_request.add_header("If-Modified-Since", &serde_json::to_string(&if_modified_since).unwrap());
    };
    if let Some(if_none_match) = input.if_none_match {
        http_request.add_header("If-None-Match", &serde_json::to_string(&if_none_match).unwrap());
    };
    if let Some(if_unmodified_since) = input.if_unmodified_since {
        http_request.add_header("If-Unmodified-Since", &serde_json::to_string(&if_unmodified_since).unwrap());
    };
    if let Some(range) = input.range {
        http_request.add_header("Range", &serde_json::to_string(&range).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        http_request.add_header("x-amz-server-side-encryption-customer-algorithm", &serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        http_request.add_header("x-amz-server-side-encryption-customer-key", &serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        http_request.add_header("x-amz-server-side-encryption-customer-key-MD5", &serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };
    if let Some(part_number) = input.part_number {
        http_request.add_param("partNumber", &serde_json::to_string(&part_number).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: HeadObjectOutput = Default::default();
                        output.delete_marker = match response.headers().get("x-amz-delete-marker") {
                            Some(v) => Some(bool::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.accept_ranges = match response.headers().get("accept-ranges") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.expiration = match response.headers().get("x-amz-expiration") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.restore = match response.headers().get("x-amz-restore") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.archive_status = match response.headers().get("x-amz-archive-status") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.last_modified = match response.headers().get("Last-Modified") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.content_length = match response.headers().get("Content-Length") {
                            Some(v) => Some(u64::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.e_tag = match response.headers().get("ETag") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.missing_meta = match response.headers().get("x-amz-missing-meta") {
                            Some(v) => Some(i64::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.version_id = match response.headers().get("x-amz-version-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.cache_control = match response.headers().get("Cache-Control") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.content_disposition = match response.headers().get("Content-Disposition") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.content_encoding = match response.headers().get("Content-Encoding") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.content_language = match response.headers().get("Content-Language") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.content_type = match response.headers().get("Content-Type") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.expires = match response.headers().get("Expires") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.website_redirect_location = match response.headers().get("x-amz-website-redirect-location") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.server_side_encryption = match response.headers().get("x-amz-server-side-encryption") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_algorithm = match response.headers().get("x-amz-server-side-encryption-customer-algorithm") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_key_md5 = match response.headers().get("x-amz-server-side-encryption-customer-key-MD5") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.ssekms_key_id = match response.headers().get("x-amz-server-side-encryption-aws-kms-key-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.bucket_key_enabled = match response.headers().get("x-amz-server-side-encryption-bucket-key-enabled") {
                            Some(v) => Some(bool::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.storage_class = match response.headers().get("x-amz-storage-class") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.replication_status = match response.headers().get("x-amz-replication-status") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.parts_count = match response.headers().get("x-amz-mp-parts-count") {
                            Some(v) => Some(i64::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.object_lock_mode = match response.headers().get("x-amz-object-lock-mode") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.object_lock_retain_until_date = match response.headers().get("x-amz-object-lock-retain-until-date") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.object_lock_legal_hold_status = match response.headers().get("x-amz-object-lock-legal-hold") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<HeadObjectOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<HeadObjectOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn list_bucket_analytics_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketAnalyticsConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_analytics_configurations(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_bucket_analytics_configurations(input: ListBucketAnalyticsConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?analytics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(continuation_token) = input.continuation_token {
        http_request.add_param("continuation-token", &serde_json::to_string(&continuation_token).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: ListBucketAnalyticsConfigurationsOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match ListBucketAnalyticsConfigurationsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<ListBucketAnalyticsConfigurationsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListBucketAnalyticsConfigurationsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn list_bucket_intelligent_tiering_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketIntelligentTieringConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_intelligent_tiering_configurations(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_bucket_intelligent_tiering_configurations(input: ListBucketIntelligentTieringConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?intelligent-tiering");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(continuation_token) = input.continuation_token {
        http_request.add_param("continuation-token", &serde_json::to_string(&continuation_token).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: ListBucketIntelligentTieringConfigurationsOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match ListBucketIntelligentTieringConfigurationsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<ListBucketIntelligentTieringConfigurationsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListBucketIntelligentTieringConfigurationsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn list_bucket_inventory_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketInventoryConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_inventory_configurations(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_bucket_inventory_configurations(input: ListBucketInventoryConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?inventory");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(continuation_token) = input.continuation_token {
        http_request.add_param("continuation-token", &serde_json::to_string(&continuation_token).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: ListBucketInventoryConfigurationsOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match ListBucketInventoryConfigurationsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<ListBucketInventoryConfigurationsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListBucketInventoryConfigurationsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn list_bucket_metrics_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketMetricsConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_metrics_configurations(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_bucket_metrics_configurations(input: ListBucketMetricsConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?metrics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(continuation_token) = input.continuation_token {
        http_request.add_param("continuation-token", &serde_json::to_string(&continuation_token).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: ListBucketMetricsConfigurationsOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match ListBucketMetricsConfigurationsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<ListBucketMetricsConfigurationsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListBucketMetricsConfigurationsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn list_buckets(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: () = serde_json::from_slice(input.as_slice()).unwrap();
    __list_buckets(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_buckets(input: ()) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: ListBucketsOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match ListBucketsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<ListBucketsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListBucketsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn list_multipart_uploads(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListMultipartUploadsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_multipart_uploads(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_multipart_uploads(input: ListMultipartUploadsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?uploads");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(delimiter) = input.delimiter {
        http_request.add_param("delimiter", &serde_json::to_string(&delimiter).unwrap());
    };
    if let Some(encoding_type) = input.encoding_type {
        http_request.add_param("encoding-type", &serde_json::to_string(&encoding_type).unwrap());
    };
    if let Some(key_marker) = input.key_marker {
        http_request.add_param("key-marker", &serde_json::to_string(&key_marker).unwrap());
    };
    if let Some(max_uploads) = input.max_uploads {
        http_request.add_param("max-uploads", &serde_json::to_string(&max_uploads).unwrap());
    };
    if let Some(prefix) = input.prefix {
        http_request.add_param("prefix", &serde_json::to_string(&prefix).unwrap());
    };
    if let Some(upload_id_marker) = input.upload_id_marker {
        http_request.add_param("upload-id-marker", &serde_json::to_string(&upload_id_marker).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: ListMultipartUploadsOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match ListMultipartUploadsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<ListMultipartUploadsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListMultipartUploadsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn list_object_versions(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListObjectVersionsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_object_versions(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_object_versions(input: ListObjectVersionsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?versions");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(delimiter) = input.delimiter {
        http_request.add_param("delimiter", &serde_json::to_string(&delimiter).unwrap());
    };
    if let Some(encoding_type) = input.encoding_type {
        http_request.add_param("encoding-type", &serde_json::to_string(&encoding_type).unwrap());
    };
    if let Some(key_marker) = input.key_marker {
        http_request.add_param("key-marker", &serde_json::to_string(&key_marker).unwrap());
    };
    if let Some(max_keys) = input.max_keys {
        http_request.add_param("max-keys", &serde_json::to_string(&max_keys).unwrap());
    };
    if let Some(prefix) = input.prefix {
        http_request.add_param("prefix", &serde_json::to_string(&prefix).unwrap());
    };
    if let Some(version_id_marker) = input.version_id_marker {
        http_request.add_param("version-id-marker", &serde_json::to_string(&version_id_marker).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: ListObjectVersionsOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match ListObjectVersionsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<ListObjectVersionsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListObjectVersionsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn list_objects(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListObjectsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_objects(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_objects(input: ListObjectsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(delimiter) = input.delimiter {
        http_request.add_param("delimiter", &serde_json::to_string(&delimiter).unwrap());
    };
    if let Some(encoding_type) = input.encoding_type {
        http_request.add_param("encoding-type", &serde_json::to_string(&encoding_type).unwrap());
    };
    if let Some(marker) = input.marker {
        http_request.add_param("marker", &serde_json::to_string(&marker).unwrap());
    };
    if let Some(max_keys) = input.max_keys {
        http_request.add_param("max-keys", &serde_json::to_string(&max_keys).unwrap());
    };
    if let Some(prefix) = input.prefix {
        http_request.add_param("prefix", &serde_json::to_string(&prefix).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: ListObjectsOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match ListObjectsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<ListObjectsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListObjectsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn list_objects_v2(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListObjectsV2Request = serde_json::from_slice(input.as_slice()).unwrap();
    __list_objects_v2(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_objects_v2(input: ListObjectsV2Request) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?list-type=2");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(delimiter) = input.delimiter {
        http_request.add_param("delimiter", &serde_json::to_string(&delimiter).unwrap());
    };
    if let Some(encoding_type) = input.encoding_type {
        http_request.add_param("encoding-type", &serde_json::to_string(&encoding_type).unwrap());
    };
    if let Some(max_keys) = input.max_keys {
        http_request.add_param("max-keys", &serde_json::to_string(&max_keys).unwrap());
    };
    if let Some(prefix) = input.prefix {
        http_request.add_param("prefix", &serde_json::to_string(&prefix).unwrap());
    };
    if let Some(continuation_token) = input.continuation_token {
        http_request.add_param("continuation-token", &serde_json::to_string(&continuation_token).unwrap());
    };
    if let Some(fetch_owner) = input.fetch_owner {
        http_request.add_param("fetch-owner", &serde_json::to_string(&fetch_owner).unwrap());
    };
    if let Some(start_after) = input.start_after {
        http_request.add_param("start-after", &serde_json::to_string(&start_after).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: ListObjectsV2Output = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match ListObjectsV2OutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<ListObjectsV2Output, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListObjectsV2Output, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn list_parts(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListPartsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_parts(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __list_parts(input: ListPartsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "GET",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(max_parts) = input.max_parts {
        http_request.add_param("max-parts", &serde_json::to_string(&max_parts).unwrap());
    };
    if let Some(part_number_marker) = input.part_number_marker {
        http_request.add_param("part-number-marker", &serde_json::to_string(&part_number_marker).unwrap());
    };
    http_request.add_param("uploadId", &serde_json::to_string(&input.upload_id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: ListPartsOutput = Default::default();
                        output.abort_date = match response.headers().get("x-amz-abort-date") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.abort_rule_id = match response.headers().get("x-amz-abort-rule-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match ListPartsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<ListPartsOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<ListPartsOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_accelerate_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketAccelerateConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_accelerate_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_accelerate_configuration(input: PutBucketAccelerateConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?accelerate");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("AccelerateConfiguration", serde_xml_rs::to_string(&input.accelerate_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_acl(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_acl(input: PutBucketAclRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?acl");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(access_control_policy) = input.access_control_policy {
        body.insert("AccessControlPolicy", serde_json::to_string(&access_control_policy).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(acl) = input.acl {
        http_request.add_header("x-amz-acl", &serde_json::to_string(&acl).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        http_request.add_header("x-amz-grant-full-control", &serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        http_request.add_header("x-amz-grant-read", &serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        http_request.add_header("x-amz-grant-read-acp", &serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write) = input.grant_write {
        http_request.add_header("x-amz-grant-write", &serde_json::to_string(&grant_write).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        http_request.add_header("x-amz-grant-write-acp", &serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_analytics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketAnalyticsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_analytics_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_analytics_configuration(input: PutBucketAnalyticsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?analytics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("AnalyticsConfiguration", serde_xml_rs::to_string(&input.analytics_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_cors(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketCorsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_cors(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_cors(input: PutBucketCorsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?cors");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("CORSConfiguration", serde_xml_rs::to_string(&input.cors_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_encryption(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketEncryptionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_encryption(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_encryption(input: PutBucketEncryptionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?encryption");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("ServerSideEncryptionConfiguration", serde_xml_rs::to_string(&input.server_side_encryption_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_intelligent_tiering_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketIntelligentTieringConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_intelligent_tiering_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_intelligent_tiering_configuration(input: PutBucketIntelligentTieringConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?intelligent-tiering");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("IntelligentTieringConfiguration", serde_xml_rs::to_string(&input.intelligent_tiering_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_inventory_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketInventoryConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_inventory_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_inventory_configuration(input: PutBucketInventoryConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?inventory");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("InventoryConfiguration", serde_xml_rs::to_string(&input.inventory_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_lifecycle(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketLifecycleRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_lifecycle(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_lifecycle(input: PutBucketLifecycleRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?lifecycle");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(lifecycle_configuration) = input.lifecycle_configuration {
        body.insert("LifecycleConfiguration", serde_json::to_string(&lifecycle_configuration).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_lifecycle_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketLifecycleConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_lifecycle_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_lifecycle_configuration(input: PutBucketLifecycleConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?lifecycle");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(lifecycle_configuration) = input.lifecycle_configuration {
        body.insert("LifecycleConfiguration", serde_json::to_string(&lifecycle_configuration).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_logging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketLoggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_logging(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_logging(input: PutBucketLoggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?logging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("BucketLoggingStatus", serde_xml_rs::to_string(&input.bucket_logging_status).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_metrics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketMetricsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_metrics_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_metrics_configuration(input: PutBucketMetricsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?metrics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("MetricsConfiguration", serde_xml_rs::to_string(&input.metrics_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("id", &serde_json::to_string(&input.id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_notification(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketNotificationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_notification(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_notification(input: PutBucketNotificationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?notification");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("NotificationConfiguration", serde_xml_rs::to_string(&input.notification_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_notification_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketNotificationConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_notification_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_notification_configuration(input: PutBucketNotificationConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?notification");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("NotificationConfiguration", serde_xml_rs::to_string(&input.notification_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_ownership_controls(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketOwnershipControlsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_ownership_controls(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_ownership_controls(input: PutBucketOwnershipControlsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?ownershipControls");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("OwnershipControls", serde_xml_rs::to_string(&input.ownership_controls).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketPolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_policy(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_policy(input: PutBucketPolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?policy");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("Policy", serde_xml_rs::to_string(&input.policy).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(confirm_remove_self_bucket_access) = input.confirm_remove_self_bucket_access {
        http_request.add_header("x-amz-confirm-remove-self-bucket-access", &serde_json::to_string(&confirm_remove_self_bucket_access).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_replication(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketReplicationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_replication(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_replication(input: PutBucketReplicationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?replication");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("ReplicationConfiguration", serde_xml_rs::to_string(&input.replication_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(token) = input.token {
        http_request.add_header("x-amz-bucket-object-lock-token", &serde_json::to_string(&token).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_request_payment(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketRequestPaymentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_request_payment(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_request_payment(input: PutBucketRequestPaymentRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?requestPayment");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("RequestPaymentConfiguration", serde_xml_rs::to_string(&input.request_payment_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_tagging(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_tagging(input: PutBucketTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?tagging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("Tagging", serde_xml_rs::to_string(&input.tagging).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_versioning(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketVersioningRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_versioning(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_versioning(input: PutBucketVersioningRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?versioning");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("VersioningConfiguration", serde_xml_rs::to_string(&input.versioning_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(mfa) = input.mfa {
        http_request.add_header("x-amz-mfa", &serde_json::to_string(&mfa).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_bucket_website(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketWebsiteRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_website(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_bucket_website(input: PutBucketWebsiteRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?website");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("WebsiteConfiguration", serde_xml_rs::to_string(&input.website_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_object(input: PutObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    http_request.set_payload(input.body);
    
    if let Some(acl) = input.acl {
        http_request.add_header("x-amz-acl", &serde_json::to_string(&acl).unwrap());
    };
    if let Some(cache_control) = input.cache_control {
        http_request.add_header("Cache-Control", &serde_json::to_string(&cache_control).unwrap());
    };
    if let Some(content_disposition) = input.content_disposition {
        http_request.add_header("Content-Disposition", &serde_json::to_string(&content_disposition).unwrap());
    };
    if let Some(content_encoding) = input.content_encoding {
        http_request.add_header("Content-Encoding", &serde_json::to_string(&content_encoding).unwrap());
    };
    if let Some(content_language) = input.content_language {
        http_request.add_header("Content-Language", &serde_json::to_string(&content_language).unwrap());
    };
    if let Some(content_length) = input.content_length {
        http_request.add_header("Content-Length", &serde_json::to_string(&content_length).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(content_type) = input.content_type {
        http_request.add_header("Content-Type", &serde_json::to_string(&content_type).unwrap());
    };
    if let Some(expires) = input.expires {
        http_request.add_header("Expires", &serde_json::to_string(&expires).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        http_request.add_header("x-amz-grant-full-control", &serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        http_request.add_header("x-amz-grant-read", &serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        http_request.add_header("x-amz-grant-read-acp", &serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        http_request.add_header("x-amz-grant-write-acp", &serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(server_side_encryption) = input.server_side_encryption {
        http_request.add_header("x-amz-server-side-encryption", &serde_json::to_string(&server_side_encryption).unwrap());
    };
    if let Some(storage_class) = input.storage_class {
        http_request.add_header("x-amz-storage-class", &serde_json::to_string(&storage_class).unwrap());
    };
    if let Some(website_redirect_location) = input.website_redirect_location {
        http_request.add_header("x-amz-website-redirect-location", &serde_json::to_string(&website_redirect_location).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        http_request.add_header("x-amz-server-side-encryption-customer-algorithm", &serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        http_request.add_header("x-amz-server-side-encryption-customer-key", &serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        http_request.add_header("x-amz-server-side-encryption-customer-key-MD5", &serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(ssekms_key_id) = input.ssekms_key_id {
        http_request.add_header("x-amz-server-side-encryption-aws-kms-key-id", &serde_json::to_string(&ssekms_key_id).unwrap());
    };
    if let Some(ssekms_encryption_context) = input.ssekms_encryption_context {
        http_request.add_header("x-amz-server-side-encryption-context", &serde_json::to_string(&ssekms_encryption_context).unwrap());
    };
    if let Some(bucket_key_enabled) = input.bucket_key_enabled {
        http_request.add_header("x-amz-server-side-encryption-bucket-key-enabled", &serde_json::to_string(&bucket_key_enabled).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(tagging) = input.tagging {
        http_request.add_header("x-amz-tagging", &serde_json::to_string(&tagging).unwrap());
    };
    if let Some(object_lock_mode) = input.object_lock_mode {
        http_request.add_header("x-amz-object-lock-mode", &serde_json::to_string(&object_lock_mode).unwrap());
    };
    if let Some(object_lock_retain_until_date) = input.object_lock_retain_until_date {
        http_request.add_header("x-amz-object-lock-retain-until-date", &serde_json::to_string(&object_lock_retain_until_date).unwrap());
    };
    if let Some(object_lock_legal_hold_status) = input.object_lock_legal_hold_status {
        http_request.add_header("x-amz-object-lock-legal-hold", &serde_json::to_string(&object_lock_legal_hold_status).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectOutput = Default::default();
                        output.expiration = match response.headers().get("x-amz-expiration") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.e_tag = match response.headers().get("ETag") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.server_side_encryption = match response.headers().get("x-amz-server-side-encryption") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.version_id = match response.headers().get("x-amz-version-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_algorithm = match response.headers().get("x-amz-server-side-encryption-customer-algorithm") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_key_md5 = match response.headers().get("x-amz-server-side-encryption-customer-key-MD5") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.ssekms_key_id = match response.headers().get("x-amz-server-side-encryption-aws-kms-key-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.ssekms_encryption_context = match response.headers().get("x-amz-server-side-encryption-context") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.bucket_key_enabled = match response.headers().get("x-amz-server-side-encryption-bucket-key-enabled") {
                            Some(v) => Some(bool::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<PutObjectOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PutObjectOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_object_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_acl(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_object_acl(input: PutObjectAclRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?acl");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(access_control_policy) = input.access_control_policy {
        body.insert("AccessControlPolicy", serde_json::to_string(&access_control_policy).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(acl) = input.acl {
        http_request.add_header("x-amz-acl", &serde_json::to_string(&acl).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        http_request.add_header("x-amz-grant-full-control", &serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        http_request.add_header("x-amz-grant-read", &serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        http_request.add_header("x-amz-grant-read-acp", &serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write) = input.grant_write {
        http_request.add_header("x-amz-grant-write", &serde_json::to_string(&grant_write).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        http_request.add_header("x-amz-grant-write-acp", &serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectAclOutput = Default::default();
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<PutObjectAclOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PutObjectAclOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_object_legal_hold(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectLegalHoldRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_legal_hold(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_object_legal_hold(input: PutObjectLegalHoldRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?legal-hold");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(legal_hold) = input.legal_hold {
        body.insert("LegalHold", serde_json::to_string(&legal_hold).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectLegalHoldOutput = Default::default();
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<PutObjectLegalHoldOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PutObjectLegalHoldOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_object_lock_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectLockConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_lock_configuration(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_object_lock_configuration(input: PutObjectLockConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?object-lock");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(object_lock_configuration) = input.object_lock_configuration {
        body.insert("ObjectLockConfiguration", serde_json::to_string(&object_lock_configuration).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(token) = input.token {
        http_request.add_header("x-amz-bucket-object-lock-token", &serde_json::to_string(&token).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectLockConfigurationOutput = Default::default();
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<PutObjectLockConfigurationOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PutObjectLockConfigurationOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_object_retention(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectRetentionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_retention(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_object_retention(input: PutObjectRetentionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?retention");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(retention) = input.retention {
        body.insert("Retention", serde_json::to_string(&retention).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(bypass_governance_retention) = input.bypass_governance_retention {
        http_request.add_header("x-amz-bypass-governance-retention", &serde_json::to_string(&bypass_governance_retention).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectRetentionOutput = Default::default();
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<PutObjectRetentionOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PutObjectRetentionOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_object_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_tagging(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_object_tagging(input: PutObjectTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?tagging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("Tagging", serde_xml_rs::to_string(&input.tagging).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectTaggingOutput = Default::default();
                        output.version_id = match response.headers().get("x-amz-version-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<PutObjectTaggingOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<PutObjectTaggingOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn put_public_access_block(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutPublicAccessBlockRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_public_access_block(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __put_public_access_block(input: PutPublicAccessBlockRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?publicAccessBlock");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("PublicAccessBlockConfiguration", serde_xml_rs::to_string(&input.public_access_block_configuration).unwrap());
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: () = Default::default();


                        serde_json::to_vec(&Result::<(), guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<(), guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn restore_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: RestoreObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __restore_object(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __restore_object(input: RestoreObjectRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?restore");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if let Some(restore_request) = input.restore_request {
        body.insert("RestoreRequest", serde_json::to_string(&restore_request).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        http_request.add_param("versionId", &serde_json::to_string(&version_id).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: RestoreObjectOutput = Default::default();
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.restore_output_path = match response.headers().get("x-amz-restore-output-path") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<RestoreObjectOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<RestoreObjectOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn select_object_content(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: SelectObjectContentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __select_object_content(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __select_object_content(input: SelectObjectContentRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?select&select-type=2");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "POST",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    body.insert("Expression", serde_xml_rs::to_string(&input.expression).unwrap());
    body.insert("ExpressionType", serde_xml_rs::to_string(&input.expression_type).unwrap());
    if let Some(request_progress) = input.request_progress {
        body.insert("RequestProgress", serde_json::to_string(&request_progress).unwrap());
    };
    body.insert("InputSerialization", serde_xml_rs::to_string(&input.input_serialization).unwrap());
    body.insert("OutputSerialization", serde_xml_rs::to_string(&input.output_serialization).unwrap());
    if let Some(scan_range) = input.scan_range {
        body.insert("ScanRange", serde_json::to_string(&scan_range).unwrap());
    };
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        http_request.add_header("x-amz-server-side-encryption-customer-algorithm", &serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        http_request.add_header("x-amz-server-side-encryption-customer-key", &serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        http_request.add_header("x-amz-server-side-encryption-customer-key-MD5", &serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: SelectObjectContentOutput = Default::default();

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match SelectObjectContentOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<SelectObjectContentOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<SelectObjectContentOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn upload_part(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UploadPartRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __upload_part(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __upload_part(input: UploadPartRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    http_request.set_payload(input.body);
    
    if let Some(content_length) = input.content_length {
        http_request.add_header("Content-Length", &serde_json::to_string(&content_length).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        http_request.add_header("Content-MD5", &serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        http_request.add_header("x-amz-server-side-encryption-customer-algorithm", &serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        http_request.add_header("x-amz-server-side-encryption-customer-key", &serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        http_request.add_header("x-amz-server-side-encryption-customer-key-MD5", &serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    http_request.add_param("partNumber", &serde_json::to_string(&input.part_number).unwrap());
    http_request.add_param("uploadId", &serde_json::to_string(&input.upload_id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: UploadPartOutput = Default::default();
                        output.server_side_encryption = match response.headers().get("x-amz-server-side-encryption") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.e_tag = match response.headers().get("ETag") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_algorithm = match response.headers().get("x-amz-server-side-encryption-customer-algorithm") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_key_md5 = match response.headers().get("x-amz-server-side-encryption-customer-key-MD5") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.ssekms_key_id = match response.headers().get("x-amz-server-side-encryption-aws-kms-key-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.bucket_key_enabled = match response.headers().get("x-amz-server-side-encryption-bucket-key-enabled") {
                            Some(v) => Some(bool::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };


                        serde_json::to_vec(&Result::<UploadPartOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UploadPartOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

#[allow(dead_code)]
pub fn upload_part_copy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UploadPartCopyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __upload_part_copy(deserialized)
}
#[allow(unused_assignments, unused_mut, unused_variables)]
fn __upload_part_copy(input: UploadPartCopyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut http_request = SignedRequest::new(
        "PUT",
        "s3",
        &Region::from_str(&std::env::var("AWS_REGION").unwrap_or(String::from("us-east-1")))
            .unwrap_or(Region::UsEast1),
        &path,
    );
    
    let mut body: std::collections::HashMap<&str, String> = Default::default();
    if body.len() != 0 {
        http_request.set_payload(Some(serde_xml_rs::to_string(&body).unwrap()));
    } else {
        http_request.set_payload(Option::<String>::None);
    }
    
    http_request.add_header("x-amz-copy-source", &serde_json::to_string(&input.copy_source).unwrap());
    if let Some(copy_source_if_match) = input.copy_source_if_match {
        http_request.add_header("x-amz-copy-source-if-match", &serde_json::to_string(&copy_source_if_match).unwrap());
    };
    if let Some(copy_source_if_modified_since) = input.copy_source_if_modified_since {
        http_request.add_header("x-amz-copy-source-if-modified-since", &serde_json::to_string(&copy_source_if_modified_since).unwrap());
    };
    if let Some(copy_source_if_none_match) = input.copy_source_if_none_match {
        http_request.add_header("x-amz-copy-source-if-none-match", &serde_json::to_string(&copy_source_if_none_match).unwrap());
    };
    if let Some(copy_source_if_unmodified_since) = input.copy_source_if_unmodified_since {
        http_request.add_header("x-amz-copy-source-if-unmodified-since", &serde_json::to_string(&copy_source_if_unmodified_since).unwrap());
    };
    if let Some(copy_source_range) = input.copy_source_range {
        http_request.add_header("x-amz-copy-source-range", &serde_json::to_string(&copy_source_range).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        http_request.add_header("x-amz-server-side-encryption-customer-algorithm", &serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        http_request.add_header("x-amz-server-side-encryption-customer-key", &serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        http_request.add_header("x-amz-server-side-encryption-customer-key-MD5", &serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(copy_source_sse_customer_algorithm) = input.copy_source_sse_customer_algorithm {
        http_request.add_header("x-amz-copy-source-server-side-encryption-customer-algorithm", &serde_json::to_string(&copy_source_sse_customer_algorithm).unwrap());
    };
    if let Some(copy_source_sse_customer_key) = input.copy_source_sse_customer_key {
        http_request.add_header("x-amz-copy-source-server-side-encryption-customer-key", &serde_json::to_string(&copy_source_sse_customer_key).unwrap());
    };
    if let Some(copy_source_sse_customer_key_md5) = input.copy_source_sse_customer_key_md5 {
        http_request.add_header("x-amz-copy-source-server-side-encryption-customer-key-MD5", &serde_json::to_string(&copy_source_sse_customer_key_md5).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        http_request.add_header("x-amz-request-payer", &serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        http_request.add_header("x-amz-expected-bucket-owner", &serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(expected_source_bucket_owner) = input.expected_source_bucket_owner {
        http_request.add_header("x-amz-source-expected-bucket-owner", &serde_json::to_string(&expected_source_bucket_owner).unwrap());
    };
    http_request.add_param("partNumber", &serde_json::to_string(&input.part_number).unwrap());
    http_request.add_param("uploadId", &serde_json::to_string(&input.upload_id).unwrap());

    Box::pin(async move {
        match crate::CLIENT.call(http_request).await {
            Ok(response) => {
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        let mut output: UploadPartCopyOutput = Default::default();
                        output.copy_source_version_id = match response.headers().get("x-amz-copy-source-version-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.server_side_encryption = match response.headers().get("x-amz-server-side-encryption") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_algorithm = match response.headers().get("x-amz-server-side-encryption-customer-algorithm") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.sse_customer_key_md5 = match response.headers().get("x-amz-server-side-encryption-customer-key-MD5") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.ssekms_key_id = match response.headers().get("x-amz-server-side-encryption-aws-kms-key-id") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.bucket_key_enabled = match response.headers().get("x-amz-server-side-encryption-bucket-key-enabled") {
                            Some(v) => Some(bool::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };
                        output.request_charged = match response.headers().get("x-amz-request-charged") {
                            Some(v) => Some(String::from_str(v.to_str().unwrap()).unwrap()),
                            None => None,
                        };

                        let body = &*hyper::body::to_bytes(response.into_body()).await.unwrap();
                        let body = String::from(std::str::from_utf8(body).unwrap());
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match UploadPartCopyOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };

                        serde_json::to_vec(&Result::<UploadPartCopyOutput, guest::Error>::Ok(output)).unwrap()
                    }
                    status => {
                        serde_json::to_vec(&Result::<UploadPartCopyOutput, guest::Error>::Err(guest::Error {
                            why: String::from(status.canonical_reason().unwrap()),
                        }))
                        .unwrap()
                    }
                }
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

