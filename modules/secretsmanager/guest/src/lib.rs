use std::fmt;
use std::fmt::Formatter;

use assemblylift_core_iomod_guest::{call, iomod};
use serde::{Deserialize, Serialize};

use crate::structs::*;

iomod!(akkoro.aws.secretsmanager);

pub mod structs;

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

call!(cancel_rotate_secret, CancelRotateSecretRequest => Result<CancelRotateSecretResponse, Error>);
call!(create_secret, CreateSecretRequest => Result<CreateSecretResponse, Error>);
call!(delete_resource_policy, DeleteResourcePolicyRequest => Result<DeleteResourcePolicyResponse, Error>);
call!(delete_secret, DeleteSecretRequest => Result<DeleteSecretResponse, Error>);
call!(describe_secret, DescribeSecretRequest => Result<DescribeSecretResponse, Error>);
call!(get_random_password, GetRandomPasswordRequest => Result<GetRandomPasswordResponse, Error>);
call!(get_resource_policy, GetResourcePolicyRequest => Result<GetResourcePolicyResponse, Error>);
call!(get_secret_value, GetSecretValueRequest => Result<GetSecretValueResponse, Error>);
call!(list_secret_version_ids, ListSecretVersionIdsRequest => Result<ListSecretVersionIdsResponse, Error>);
call!(list_secrets, ListSecretsRequest => Result<ListSecretsResponse, Error>);
call!(put_resource_policy, PutResourcePolicyRequest => Result<PutResourcePolicyResponse, Error>);
call!(put_secret_value, PutSecretValueRequest => Result<PutSecretValueResponse, Error>);
call!(remove_regions_from_replication, RemoveRegionsFromReplicationRequest => Result<RemoveRegionsFromReplicationResponse, Error>);
call!(replicate_secret_to_regions, ReplicateSecretToRegionsRequest => Result<ReplicateSecretToRegionsResponse, Error>);
call!(restore_secret, RestoreSecretRequest => Result<RestoreSecretResponse, Error>);
call!(rotate_secret, RotateSecretRequest => Result<RotateSecretResponse, Error>);
call!(stop_replication_to_replica, StopReplicationToReplicaRequest => Result<StopReplicationToReplicaResponse, Error>);
call!(tag_resource, TagResourceRequest => Result<(), Error>);
call!(untag_resource, UntagResourceRequest => Result<(), Error>);
call!(update_secret, UpdateSecretRequest => Result<UpdateSecretResponse, Error>);
call!(update_secret_version_stage, UpdateSecretVersionStageRequest => Result<UpdateSecretVersionStageResponse, Error>);
call!(validate_resource_policy, ValidateResourcePolicyRequest => Result<ValidateResourcePolicyResponse, Error>);
