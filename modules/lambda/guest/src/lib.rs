use std::fmt;

use assemblylift_core_iomod_guest::{call, export_iomod_guest};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};

use crate::structs::*;

export_iomod_guest!(akkoro, aws, lambda);

// mod serialization;
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

call!(add_layer_version_permission, AddLayerVersionPermissionRequest => Result<AddLayerVersionPermissionResponse, Error>);
call!(add_permission, AddPermissionRequest => Result<AddPermissionResponse, Error>);
call!(create_alias, CreateAliasRequest => Result<AliasConfiguration, Error>);
call!(create_code_signing_config, CreateCodeSigningConfigRequest => Result<CreateCodeSigningConfigResponse, Error>);
call!(create_event_source_mapping, CreateEventSourceMappingRequest => Result<EventSourceMappingConfiguration, Error>);
call!(create_function, CreateFunctionRequest => Result<FunctionConfiguration, Error>);
call!(delete_alias, DeleteAliasRequest => Result<(), Error>);
call!(delete_code_signing_config, DeleteCodeSigningConfigRequest => Result<DeleteCodeSigningConfigResponse, Error>);
call!(delete_event_source_mapping, DeleteEventSourceMappingRequest => Result<EventSourceMappingConfiguration, Error>);
call!(delete_function, DeleteFunctionRequest => Result<(), Error>);
call!(delete_function_code_signing_config, DeleteFunctionCodeSigningConfigRequest => Result<(), Error>);
call!(delete_function_concurrency, DeleteFunctionConcurrencyRequest => Result<(), Error>);
call!(delete_function_event_invoke_config, DeleteFunctionEventInvokeConfigRequest => Result<(), Error>);
call!(delete_layer_version, DeleteLayerVersionRequest => Result<(), Error>);
call!(delete_provisioned_concurrency_config, DeleteProvisionedConcurrencyConfigRequest => Result<(), Error>);
call!(get_account_settings, GetAccountSettingsRequest => Result<GetAccountSettingsResponse, Error>);
call!(get_alias, GetAliasRequest => Result<AliasConfiguration, Error>);
call!(get_code_signing_config, GetCodeSigningConfigRequest => Result<GetCodeSigningConfigResponse, Error>);
call!(get_event_source_mapping, GetEventSourceMappingRequest => Result<EventSourceMappingConfiguration, Error>);
call!(get_function, GetFunctionRequest => Result<GetFunctionResponse, Error>);
call!(get_function_code_signing_config, GetFunctionCodeSigningConfigRequest => Result<GetFunctionCodeSigningConfigResponse, Error>);
call!(get_function_concurrency, GetFunctionConcurrencyRequest => Result<GetFunctionConcurrencyResponse, Error>);
call!(get_function_configuration, GetFunctionConfigurationRequest => Result<FunctionConfiguration, Error>);
call!(get_function_event_invoke_config, GetFunctionEventInvokeConfigRequest => Result<FunctionEventInvokeConfig, Error>);
call!(get_layer_version, GetLayerVersionRequest => Result<GetLayerVersionResponse, Error>);
call!(get_layer_version_by_arn, GetLayerVersionByArnRequest => Result<GetLayerVersionResponse, Error>);
call!(get_layer_version_policy, GetLayerVersionPolicyRequest => Result<GetLayerVersionPolicyResponse, Error>);
call!(get_policy, GetPolicyRequest => Result<GetPolicyResponse, Error>);
call!(get_provisioned_concurrency_config, GetProvisionedConcurrencyConfigRequest => Result<GetProvisionedConcurrencyConfigResponse, Error>);
call!(invoke, InvocationRequest => Result<InvocationResponse, Error>);
call!(invoke_async, InvokeAsyncRequest => Result<InvokeAsyncResponse, Error>);
call!(list_aliases, ListAliasesRequest => Result<ListAliasesResponse, Error>);
call!(list_code_signing_configs, ListCodeSigningConfigsRequest => Result<ListCodeSigningConfigsResponse, Error>);
call!(list_event_source_mappings, ListEventSourceMappingsRequest => Result<ListEventSourceMappingsResponse, Error>);
call!(list_function_event_invoke_configs, ListFunctionEventInvokeConfigsRequest => Result<ListFunctionEventInvokeConfigsResponse, Error>);
call!(list_functions, ListFunctionsRequest => Result<ListFunctionsResponse, Error>);
call!(list_functions_by_code_signing_config, ListFunctionsByCodeSigningConfigRequest => Result<ListFunctionsByCodeSigningConfigResponse, Error>);
call!(list_layer_versions, ListLayerVersionsRequest => Result<ListLayerVersionsResponse, Error>);
call!(list_layers, ListLayersRequest => Result<ListLayersResponse, Error>);
call!(list_provisioned_concurrency_configs, ListProvisionedConcurrencyConfigsRequest => Result<ListProvisionedConcurrencyConfigsResponse, Error>);
call!(list_tags, ListTagsRequest => Result<ListTagsResponse, Error>);
call!(list_versions_by_function, ListVersionsByFunctionRequest => Result<ListVersionsByFunctionResponse, Error>);
call!(publish_layer_version, PublishLayerVersionRequest => Result<PublishLayerVersionResponse, Error>);
call!(publish_version, PublishVersionRequest => Result<FunctionConfiguration, Error>);
call!(put_function_code_signing_config, PutFunctionCodeSigningConfigRequest => Result<PutFunctionCodeSigningConfigResponse, Error>);
call!(put_function_concurrency, PutFunctionConcurrencyRequest => Result<Concurrency, Error>);
call!(put_function_event_invoke_config, PutFunctionEventInvokeConfigRequest => Result<FunctionEventInvokeConfig, Error>);
call!(put_provisioned_concurrency_config, PutProvisionedConcurrencyConfigRequest => Result<PutProvisionedConcurrencyConfigResponse, Error>);
call!(remove_layer_version_permission, RemoveLayerVersionPermissionRequest => Result<(), Error>);
call!(remove_permission, RemovePermissionRequest => Result<(), Error>);
call!(tag_resource, TagResourceRequest => Result<(), Error>);
call!(untag_resource, UntagResourceRequest => Result<(), Error>);
call!(update_alias, UpdateAliasRequest => Result<AliasConfiguration, Error>);
call!(update_code_signing_config, UpdateCodeSigningConfigRequest => Result<UpdateCodeSigningConfigResponse, Error>);
call!(update_event_source_mapping, UpdateEventSourceMappingRequest => Result<EventSourceMappingConfiguration, Error>);
call!(update_function_code, UpdateFunctionCodeRequest => Result<FunctionConfiguration, Error>);
call!(update_function_configuration, UpdateFunctionConfigurationRequest => Result<FunctionConfiguration, Error>);
call!(update_function_event_invoke_config, UpdateFunctionEventInvokeConfigRequest => Result<FunctionEventInvokeConfig, Error>);
