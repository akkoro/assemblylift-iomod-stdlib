// This file is generated!
// See https://github.com/akkoro/asml-aws-codegen

mod client;

use std::str::FromStr;

use assemblylift_core_iomod::iomod;
use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use hyper::StatusCode;

use xml;
use xml::reader::{EventReader, ParserConfig};

use guest::xml_util;
use guest::xml_util::util::Next;
use guest::structs::*;

static CLIENT: Lazy<client::Client> = Lazy::new(|| {
    use std::env;
    let mut c = client::Client::new(
        String::from("s3"), 
        env::var("AWS_REGION").unwrap_or(String::from("us-east-1")),
    );
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

pub fn abort_multipart_upload(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: AbortMultipartUploadRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __abort_multipart_upload(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "uploadId", serde_json::to_string(&input.upload_id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "uploadId", serde_json::to_string(&input.upload_id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: AbortMultipartUploadOutput = Default::default();
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn complete_multipart_upload(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CompleteMultipartUploadRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __complete_multipart_upload(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(multipart_upload) = input.multipart_upload {
        body.insert("MultipartUpload", serde_json::to_string(&multipart_upload).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "uploadId", serde_json::to_string(&input.upload_id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "uploadId", serde_json::to_string(&input.upload_id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("POST", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: CompleteMultipartUploadOutput = Default::default();
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match CompleteMultipartUploadOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };
                        output.expiration = Some(String::from_str(response.headers()["x-amz-expiration"].to_str().unwrap()).unwrap());
                        output.server_side_encryption = Some(String::from_str(response.headers()["x-amz-server-side-encryption"].to_str().unwrap()).unwrap());
                        output.version_id = Some(String::from_str(response.headers()["x-amz-version-id"].to_str().unwrap()).unwrap());
                        output.ssekms_key_id = Some(String::from_str(response.headers()["x-amz-server-side-encryption-aws-kms-key-id"].to_str().unwrap()).unwrap());
                        output.bucket_key_enabled = Some(bool::from_str(response.headers()["x-amz-server-side-encryption-bucket-key-enabled"].to_str().unwrap()).unwrap());
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn copy_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CopyObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __copy_object(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(acl) = input.acl {
        headers.insert("x-amz-acl", serde_json::to_string(&acl).unwrap());
    };
    if let Some(cache_control) = input.cache_control {
        headers.insert("Cache-Control", serde_json::to_string(&cache_control).unwrap());
    };
    if let Some(content_disposition) = input.content_disposition {
        headers.insert("Content-Disposition", serde_json::to_string(&content_disposition).unwrap());
    };
    if let Some(content_encoding) = input.content_encoding {
        headers.insert("Content-Encoding", serde_json::to_string(&content_encoding).unwrap());
    };
    if let Some(content_language) = input.content_language {
        headers.insert("Content-Language", serde_json::to_string(&content_language).unwrap());
    };
    if let Some(content_type) = input.content_type {
        headers.insert("Content-Type", serde_json::to_string(&content_type).unwrap());
    };
    headers.insert("x-amz-copy-source", serde_json::to_string(&input.copy_source).unwrap());
    if let Some(copy_source_if_match) = input.copy_source_if_match {
        headers.insert("x-amz-copy-source-if-match", serde_json::to_string(&copy_source_if_match).unwrap());
    };
    if let Some(copy_source_if_modified_since) = input.copy_source_if_modified_since {
        headers.insert("x-amz-copy-source-if-modified-since", serde_json::to_string(&copy_source_if_modified_since).unwrap());
    };
    if let Some(copy_source_if_none_match) = input.copy_source_if_none_match {
        headers.insert("x-amz-copy-source-if-none-match", serde_json::to_string(&copy_source_if_none_match).unwrap());
    };
    if let Some(copy_source_if_unmodified_since) = input.copy_source_if_unmodified_since {
        headers.insert("x-amz-copy-source-if-unmodified-since", serde_json::to_string(&copy_source_if_unmodified_since).unwrap());
    };
    if let Some(expires) = input.expires {
        headers.insert("Expires", serde_json::to_string(&expires).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        headers.insert("x-amz-grant-full-control", serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        headers.insert("x-amz-grant-read", serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        headers.insert("x-amz-grant-read-acp", serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        headers.insert("x-amz-grant-write-acp", serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(metadata_directive) = input.metadata_directive {
        headers.insert("x-amz-metadata-directive", serde_json::to_string(&metadata_directive).unwrap());
    };
    if let Some(tagging_directive) = input.tagging_directive {
        headers.insert("x-amz-tagging-directive", serde_json::to_string(&tagging_directive).unwrap());
    };
    if let Some(server_side_encryption) = input.server_side_encryption {
        headers.insert("x-amz-server-side-encryption", serde_json::to_string(&server_side_encryption).unwrap());
    };
    if let Some(storage_class) = input.storage_class {
        headers.insert("x-amz-storage-class", serde_json::to_string(&storage_class).unwrap());
    };
    if let Some(website_redirect_location) = input.website_redirect_location {
        headers.insert("x-amz-website-redirect-location", serde_json::to_string(&website_redirect_location).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        headers.insert("x-amz-server-side-encryption-customer-algorithm", serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        headers.insert("x-amz-server-side-encryption-customer-key", serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        headers.insert("x-amz-server-side-encryption-customer-key-MD5", serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(ssekms_key_id) = input.ssekms_key_id {
        headers.insert("x-amz-server-side-encryption-aws-kms-key-id", serde_json::to_string(&ssekms_key_id).unwrap());
    };
    if let Some(ssekms_encryption_context) = input.ssekms_encryption_context {
        headers.insert("x-amz-server-side-encryption-context", serde_json::to_string(&ssekms_encryption_context).unwrap());
    };
    if let Some(bucket_key_enabled) = input.bucket_key_enabled {
        headers.insert("x-amz-server-side-encryption-bucket-key-enabled", serde_json::to_string(&bucket_key_enabled).unwrap());
    };
    if let Some(copy_source_sse_customer_algorithm) = input.copy_source_sse_customer_algorithm {
        headers.insert("x-amz-copy-source-server-side-encryption-customer-algorithm", serde_json::to_string(&copy_source_sse_customer_algorithm).unwrap());
    };
    if let Some(copy_source_sse_customer_key) = input.copy_source_sse_customer_key {
        headers.insert("x-amz-copy-source-server-side-encryption-customer-key", serde_json::to_string(&copy_source_sse_customer_key).unwrap());
    };
    if let Some(copy_source_sse_customer_key_md5) = input.copy_source_sse_customer_key_md5 {
        headers.insert("x-amz-copy-source-server-side-encryption-customer-key-MD5", serde_json::to_string(&copy_source_sse_customer_key_md5).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(tagging) = input.tagging {
        headers.insert("x-amz-tagging", serde_json::to_string(&tagging).unwrap());
    };
    if let Some(object_lock_mode) = input.object_lock_mode {
        headers.insert("x-amz-object-lock-mode", serde_json::to_string(&object_lock_mode).unwrap());
    };
    if let Some(object_lock_retain_until_date) = input.object_lock_retain_until_date {
        headers.insert("x-amz-object-lock-retain-until-date", serde_json::to_string(&object_lock_retain_until_date).unwrap());
    };
    if let Some(object_lock_legal_hold_status) = input.object_lock_legal_hold_status {
        headers.insert("x-amz-object-lock-legal-hold", serde_json::to_string(&object_lock_legal_hold_status).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(expected_source_bucket_owner) = input.expected_source_bucket_owner {
        headers.insert("x-amz-source-expected-bucket-owner", serde_json::to_string(&expected_source_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: CopyObjectOutput = Default::default();
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match CopyObjectOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };
                        output.expiration = Some(String::from_str(response.headers()["x-amz-expiration"].to_str().unwrap()).unwrap());
                        output.copy_source_version_id = Some(String::from_str(response.headers()["x-amz-copy-source-version-id"].to_str().unwrap()).unwrap());
                        output.version_id = Some(String::from_str(response.headers()["x-amz-version-id"].to_str().unwrap()).unwrap());
                        output.server_side_encryption = Some(String::from_str(response.headers()["x-amz-server-side-encryption"].to_str().unwrap()).unwrap());
                        output.sse_customer_algorithm = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-algorithm"].to_str().unwrap()).unwrap());
                        output.sse_customer_key_md5 = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-key-MD5"].to_str().unwrap()).unwrap());
                        output.ssekms_key_id = Some(String::from_str(response.headers()["x-amz-server-side-encryption-aws-kms-key-id"].to_str().unwrap()).unwrap());
                        output.ssekms_encryption_context = Some(String::from_str(response.headers()["x-amz-server-side-encryption-context"].to_str().unwrap()).unwrap());
                        output.bucket_key_enabled = Some(bool::from_str(response.headers()["x-amz-server-side-encryption-bucket-key-enabled"].to_str().unwrap()).unwrap());
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn create_bucket(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateBucketRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_bucket(deserialized)
}

fn __create_bucket(input: CreateBucketRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(create_bucket_configuration) = input.create_bucket_configuration {
        body.insert("CreateBucketConfiguration", serde_json::to_string(&create_bucket_configuration).unwrap());
    };
    if let Some(acl) = input.acl {
        headers.insert("x-amz-acl", serde_json::to_string(&acl).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        headers.insert("x-amz-grant-full-control", serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        headers.insert("x-amz-grant-read", serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        headers.insert("x-amz-grant-read-acp", serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write) = input.grant_write {
        headers.insert("x-amz-grant-write", serde_json::to_string(&grant_write).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        headers.insert("x-amz-grant-write-acp", serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(object_lock_enabled_for_bucket) = input.object_lock_enabled_for_bucket {
        headers.insert("x-amz-bucket-object-lock-enabled", serde_json::to_string(&object_lock_enabled_for_bucket).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: CreateBucketOutput = Default::default();
                        output.location = Some(String::from_str(response.headers()["Location"].to_str().unwrap()).unwrap());
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

pub fn create_multipart_upload(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: CreateMultipartUploadRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __create_multipart_upload(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(acl) = input.acl {
        headers.insert("x-amz-acl", serde_json::to_string(&acl).unwrap());
    };
    if let Some(cache_control) = input.cache_control {
        headers.insert("Cache-Control", serde_json::to_string(&cache_control).unwrap());
    };
    if let Some(content_disposition) = input.content_disposition {
        headers.insert("Content-Disposition", serde_json::to_string(&content_disposition).unwrap());
    };
    if let Some(content_encoding) = input.content_encoding {
        headers.insert("Content-Encoding", serde_json::to_string(&content_encoding).unwrap());
    };
    if let Some(content_language) = input.content_language {
        headers.insert("Content-Language", serde_json::to_string(&content_language).unwrap());
    };
    if let Some(content_type) = input.content_type {
        headers.insert("Content-Type", serde_json::to_string(&content_type).unwrap());
    };
    if let Some(expires) = input.expires {
        headers.insert("Expires", serde_json::to_string(&expires).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        headers.insert("x-amz-grant-full-control", serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        headers.insert("x-amz-grant-read", serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        headers.insert("x-amz-grant-read-acp", serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        headers.insert("x-amz-grant-write-acp", serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(server_side_encryption) = input.server_side_encryption {
        headers.insert("x-amz-server-side-encryption", serde_json::to_string(&server_side_encryption).unwrap());
    };
    if let Some(storage_class) = input.storage_class {
        headers.insert("x-amz-storage-class", serde_json::to_string(&storage_class).unwrap());
    };
    if let Some(website_redirect_location) = input.website_redirect_location {
        headers.insert("x-amz-website-redirect-location", serde_json::to_string(&website_redirect_location).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        headers.insert("x-amz-server-side-encryption-customer-algorithm", serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        headers.insert("x-amz-server-side-encryption-customer-key", serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        headers.insert("x-amz-server-side-encryption-customer-key-MD5", serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(ssekms_key_id) = input.ssekms_key_id {
        headers.insert("x-amz-server-side-encryption-aws-kms-key-id", serde_json::to_string(&ssekms_key_id).unwrap());
    };
    if let Some(ssekms_encryption_context) = input.ssekms_encryption_context {
        headers.insert("x-amz-server-side-encryption-context", serde_json::to_string(&ssekms_encryption_context).unwrap());
    };
    if let Some(bucket_key_enabled) = input.bucket_key_enabled {
        headers.insert("x-amz-server-side-encryption-bucket-key-enabled", serde_json::to_string(&bucket_key_enabled).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(tagging) = input.tagging {
        headers.insert("x-amz-tagging", serde_json::to_string(&tagging).unwrap());
    };
    if let Some(object_lock_mode) = input.object_lock_mode {
        headers.insert("x-amz-object-lock-mode", serde_json::to_string(&object_lock_mode).unwrap());
    };
    if let Some(object_lock_retain_until_date) = input.object_lock_retain_until_date {
        headers.insert("x-amz-object-lock-retain-until-date", serde_json::to_string(&object_lock_retain_until_date).unwrap());
    };
    if let Some(object_lock_legal_hold_status) = input.object_lock_legal_hold_status {
        headers.insert("x-amz-object-lock-legal-hold", serde_json::to_string(&object_lock_legal_hold_status).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("POST", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: CreateMultipartUploadOutput = Default::default();
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match CreateMultipartUploadOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };
                        output.abort_date = Some(String::from_str(response.headers()["x-amz-abort-date"].to_str().unwrap()).unwrap());
                        output.abort_rule_id = Some(String::from_str(response.headers()["x-amz-abort-rule-id"].to_str().unwrap()).unwrap());
                        output.server_side_encryption = Some(String::from_str(response.headers()["x-amz-server-side-encryption"].to_str().unwrap()).unwrap());
                        output.sse_customer_algorithm = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-algorithm"].to_str().unwrap()).unwrap());
                        output.sse_customer_key_md5 = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-key-MD5"].to_str().unwrap()).unwrap());
                        output.ssekms_key_id = Some(String::from_str(response.headers()["x-amz-server-side-encryption-aws-kms-key-id"].to_str().unwrap()).unwrap());
                        output.ssekms_encryption_context = Some(String::from_str(response.headers()["x-amz-server-side-encryption-context"].to_str().unwrap()).unwrap());
                        output.bucket_key_enabled = Some(bool::from_str(response.headers()["x-amz-server-side-encryption-bucket-key-enabled"].to_str().unwrap()).unwrap());
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn delete_bucket(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket(deserialized)
}

fn __delete_bucket(input: DeleteBucketRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_analytics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketAnalyticsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_analytics_configuration(deserialized)
}

fn __delete_bucket_analytics_configuration(input: DeleteBucketAnalyticsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?analytics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_cors(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketCorsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_cors(deserialized)
}

fn __delete_bucket_cors(input: DeleteBucketCorsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?cors");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_encryption(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketEncryptionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_encryption(deserialized)
}

fn __delete_bucket_encryption(input: DeleteBucketEncryptionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?encryption");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_intelligent_tiering_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketIntelligentTieringConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_intelligent_tiering_configuration(deserialized)
}

fn __delete_bucket_intelligent_tiering_configuration(input: DeleteBucketIntelligentTieringConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?intelligent-tiering");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_inventory_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketInventoryConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_inventory_configuration(deserialized)
}

fn __delete_bucket_inventory_configuration(input: DeleteBucketInventoryConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?inventory");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_lifecycle(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketLifecycleRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_lifecycle(deserialized)
}

fn __delete_bucket_lifecycle(input: DeleteBucketLifecycleRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?lifecycle");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_metrics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketMetricsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_metrics_configuration(deserialized)
}

fn __delete_bucket_metrics_configuration(input: DeleteBucketMetricsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?metrics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_ownership_controls(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketOwnershipControlsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_ownership_controls(deserialized)
}

fn __delete_bucket_ownership_controls(input: DeleteBucketOwnershipControlsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?ownershipControls");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketPolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_policy(deserialized)
}

fn __delete_bucket_policy(input: DeleteBucketPolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?policy");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_replication(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketReplicationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_replication(deserialized)
}

fn __delete_bucket_replication(input: DeleteBucketReplicationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?replication");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_tagging(deserialized)
}

fn __delete_bucket_tagging(input: DeleteBucketTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?tagging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_bucket_website(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteBucketWebsiteRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_bucket_website(deserialized)
}

fn __delete_bucket_website(input: DeleteBucketWebsiteRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?website");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn delete_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_object(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(mfa) = input.mfa {
        headers.insert("x-amz-mfa", serde_json::to_string(&mfa).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(bypass_governance_retention) = input.bypass_governance_retention {
        headers.insert("x-amz-bypass-governance-retention", serde_json::to_string(&bypass_governance_retention).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: DeleteObjectOutput = Default::default();
                        output.delete_marker = Some(bool::from_str(response.headers()["x-amz-delete-marker"].to_str().unwrap()).unwrap());
                        output.version_id = Some(String::from_str(response.headers()["x-amz-version-id"].to_str().unwrap()).unwrap());
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn delete_object_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteObjectTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_object_tagging(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: DeleteObjectTaggingOutput = Default::default();
                        output.version_id = Some(String::from_str(response.headers()["x-amz-version-id"].to_str().unwrap()).unwrap());
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

pub fn delete_objects(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeleteObjectsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_objects(deserialized)
}

fn __delete_objects(input: DeleteObjectsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?delete");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("Delete", serde_xml_rs::to_string(&input.delete).unwrap());
    if let Some(mfa) = input.mfa {
        headers.insert("x-amz-mfa", serde_json::to_string(&mfa).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(bypass_governance_retention) = input.bypass_governance_retention {
        headers.insert("x-amz-bypass-governance-retention", serde_json::to_string(&bypass_governance_retention).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("POST", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: DeleteObjectsOutput = Default::default();
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match DeleteObjectsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn delete_public_access_block(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: DeletePublicAccessBlockRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __delete_public_access_block(deserialized)
}

fn __delete_public_access_block(input: DeletePublicAccessBlockRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?publicAccessBlock");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("DELETE", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn get_bucket_accelerate_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketAccelerateConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_accelerate_configuration(deserialized)
}

fn __get_bucket_accelerate_configuration(input: GetBucketAccelerateConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?accelerate");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketAccelerateConfigurationOutput = Default::default();
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

pub fn get_bucket_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_acl(deserialized)
}

fn __get_bucket_acl(input: GetBucketAclRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?acl");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketAclOutput = Default::default();
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

pub fn get_bucket_analytics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketAnalyticsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_analytics_configuration(deserialized)
}

fn __get_bucket_analytics_configuration(input: GetBucketAnalyticsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?analytics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketAnalyticsConfigurationOutput = Default::default();
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

pub fn get_bucket_cors(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketCorsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_cors(deserialized)
}

fn __get_bucket_cors(input: GetBucketCorsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?cors");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketCorsOutput = Default::default();
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

pub fn get_bucket_encryption(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketEncryptionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_encryption(deserialized)
}

fn __get_bucket_encryption(input: GetBucketEncryptionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?encryption");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketEncryptionOutput = Default::default();
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

pub fn get_bucket_intelligent_tiering_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketIntelligentTieringConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_intelligent_tiering_configuration(deserialized)
}

fn __get_bucket_intelligent_tiering_configuration(input: GetBucketIntelligentTieringConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?intelligent-tiering");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketIntelligentTieringConfigurationOutput = Default::default();
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

pub fn get_bucket_inventory_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketInventoryConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_inventory_configuration(deserialized)
}

fn __get_bucket_inventory_configuration(input: GetBucketInventoryConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?inventory");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketInventoryConfigurationOutput = Default::default();
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

pub fn get_bucket_lifecycle(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLifecycleRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_lifecycle(deserialized)
}

fn __get_bucket_lifecycle(input: GetBucketLifecycleRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?lifecycle");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketLifecycleOutput = Default::default();
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

pub fn get_bucket_lifecycle_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLifecycleConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_lifecycle_configuration(deserialized)
}

fn __get_bucket_lifecycle_configuration(input: GetBucketLifecycleConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?lifecycle");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketLifecycleConfigurationOutput = Default::default();
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

pub fn get_bucket_location(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLocationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_location(deserialized)
}

fn __get_bucket_location(input: GetBucketLocationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?location");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketLocationOutput = Default::default();
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

pub fn get_bucket_logging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketLoggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_logging(deserialized)
}

fn __get_bucket_logging(input: GetBucketLoggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?logging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketLoggingOutput = Default::default();
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

pub fn get_bucket_metrics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketMetricsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_metrics_configuration(deserialized)
}

fn __get_bucket_metrics_configuration(input: GetBucketMetricsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?metrics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketMetricsConfigurationOutput = Default::default();
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

pub fn get_bucket_notification(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketNotificationConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_notification(deserialized)
}

fn __get_bucket_notification(input: GetBucketNotificationConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?notification");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: NotificationConfigurationDeprecated = Default::default();
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

pub fn get_bucket_notification_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketNotificationConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_notification_configuration(deserialized)
}

fn __get_bucket_notification_configuration(input: GetBucketNotificationConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?notification");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: NotificationConfiguration = Default::default();
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

pub fn get_bucket_ownership_controls(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketOwnershipControlsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_ownership_controls(deserialized)
}

fn __get_bucket_ownership_controls(input: GetBucketOwnershipControlsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?ownershipControls");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketOwnershipControlsOutput = Default::default();
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

pub fn get_bucket_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketPolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_policy(deserialized)
}

fn __get_bucket_policy(input: GetBucketPolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?policy");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketPolicyOutput = Default::default();
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

pub fn get_bucket_policy_status(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketPolicyStatusRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_policy_status(deserialized)
}

fn __get_bucket_policy_status(input: GetBucketPolicyStatusRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?policyStatus");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketPolicyStatusOutput = Default::default();
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

pub fn get_bucket_replication(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketReplicationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_replication(deserialized)
}

fn __get_bucket_replication(input: GetBucketReplicationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?replication");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketReplicationOutput = Default::default();
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

pub fn get_bucket_request_payment(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketRequestPaymentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_request_payment(deserialized)
}

fn __get_bucket_request_payment(input: GetBucketRequestPaymentRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?requestPayment");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketRequestPaymentOutput = Default::default();
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

pub fn get_bucket_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_tagging(deserialized)
}

fn __get_bucket_tagging(input: GetBucketTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?tagging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketTaggingOutput = Default::default();
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

pub fn get_bucket_versioning(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketVersioningRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_versioning(deserialized)
}

fn __get_bucket_versioning(input: GetBucketVersioningRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?versioning");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketVersioningOutput = Default::default();
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

pub fn get_bucket_website(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetBucketWebsiteRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_bucket_website(deserialized)
}

fn __get_bucket_website(input: GetBucketWebsiteRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?website");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetBucketWebsiteOutput = Default::default();
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

pub fn get_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(if_match) = input.if_match {
        headers.insert("If-Match", serde_json::to_string(&if_match).unwrap());
    };
    if let Some(if_modified_since) = input.if_modified_since {
        headers.insert("If-Modified-Since", serde_json::to_string(&if_modified_since).unwrap());
    };
    if let Some(if_none_match) = input.if_none_match {
        headers.insert("If-None-Match", serde_json::to_string(&if_none_match).unwrap());
    };
    if let Some(if_unmodified_since) = input.if_unmodified_since {
        headers.insert("If-Unmodified-Since", serde_json::to_string(&if_unmodified_since).unwrap());
    };
    if let Some(range) = input.range {
        headers.insert("Range", serde_json::to_string(&range).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        headers.insert("x-amz-server-side-encryption-customer-algorithm", serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        headers.insert("x-amz-server-side-encryption-customer-key", serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        headers.insert("x-amz-server-side-encryption-customer-key-MD5", serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(response_cache_control) = input.response_cache_control {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "response-cache-control", serde_json::to_string(&response_cache_control).unwrap())),
            None => path.push_str(&format!("?{}={}", "response-cache-control", serde_json::to_string(&response_cache_control).unwrap())),
        }
    };
    if let Some(response_content_disposition) = input.response_content_disposition {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "response-content-disposition", serde_json::to_string(&response_content_disposition).unwrap())),
            None => path.push_str(&format!("?{}={}", "response-content-disposition", serde_json::to_string(&response_content_disposition).unwrap())),
        }
    };
    if let Some(response_content_encoding) = input.response_content_encoding {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "response-content-encoding", serde_json::to_string(&response_content_encoding).unwrap())),
            None => path.push_str(&format!("?{}={}", "response-content-encoding", serde_json::to_string(&response_content_encoding).unwrap())),
        }
    };
    if let Some(response_content_language) = input.response_content_language {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "response-content-language", serde_json::to_string(&response_content_language).unwrap())),
            None => path.push_str(&format!("?{}={}", "response-content-language", serde_json::to_string(&response_content_language).unwrap())),
        }
    };
    if let Some(response_content_type) = input.response_content_type {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "response-content-type", serde_json::to_string(&response_content_type).unwrap())),
            None => path.push_str(&format!("?{}={}", "response-content-type", serde_json::to_string(&response_content_type).unwrap())),
        }
    };
    if let Some(response_expires) = input.response_expires {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "response-expires", serde_json::to_string(&response_expires).unwrap())),
            None => path.push_str(&format!("?{}={}", "response-expires", serde_json::to_string(&response_expires).unwrap())),
        }
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };
    if let Some(part_number) = input.part_number {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "partNumber", serde_json::to_string(&part_number).unwrap())),
            None => path.push_str(&format!("?{}={}", "partNumber", serde_json::to_string(&part_number).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectOutput = Default::default();
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetObjectOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };
                        output.delete_marker = Some(bool::from_str(response.headers()["x-amz-delete-marker"].to_str().unwrap()).unwrap());
                        output.accept_ranges = Some(String::from_str(response.headers()["accept-ranges"].to_str().unwrap()).unwrap());
                        output.expiration = Some(String::from_str(response.headers()["x-amz-expiration"].to_str().unwrap()).unwrap());
                        output.restore = Some(String::from_str(response.headers()["x-amz-restore"].to_str().unwrap()).unwrap());
                        output.last_modified = Some(String::from_str(response.headers()["Last-Modified"].to_str().unwrap()).unwrap());
                        output.content_length = Some(u64::from_str(response.headers()["Content-Length"].to_str().unwrap()).unwrap());
                        output.e_tag = Some(String::from_str(response.headers()["ETag"].to_str().unwrap()).unwrap());
                        output.missing_meta = Some(i64::from_str(response.headers()["x-amz-missing-meta"].to_str().unwrap()).unwrap());
                        output.version_id = Some(String::from_str(response.headers()["x-amz-version-id"].to_str().unwrap()).unwrap());
                        output.cache_control = Some(String::from_str(response.headers()["Cache-Control"].to_str().unwrap()).unwrap());
                        output.content_disposition = Some(String::from_str(response.headers()["Content-Disposition"].to_str().unwrap()).unwrap());
                        output.content_encoding = Some(String::from_str(response.headers()["Content-Encoding"].to_str().unwrap()).unwrap());
                        output.content_language = Some(String::from_str(response.headers()["Content-Language"].to_str().unwrap()).unwrap());
                        output.content_range = Some(String::from_str(response.headers()["Content-Range"].to_str().unwrap()).unwrap());
                        output.content_type = Some(String::from_str(response.headers()["Content-Type"].to_str().unwrap()).unwrap());
                        output.expires = Some(String::from_str(response.headers()["Expires"].to_str().unwrap()).unwrap());
                        output.website_redirect_location = Some(String::from_str(response.headers()["x-amz-website-redirect-location"].to_str().unwrap()).unwrap());
                        output.server_side_encryption = Some(String::from_str(response.headers()["x-amz-server-side-encryption"].to_str().unwrap()).unwrap());
                        output.sse_customer_algorithm = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-algorithm"].to_str().unwrap()).unwrap());
                        output.sse_customer_key_md5 = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-key-MD5"].to_str().unwrap()).unwrap());
                        output.ssekms_key_id = Some(String::from_str(response.headers()["x-amz-server-side-encryption-aws-kms-key-id"].to_str().unwrap()).unwrap());
                        output.bucket_key_enabled = Some(bool::from_str(response.headers()["x-amz-server-side-encryption-bucket-key-enabled"].to_str().unwrap()).unwrap());
                        output.storage_class = Some(String::from_str(response.headers()["x-amz-storage-class"].to_str().unwrap()).unwrap());
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
                        output.replication_status = Some(String::from_str(response.headers()["x-amz-replication-status"].to_str().unwrap()).unwrap());
                        output.parts_count = Some(i64::from_str(response.headers()["x-amz-mp-parts-count"].to_str().unwrap()).unwrap());
                        output.tag_count = Some(i64::from_str(response.headers()["x-amz-tagging-count"].to_str().unwrap()).unwrap());
                        output.object_lock_mode = Some(String::from_str(response.headers()["x-amz-object-lock-mode"].to_str().unwrap()).unwrap());
                        output.object_lock_retain_until_date = Some(String::from_str(response.headers()["x-amz-object-lock-retain-until-date"].to_str().unwrap()).unwrap());
                        output.object_lock_legal_hold_status = Some(String::from_str(response.headers()["x-amz-object-lock-legal-hold"].to_str().unwrap()).unwrap());
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

pub fn get_object_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_acl(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectAclOutput = Default::default();
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetObjectAclOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn get_object_legal_hold(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectLegalHoldRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_legal_hold(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectLegalHoldOutput = Default::default();
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

pub fn get_object_lock_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectLockConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_lock_configuration(deserialized)
}

fn __get_object_lock_configuration(input: GetObjectLockConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?object-lock");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectLockConfigurationOutput = Default::default();
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

pub fn get_object_retention(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectRetentionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_retention(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectRetentionOutput = Default::default();
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

pub fn get_object_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_tagging(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectTaggingOutput = Default::default();
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetObjectTaggingOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };
                        output.version_id = Some(String::from_str(response.headers()["x-amz-version-id"].to_str().unwrap()).unwrap());
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

pub fn get_object_torrent(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetObjectTorrentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_object_torrent(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetObjectTorrentOutput = Default::default();
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match GetObjectTorrentOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn get_public_access_block(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: GetPublicAccessBlockRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __get_public_access_block(deserialized)
}

fn __get_public_access_block(input: GetPublicAccessBlockRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?publicAccessBlock");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: GetPublicAccessBlockOutput = Default::default();
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

pub fn head_bucket(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: HeadBucketRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __head_bucket(deserialized)
}

fn __head_bucket(input: HeadBucketRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("HEAD", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn head_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: HeadObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __head_object(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(if_match) = input.if_match {
        headers.insert("If-Match", serde_json::to_string(&if_match).unwrap());
    };
    if let Some(if_modified_since) = input.if_modified_since {
        headers.insert("If-Modified-Since", serde_json::to_string(&if_modified_since).unwrap());
    };
    if let Some(if_none_match) = input.if_none_match {
        headers.insert("If-None-Match", serde_json::to_string(&if_none_match).unwrap());
    };
    if let Some(if_unmodified_since) = input.if_unmodified_since {
        headers.insert("If-Unmodified-Since", serde_json::to_string(&if_unmodified_since).unwrap());
    };
    if let Some(range) = input.range {
        headers.insert("Range", serde_json::to_string(&range).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        headers.insert("x-amz-server-side-encryption-customer-algorithm", serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        headers.insert("x-amz-server-side-encryption-customer-key", serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        headers.insert("x-amz-server-side-encryption-customer-key-MD5", serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };
    if let Some(part_number) = input.part_number {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "partNumber", serde_json::to_string(&part_number).unwrap())),
            None => path.push_str(&format!("?{}={}", "partNumber", serde_json::to_string(&part_number).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("HEAD", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: HeadObjectOutput = Default::default();
                        output.delete_marker = Some(bool::from_str(response.headers()["x-amz-delete-marker"].to_str().unwrap()).unwrap());
                        output.accept_ranges = Some(String::from_str(response.headers()["accept-ranges"].to_str().unwrap()).unwrap());
                        output.expiration = Some(String::from_str(response.headers()["x-amz-expiration"].to_str().unwrap()).unwrap());
                        output.restore = Some(String::from_str(response.headers()["x-amz-restore"].to_str().unwrap()).unwrap());
                        output.archive_status = Some(String::from_str(response.headers()["x-amz-archive-status"].to_str().unwrap()).unwrap());
                        output.last_modified = Some(String::from_str(response.headers()["Last-Modified"].to_str().unwrap()).unwrap());
                        output.content_length = Some(u64::from_str(response.headers()["Content-Length"].to_str().unwrap()).unwrap());
                        output.e_tag = Some(String::from_str(response.headers()["ETag"].to_str().unwrap()).unwrap());
                        output.missing_meta = Some(i64::from_str(response.headers()["x-amz-missing-meta"].to_str().unwrap()).unwrap());
                        output.version_id = Some(String::from_str(response.headers()["x-amz-version-id"].to_str().unwrap()).unwrap());
                        output.cache_control = Some(String::from_str(response.headers()["Cache-Control"].to_str().unwrap()).unwrap());
                        output.content_disposition = Some(String::from_str(response.headers()["Content-Disposition"].to_str().unwrap()).unwrap());
                        output.content_encoding = Some(String::from_str(response.headers()["Content-Encoding"].to_str().unwrap()).unwrap());
                        output.content_language = Some(String::from_str(response.headers()["Content-Language"].to_str().unwrap()).unwrap());
                        output.content_type = Some(String::from_str(response.headers()["Content-Type"].to_str().unwrap()).unwrap());
                        output.expires = Some(String::from_str(response.headers()["Expires"].to_str().unwrap()).unwrap());
                        output.website_redirect_location = Some(String::from_str(response.headers()["x-amz-website-redirect-location"].to_str().unwrap()).unwrap());
                        output.server_side_encryption = Some(String::from_str(response.headers()["x-amz-server-side-encryption"].to_str().unwrap()).unwrap());
                        output.sse_customer_algorithm = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-algorithm"].to_str().unwrap()).unwrap());
                        output.sse_customer_key_md5 = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-key-MD5"].to_str().unwrap()).unwrap());
                        output.ssekms_key_id = Some(String::from_str(response.headers()["x-amz-server-side-encryption-aws-kms-key-id"].to_str().unwrap()).unwrap());
                        output.bucket_key_enabled = Some(bool::from_str(response.headers()["x-amz-server-side-encryption-bucket-key-enabled"].to_str().unwrap()).unwrap());
                        output.storage_class = Some(String::from_str(response.headers()["x-amz-storage-class"].to_str().unwrap()).unwrap());
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
                        output.replication_status = Some(String::from_str(response.headers()["x-amz-replication-status"].to_str().unwrap()).unwrap());
                        output.parts_count = Some(i64::from_str(response.headers()["x-amz-mp-parts-count"].to_str().unwrap()).unwrap());
                        output.object_lock_mode = Some(String::from_str(response.headers()["x-amz-object-lock-mode"].to_str().unwrap()).unwrap());
                        output.object_lock_retain_until_date = Some(String::from_str(response.headers()["x-amz-object-lock-retain-until-date"].to_str().unwrap()).unwrap());
                        output.object_lock_legal_hold_status = Some(String::from_str(response.headers()["x-amz-object-lock-legal-hold"].to_str().unwrap()).unwrap());
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

pub fn list_bucket_analytics_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketAnalyticsConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_analytics_configurations(deserialized)
}

fn __list_bucket_analytics_configurations(input: ListBucketAnalyticsConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?analytics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(continuation_token) = input.continuation_token {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "continuation-token", serde_json::to_string(&continuation_token).unwrap())),
            None => path.push_str(&format!("?{}={}", "continuation-token", serde_json::to_string(&continuation_token).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: ListBucketAnalyticsConfigurationsOutput = Default::default();
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

pub fn list_bucket_intelligent_tiering_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketIntelligentTieringConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_intelligent_tiering_configurations(deserialized)
}

fn __list_bucket_intelligent_tiering_configurations(input: ListBucketIntelligentTieringConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?intelligent-tiering");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(continuation_token) = input.continuation_token {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "continuation-token", serde_json::to_string(&continuation_token).unwrap())),
            None => path.push_str(&format!("?{}={}", "continuation-token", serde_json::to_string(&continuation_token).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: ListBucketIntelligentTieringConfigurationsOutput = Default::default();
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

pub fn list_bucket_inventory_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketInventoryConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_inventory_configurations(deserialized)
}

fn __list_bucket_inventory_configurations(input: ListBucketInventoryConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?inventory");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(continuation_token) = input.continuation_token {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "continuation-token", serde_json::to_string(&continuation_token).unwrap())),
            None => path.push_str(&format!("?{}={}", "continuation-token", serde_json::to_string(&continuation_token).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: ListBucketInventoryConfigurationsOutput = Default::default();
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

pub fn list_bucket_metrics_configurations(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListBucketMetricsConfigurationsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_bucket_metrics_configurations(deserialized)
}

fn __list_bucket_metrics_configurations(input: ListBucketMetricsConfigurationsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?metrics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(continuation_token) = input.continuation_token {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "continuation-token", serde_json::to_string(&continuation_token).unwrap())),
            None => path.push_str(&format!("?{}={}", "continuation-token", serde_json::to_string(&continuation_token).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: ListBucketMetricsConfigurationsOutput = Default::default();
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

pub fn list_buckets(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: () = serde_json::from_slice(input.as_slice()).unwrap();
    __list_buckets(deserialized)
}

fn __list_buckets(input: ()) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/");

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: ListBucketsOutput = Default::default();
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

pub fn list_multipart_uploads(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListMultipartUploadsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_multipart_uploads(deserialized)
}

fn __list_multipart_uploads(input: ListMultipartUploadsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?uploads");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(delimiter) = input.delimiter {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "delimiter", serde_json::to_string(&delimiter).unwrap())),
            None => path.push_str(&format!("?{}={}", "delimiter", serde_json::to_string(&delimiter).unwrap())),
        }
    };
    if let Some(encoding_type) = input.encoding_type {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "encoding-type", serde_json::to_string(&encoding_type).unwrap())),
            None => path.push_str(&format!("?{}={}", "encoding-type", serde_json::to_string(&encoding_type).unwrap())),
        }
    };
    if let Some(key_marker) = input.key_marker {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "key-marker", serde_json::to_string(&key_marker).unwrap())),
            None => path.push_str(&format!("?{}={}", "key-marker", serde_json::to_string(&key_marker).unwrap())),
        }
    };
    if let Some(max_uploads) = input.max_uploads {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "max-uploads", serde_json::to_string(&max_uploads).unwrap())),
            None => path.push_str(&format!("?{}={}", "max-uploads", serde_json::to_string(&max_uploads).unwrap())),
        }
    };
    if let Some(prefix) = input.prefix {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "prefix", serde_json::to_string(&prefix).unwrap())),
            None => path.push_str(&format!("?{}={}", "prefix", serde_json::to_string(&prefix).unwrap())),
        }
    };
    if let Some(upload_id_marker) = input.upload_id_marker {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "upload-id-marker", serde_json::to_string(&upload_id_marker).unwrap())),
            None => path.push_str(&format!("?{}={}", "upload-id-marker", serde_json::to_string(&upload_id_marker).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: ListMultipartUploadsOutput = Default::default();
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

pub fn list_object_versions(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListObjectVersionsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_object_versions(deserialized)
}

fn __list_object_versions(input: ListObjectVersionsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?versions");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(delimiter) = input.delimiter {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "delimiter", serde_json::to_string(&delimiter).unwrap())),
            None => path.push_str(&format!("?{}={}", "delimiter", serde_json::to_string(&delimiter).unwrap())),
        }
    };
    if let Some(encoding_type) = input.encoding_type {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "encoding-type", serde_json::to_string(&encoding_type).unwrap())),
            None => path.push_str(&format!("?{}={}", "encoding-type", serde_json::to_string(&encoding_type).unwrap())),
        }
    };
    if let Some(key_marker) = input.key_marker {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "key-marker", serde_json::to_string(&key_marker).unwrap())),
            None => path.push_str(&format!("?{}={}", "key-marker", serde_json::to_string(&key_marker).unwrap())),
        }
    };
    if let Some(max_keys) = input.max_keys {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "max-keys", serde_json::to_string(&max_keys).unwrap())),
            None => path.push_str(&format!("?{}={}", "max-keys", serde_json::to_string(&max_keys).unwrap())),
        }
    };
    if let Some(prefix) = input.prefix {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "prefix", serde_json::to_string(&prefix).unwrap())),
            None => path.push_str(&format!("?{}={}", "prefix", serde_json::to_string(&prefix).unwrap())),
        }
    };
    if let Some(version_id_marker) = input.version_id_marker {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "version-id-marker", serde_json::to_string(&version_id_marker).unwrap())),
            None => path.push_str(&format!("?{}={}", "version-id-marker", serde_json::to_string(&version_id_marker).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: ListObjectVersionsOutput = Default::default();
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

pub fn list_objects(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListObjectsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_objects(deserialized)
}

fn __list_objects(input: ListObjectsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(delimiter) = input.delimiter {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "delimiter", serde_json::to_string(&delimiter).unwrap())),
            None => path.push_str(&format!("?{}={}", "delimiter", serde_json::to_string(&delimiter).unwrap())),
        }
    };
    if let Some(encoding_type) = input.encoding_type {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "encoding-type", serde_json::to_string(&encoding_type).unwrap())),
            None => path.push_str(&format!("?{}={}", "encoding-type", serde_json::to_string(&encoding_type).unwrap())),
        }
    };
    if let Some(marker) = input.marker {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "marker", serde_json::to_string(&marker).unwrap())),
            None => path.push_str(&format!("?{}={}", "marker", serde_json::to_string(&marker).unwrap())),
        }
    };
    if let Some(max_keys) = input.max_keys {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "max-keys", serde_json::to_string(&max_keys).unwrap())),
            None => path.push_str(&format!("?{}={}", "max-keys", serde_json::to_string(&max_keys).unwrap())),
        }
    };
    if let Some(prefix) = input.prefix {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "prefix", serde_json::to_string(&prefix).unwrap())),
            None => path.push_str(&format!("?{}={}", "prefix", serde_json::to_string(&prefix).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: ListObjectsOutput = Default::default();
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

pub fn list_objects_v2(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListObjectsV2Request = serde_json::from_slice(input.as_slice()).unwrap();
    __list_objects_v2(deserialized)
}

fn __list_objects_v2(input: ListObjectsV2Request) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?list-type=2");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(delimiter) = input.delimiter {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "delimiter", serde_json::to_string(&delimiter).unwrap())),
            None => path.push_str(&format!("?{}={}", "delimiter", serde_json::to_string(&delimiter).unwrap())),
        }
    };
    if let Some(encoding_type) = input.encoding_type {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "encoding-type", serde_json::to_string(&encoding_type).unwrap())),
            None => path.push_str(&format!("?{}={}", "encoding-type", serde_json::to_string(&encoding_type).unwrap())),
        }
    };
    if let Some(max_keys) = input.max_keys {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "max-keys", serde_json::to_string(&max_keys).unwrap())),
            None => path.push_str(&format!("?{}={}", "max-keys", serde_json::to_string(&max_keys).unwrap())),
        }
    };
    if let Some(prefix) = input.prefix {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "prefix", serde_json::to_string(&prefix).unwrap())),
            None => path.push_str(&format!("?{}={}", "prefix", serde_json::to_string(&prefix).unwrap())),
        }
    };
    if let Some(continuation_token) = input.continuation_token {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "continuation-token", serde_json::to_string(&continuation_token).unwrap())),
            None => path.push_str(&format!("?{}={}", "continuation-token", serde_json::to_string(&continuation_token).unwrap())),
        }
    };
    if let Some(fetch_owner) = input.fetch_owner {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "fetch-owner", serde_json::to_string(&fetch_owner).unwrap())),
            None => path.push_str(&format!("?{}={}", "fetch-owner", serde_json::to_string(&fetch_owner).unwrap())),
        }
    };
    if let Some(start_after) = input.start_after {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "start-after", serde_json::to_string(&start_after).unwrap())),
            None => path.push_str(&format!("?{}={}", "start-after", serde_json::to_string(&start_after).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: ListObjectsV2Output = Default::default();
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

pub fn list_parts(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: ListPartsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __list_parts(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(max_parts) = input.max_parts {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "max-parts", serde_json::to_string(&max_parts).unwrap())),
            None => path.push_str(&format!("?{}={}", "max-parts", serde_json::to_string(&max_parts).unwrap())),
        }
    };
    if let Some(part_number_marker) = input.part_number_marker {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "part-number-marker", serde_json::to_string(&part_number_marker).unwrap())),
            None => path.push_str(&format!("?{}={}", "part-number-marker", serde_json::to_string(&part_number_marker).unwrap())),
        }
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "uploadId", serde_json::to_string(&input.upload_id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "uploadId", serde_json::to_string(&input.upload_id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("GET", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: ListPartsOutput = Default::default();
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match ListPartsOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };
                        output.abort_date = Some(String::from_str(response.headers()["x-amz-abort-date"].to_str().unwrap()).unwrap());
                        output.abort_rule_id = Some(String::from_str(response.headers()["x-amz-abort-rule-id"].to_str().unwrap()).unwrap());
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn put_bucket_accelerate_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketAccelerateConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_accelerate_configuration(deserialized)
}

fn __put_bucket_accelerate_configuration(input: PutBucketAccelerateConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?accelerate");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("AccelerateConfiguration", serde_xml_rs::to_string(&input.accelerate_configuration).unwrap());
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_acl(deserialized)
}

fn __put_bucket_acl(input: PutBucketAclRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?acl");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(access_control_policy) = input.access_control_policy {
        body.insert("AccessControlPolicy", serde_json::to_string(&access_control_policy).unwrap());
    };
    if let Some(acl) = input.acl {
        headers.insert("x-amz-acl", serde_json::to_string(&acl).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        headers.insert("x-amz-grant-full-control", serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        headers.insert("x-amz-grant-read", serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        headers.insert("x-amz-grant-read-acp", serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write) = input.grant_write {
        headers.insert("x-amz-grant-write", serde_json::to_string(&grant_write).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        headers.insert("x-amz-grant-write-acp", serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_analytics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketAnalyticsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_analytics_configuration(deserialized)
}

fn __put_bucket_analytics_configuration(input: PutBucketAnalyticsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?analytics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("AnalyticsConfiguration", serde_xml_rs::to_string(&input.analytics_configuration).unwrap());
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_cors(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketCorsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_cors(deserialized)
}

fn __put_bucket_cors(input: PutBucketCorsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?cors");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("CORSConfiguration", serde_xml_rs::to_string(&input.cors_configuration).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_encryption(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketEncryptionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_encryption(deserialized)
}

fn __put_bucket_encryption(input: PutBucketEncryptionRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?encryption");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("ServerSideEncryptionConfiguration", serde_xml_rs::to_string(&input.server_side_encryption_configuration).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_intelligent_tiering_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketIntelligentTieringConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_intelligent_tiering_configuration(deserialized)
}

fn __put_bucket_intelligent_tiering_configuration(input: PutBucketIntelligentTieringConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?intelligent-tiering");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("IntelligentTieringConfiguration", serde_xml_rs::to_string(&input.intelligent_tiering_configuration).unwrap());
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_inventory_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketInventoryConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_inventory_configuration(deserialized)
}

fn __put_bucket_inventory_configuration(input: PutBucketInventoryConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?inventory");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("InventoryConfiguration", serde_xml_rs::to_string(&input.inventory_configuration).unwrap());
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_lifecycle(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketLifecycleRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_lifecycle(deserialized)
}

fn __put_bucket_lifecycle(input: PutBucketLifecycleRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?lifecycle");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(lifecycle_configuration) = input.lifecycle_configuration {
        body.insert("LifecycleConfiguration", serde_json::to_string(&lifecycle_configuration).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_lifecycle_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketLifecycleConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_lifecycle_configuration(deserialized)
}

fn __put_bucket_lifecycle_configuration(input: PutBucketLifecycleConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?lifecycle");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(lifecycle_configuration) = input.lifecycle_configuration {
        body.insert("LifecycleConfiguration", serde_json::to_string(&lifecycle_configuration).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_logging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketLoggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_logging(deserialized)
}

fn __put_bucket_logging(input: PutBucketLoggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?logging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("BucketLoggingStatus", serde_xml_rs::to_string(&input.bucket_logging_status).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_metrics_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketMetricsConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_metrics_configuration(deserialized)
}

fn __put_bucket_metrics_configuration(input: PutBucketMetricsConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?metrics");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("MetricsConfiguration", serde_xml_rs::to_string(&input.metrics_configuration).unwrap());
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "id", serde_json::to_string(&input.id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_notification(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketNotificationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_notification(deserialized)
}

fn __put_bucket_notification(input: PutBucketNotificationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?notification");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("NotificationConfiguration", serde_xml_rs::to_string(&input.notification_configuration).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_notification_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketNotificationConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_notification_configuration(deserialized)
}

fn __put_bucket_notification_configuration(input: PutBucketNotificationConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?notification");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("NotificationConfiguration", serde_xml_rs::to_string(&input.notification_configuration).unwrap());
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_ownership_controls(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketOwnershipControlsRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_ownership_controls(deserialized)
}

fn __put_bucket_ownership_controls(input: PutBucketOwnershipControlsRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?ownershipControls");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("OwnershipControls", serde_xml_rs::to_string(&input.ownership_controls).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_policy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketPolicyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_policy(deserialized)
}

fn __put_bucket_policy(input: PutBucketPolicyRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?policy");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("Policy", serde_xml_rs::to_string(&input.policy).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(confirm_remove_self_bucket_access) = input.confirm_remove_self_bucket_access {
        headers.insert("x-amz-confirm-remove-self-bucket-access", serde_json::to_string(&confirm_remove_self_bucket_access).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_replication(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketReplicationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_replication(deserialized)
}

fn __put_bucket_replication(input: PutBucketReplicationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?replication");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("ReplicationConfiguration", serde_xml_rs::to_string(&input.replication_configuration).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(token) = input.token {
        headers.insert("x-amz-bucket-object-lock-token", serde_json::to_string(&token).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_request_payment(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketRequestPaymentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_request_payment(deserialized)
}

fn __put_bucket_request_payment(input: PutBucketRequestPaymentRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?requestPayment");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("RequestPaymentConfiguration", serde_xml_rs::to_string(&input.request_payment_configuration).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_tagging(deserialized)
}

fn __put_bucket_tagging(input: PutBucketTaggingRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?tagging");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("Tagging", serde_xml_rs::to_string(&input.tagging).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_versioning(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketVersioningRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_versioning(deserialized)
}

fn __put_bucket_versioning(input: PutBucketVersioningRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?versioning");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("VersioningConfiguration", serde_xml_rs::to_string(&input.versioning_configuration).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(mfa) = input.mfa {
        headers.insert("x-amz-mfa", serde_json::to_string(&mfa).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_bucket_website(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutBucketWebsiteRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_bucket_website(deserialized)
}

fn __put_bucket_website(input: PutBucketWebsiteRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?website");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("WebsiteConfiguration", serde_xml_rs::to_string(&input.website_configuration).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn put_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(acl) = input.acl {
        headers.insert("x-amz-acl", serde_json::to_string(&acl).unwrap());
    };
    if let Some(cache_control) = input.cache_control {
        headers.insert("Cache-Control", serde_json::to_string(&cache_control).unwrap());
    };
    if let Some(content_disposition) = input.content_disposition {
        headers.insert("Content-Disposition", serde_json::to_string(&content_disposition).unwrap());
    };
    if let Some(content_encoding) = input.content_encoding {
        headers.insert("Content-Encoding", serde_json::to_string(&content_encoding).unwrap());
    };
    if let Some(content_language) = input.content_language {
        headers.insert("Content-Language", serde_json::to_string(&content_language).unwrap());
    };
    if let Some(content_length) = input.content_length {
        headers.insert("Content-Length", serde_json::to_string(&content_length).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(content_type) = input.content_type {
        headers.insert("Content-Type", serde_json::to_string(&content_type).unwrap());
    };
    if let Some(expires) = input.expires {
        headers.insert("Expires", serde_json::to_string(&expires).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        headers.insert("x-amz-grant-full-control", serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        headers.insert("x-amz-grant-read", serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        headers.insert("x-amz-grant-read-acp", serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        headers.insert("x-amz-grant-write-acp", serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(server_side_encryption) = input.server_side_encryption {
        headers.insert("x-amz-server-side-encryption", serde_json::to_string(&server_side_encryption).unwrap());
    };
    if let Some(storage_class) = input.storage_class {
        headers.insert("x-amz-storage-class", serde_json::to_string(&storage_class).unwrap());
    };
    if let Some(website_redirect_location) = input.website_redirect_location {
        headers.insert("x-amz-website-redirect-location", serde_json::to_string(&website_redirect_location).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        headers.insert("x-amz-server-side-encryption-customer-algorithm", serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        headers.insert("x-amz-server-side-encryption-customer-key", serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        headers.insert("x-amz-server-side-encryption-customer-key-MD5", serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(ssekms_key_id) = input.ssekms_key_id {
        headers.insert("x-amz-server-side-encryption-aws-kms-key-id", serde_json::to_string(&ssekms_key_id).unwrap());
    };
    if let Some(ssekms_encryption_context) = input.ssekms_encryption_context {
        headers.insert("x-amz-server-side-encryption-context", serde_json::to_string(&ssekms_encryption_context).unwrap());
    };
    if let Some(bucket_key_enabled) = input.bucket_key_enabled {
        headers.insert("x-amz-server-side-encryption-bucket-key-enabled", serde_json::to_string(&bucket_key_enabled).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(tagging) = input.tagging {
        headers.insert("x-amz-tagging", serde_json::to_string(&tagging).unwrap());
    };
    if let Some(object_lock_mode) = input.object_lock_mode {
        headers.insert("x-amz-object-lock-mode", serde_json::to_string(&object_lock_mode).unwrap());
    };
    if let Some(object_lock_retain_until_date) = input.object_lock_retain_until_date {
        headers.insert("x-amz-object-lock-retain-until-date", serde_json::to_string(&object_lock_retain_until_date).unwrap());
    };
    if let Some(object_lock_legal_hold_status) = input.object_lock_legal_hold_status {
        headers.insert("x-amz-object-lock-legal-hold", serde_json::to_string(&object_lock_legal_hold_status).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectOutput = Default::default();
                        output.expiration = Some(String::from_str(response.headers()["x-amz-expiration"].to_str().unwrap()).unwrap());
                        output.e_tag = Some(String::from_str(response.headers()["ETag"].to_str().unwrap()).unwrap());
                        output.server_side_encryption = Some(String::from_str(response.headers()["x-amz-server-side-encryption"].to_str().unwrap()).unwrap());
                        output.version_id = Some(String::from_str(response.headers()["x-amz-version-id"].to_str().unwrap()).unwrap());
                        output.sse_customer_algorithm = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-algorithm"].to_str().unwrap()).unwrap());
                        output.sse_customer_key_md5 = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-key-MD5"].to_str().unwrap()).unwrap());
                        output.ssekms_key_id = Some(String::from_str(response.headers()["x-amz-server-side-encryption-aws-kms-key-id"].to_str().unwrap()).unwrap());
                        output.ssekms_encryption_context = Some(String::from_str(response.headers()["x-amz-server-side-encryption-context"].to_str().unwrap()).unwrap());
                        output.bucket_key_enabled = Some(bool::from_str(response.headers()["x-amz-server-side-encryption-bucket-key-enabled"].to_str().unwrap()).unwrap());
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn put_object_acl(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectAclRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_acl(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(access_control_policy) = input.access_control_policy {
        body.insert("AccessControlPolicy", serde_json::to_string(&access_control_policy).unwrap());
    };
    if let Some(acl) = input.acl {
        headers.insert("x-amz-acl", serde_json::to_string(&acl).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(grant_full_control) = input.grant_full_control {
        headers.insert("x-amz-grant-full-control", serde_json::to_string(&grant_full_control).unwrap());
    };
    if let Some(grant_read) = input.grant_read {
        headers.insert("x-amz-grant-read", serde_json::to_string(&grant_read).unwrap());
    };
    if let Some(grant_read_acp) = input.grant_read_acp {
        headers.insert("x-amz-grant-read-acp", serde_json::to_string(&grant_read_acp).unwrap());
    };
    if let Some(grant_write) = input.grant_write {
        headers.insert("x-amz-grant-write", serde_json::to_string(&grant_write).unwrap());
    };
    if let Some(grant_write_acp) = input.grant_write_acp {
        headers.insert("x-amz-grant-write-acp", serde_json::to_string(&grant_write_acp).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectAclOutput = Default::default();
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn put_object_legal_hold(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectLegalHoldRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_legal_hold(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(legal_hold) = input.legal_hold {
        body.insert("LegalHold", serde_json::to_string(&legal_hold).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectLegalHoldOutput = Default::default();
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn put_object_lock_configuration(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectLockConfigurationRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_lock_configuration(deserialized)
}

fn __put_object_lock_configuration(input: PutObjectLockConfigurationRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?object-lock");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(object_lock_configuration) = input.object_lock_configuration {
        body.insert("ObjectLockConfiguration", serde_json::to_string(&object_lock_configuration).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(token) = input.token {
        headers.insert("x-amz-bucket-object-lock-token", serde_json::to_string(&token).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectLockConfigurationOutput = Default::default();
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn put_object_retention(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectRetentionRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_retention(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(retention) = input.retention {
        body.insert("Retention", serde_json::to_string(&retention).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(bypass_governance_retention) = input.bypass_governance_retention {
        headers.insert("x-amz-bypass-governance-retention", serde_json::to_string(&bypass_governance_retention).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectRetentionOutput = Default::default();
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn put_object_tagging(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutObjectTaggingRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_object_tagging(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("Tagging", serde_xml_rs::to_string(&input.tagging).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: PutObjectTaggingOutput = Default::default();
                        output.version_id = Some(String::from_str(response.headers()["x-amz-version-id"].to_str().unwrap()).unwrap());
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

pub fn put_public_access_block(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: PutPublicAccessBlockRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __put_public_access_block(deserialized)
}

fn __put_public_access_block(input: PutPublicAccessBlockRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}?publicAccessBlock");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    body.insert("PublicAccessBlockConfiguration", serde_xml_rs::to_string(&input.public_access_block_configuration).unwrap());
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

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

pub fn restore_object(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: RestoreObjectRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __restore_object(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(restore_request) = input.restore_request {
        body.insert("RestoreRequest", serde_json::to_string(&restore_request).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(version_id) = input.version_id {
        match path.find('?') {
            Some(_) => path.push_str(&format!("&{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
            None => path.push_str(&format!("?{}={}", "versionId", serde_json::to_string(&version_id).unwrap())),
        }
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("POST", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: RestoreObjectOutput = Default::default();
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
                        output.restore_output_path = Some(String::from_str(response.headers()["x-amz-restore-output-path"].to_str().unwrap()).unwrap());
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

pub fn select_object_content(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: SelectObjectContentRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __select_object_content(deserialized)
}

fn __select_object_content(input: SelectObjectContentRequest) -> BoxFuture<'static, Vec<u8>> {
    let mut path = String::from("/{Bucket}/{Key+}?select&amp;select-type=2");
    path = match path.find("{Bucket}") {
        Some(_) => path.replace("{Bucket}", &input.bucket),
        None => path.to_string(),
    };
    path = match path.find("{Key+}") {
        Some(_) => path.replace("{Key+}", &input.key),
        None => path.to_string(),
    };

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
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
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        headers.insert("x-amz-server-side-encryption-customer-algorithm", serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        headers.insert("x-amz-server-side-encryption-customer-key", serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        headers.insert("x-amz-server-side-encryption-customer-key-MD5", serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("POST", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: SelectObjectContentOutput = Default::default();
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

pub fn upload_part(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UploadPartRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __upload_part(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    if let Some(content_length) = input.content_length {
        headers.insert("Content-Length", serde_json::to_string(&content_length).unwrap());
    };
    if let Some(content_md5) = input.content_md5 {
        headers.insert("Content-MD5", serde_json::to_string(&content_md5).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        headers.insert("x-amz-server-side-encryption-customer-algorithm", serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        headers.insert("x-amz-server-side-encryption-customer-key", serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        headers.insert("x-amz-server-side-encryption-customer-key-MD5", serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "partNumber", serde_json::to_string(&input.part_number).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "partNumber", serde_json::to_string(&input.part_number).unwrap())),
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "uploadId", serde_json::to_string(&input.upload_id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "uploadId", serde_json::to_string(&input.upload_id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: UploadPartOutput = Default::default();
                        output.server_side_encryption = Some(String::from_str(response.headers()["x-amz-server-side-encryption"].to_str().unwrap()).unwrap());
                        output.e_tag = Some(String::from_str(response.headers()["ETag"].to_str().unwrap()).unwrap());
                        output.sse_customer_algorithm = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-algorithm"].to_str().unwrap()).unwrap());
                        output.sse_customer_key_md5 = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-key-MD5"].to_str().unwrap()).unwrap());
                        output.ssekms_key_id = Some(String::from_str(response.headers()["x-amz-server-side-encryption-aws-kms-key-id"].to_str().unwrap()).unwrap());
                        output.bucket_key_enabled = Some(bool::from_str(response.headers()["x-amz-server-side-encryption-bucket-key-enabled"].to_str().unwrap()).unwrap());
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

pub fn upload_part_copy(input: Vec<u8>) -> BoxFuture<'static, Vec<u8>> {
    let deserialized: UploadPartCopyRequest = serde_json::from_slice(input.as_slice()).unwrap();
    __upload_part_copy(deserialized)
}

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

    let mut body = std::collections::HashMap::new();
    let mut headers = std::collections::HashMap::new();
    headers.insert("x-amz-copy-source", serde_json::to_string(&input.copy_source).unwrap());
    if let Some(copy_source_if_match) = input.copy_source_if_match {
        headers.insert("x-amz-copy-source-if-match", serde_json::to_string(&copy_source_if_match).unwrap());
    };
    if let Some(copy_source_if_modified_since) = input.copy_source_if_modified_since {
        headers.insert("x-amz-copy-source-if-modified-since", serde_json::to_string(&copy_source_if_modified_since).unwrap());
    };
    if let Some(copy_source_if_none_match) = input.copy_source_if_none_match {
        headers.insert("x-amz-copy-source-if-none-match", serde_json::to_string(&copy_source_if_none_match).unwrap());
    };
    if let Some(copy_source_if_unmodified_since) = input.copy_source_if_unmodified_since {
        headers.insert("x-amz-copy-source-if-unmodified-since", serde_json::to_string(&copy_source_if_unmodified_since).unwrap());
    };
    if let Some(copy_source_range) = input.copy_source_range {
        headers.insert("x-amz-copy-source-range", serde_json::to_string(&copy_source_range).unwrap());
    };
    if let Some(sse_customer_algorithm) = input.sse_customer_algorithm {
        headers.insert("x-amz-server-side-encryption-customer-algorithm", serde_json::to_string(&sse_customer_algorithm).unwrap());
    };
    if let Some(sse_customer_key) = input.sse_customer_key {
        headers.insert("x-amz-server-side-encryption-customer-key", serde_json::to_string(&sse_customer_key).unwrap());
    };
    if let Some(sse_customer_key_md5) = input.sse_customer_key_md5 {
        headers.insert("x-amz-server-side-encryption-customer-key-MD5", serde_json::to_string(&sse_customer_key_md5).unwrap());
    };
    if let Some(copy_source_sse_customer_algorithm) = input.copy_source_sse_customer_algorithm {
        headers.insert("x-amz-copy-source-server-side-encryption-customer-algorithm", serde_json::to_string(&copy_source_sse_customer_algorithm).unwrap());
    };
    if let Some(copy_source_sse_customer_key) = input.copy_source_sse_customer_key {
        headers.insert("x-amz-copy-source-server-side-encryption-customer-key", serde_json::to_string(&copy_source_sse_customer_key).unwrap());
    };
    if let Some(copy_source_sse_customer_key_md5) = input.copy_source_sse_customer_key_md5 {
        headers.insert("x-amz-copy-source-server-side-encryption-customer-key-MD5", serde_json::to_string(&copy_source_sse_customer_key_md5).unwrap());
    };
    if let Some(request_payer) = input.request_payer {
        headers.insert("x-amz-request-payer", serde_json::to_string(&request_payer).unwrap());
    };
    if let Some(expected_bucket_owner) = input.expected_bucket_owner {
        headers.insert("x-amz-expected-bucket-owner", serde_json::to_string(&expected_bucket_owner).unwrap());
    };
    if let Some(expected_source_bucket_owner) = input.expected_source_bucket_owner {
        headers.insert("x-amz-source-expected-bucket-owner", serde_json::to_string(&expected_source_bucket_owner).unwrap());
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "partNumber", serde_json::to_string(&input.part_number).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "partNumber", serde_json::to_string(&input.part_number).unwrap())),
    };
    path = match path.find('?') {
        Some(_) => String::from(&format!("{}&{}={}", path, "uploadId", serde_json::to_string(&input.upload_id).unwrap())),
        None => String::from(&format!("{}?{}={}", path, "uploadId", serde_json::to_string(&input.upload_id).unwrap())),
    };

    let client_input = client::ClientInput {
        body,
        headers,
    };

    Box::pin(async move {
        match crate::CLIENT.call("PUT", &path, "rest-xml", client_input).await {
            Ok(response) => {
                let status = response.status();
                let body = response.body();

                match status {
                    StatusCode::OK => {
                        let mut output: UploadPartCopyOutput = Default::default();
                        let reader = EventReader::new_with_config(body.as_ref(), ParserConfig::new().trim_whitespace(false));
                        let mut stack = xml_util::util::XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let actual_tag_name = xml_util::util::peek_at_name(&mut stack).unwrap();
                        output = match UploadPartCopyOutputDeserializer::deserialize(&actual_tag_name, &mut stack) {
                            Ok(response) => response,
                            _ => panic!("Unhandled XML parse error"),
                        };
                        output.copy_source_version_id = Some(String::from_str(response.headers()["x-amz-copy-source-version-id"].to_str().unwrap()).unwrap());
                        output.server_side_encryption = Some(String::from_str(response.headers()["x-amz-server-side-encryption"].to_str().unwrap()).unwrap());
                        output.sse_customer_algorithm = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-algorithm"].to_str().unwrap()).unwrap());
                        output.sse_customer_key_md5 = Some(String::from_str(response.headers()["x-amz-server-side-encryption-customer-key-MD5"].to_str().unwrap()).unwrap());
                        output.ssekms_key_id = Some(String::from_str(response.headers()["x-amz-server-side-encryption-aws-kms-key-id"].to_str().unwrap()).unwrap());
                        output.bucket_key_enabled = Some(bool::from_str(response.headers()["x-amz-server-side-encryption-bucket-key-enabled"].to_str().unwrap()).unwrap());
                        output.request_charged = Some(String::from_str(response.headers()["x-amz-request-charged"].to_str().unwrap()).unwrap());
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

