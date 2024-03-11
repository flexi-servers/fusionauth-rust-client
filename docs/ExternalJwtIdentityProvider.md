# ExternalJwtIdentityProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**claim_map** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**domains** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**default_key_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**header_key_parameter** | Option<**String**> |  | [optional]
**oauth2** | Option<[**models::IdentityProviderOauth2Configuration**](IdentityProviderOauth2Configuration.md)> |  | [optional]
**unique_identity_claim** | Option<**String**> |  | [optional]
**data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**application_configuration** | Option<[**std::collections::HashMap<String, models::ExternalJwtApplicationConfiguration>**](ExternalJWTApplicationConfiguration.md)> |  | [optional]
**debug** | Option<**bool**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**lambda_configuration** | Option<[**models::ProviderLambdaConfiguration**](ProviderLambdaConfiguration.md)> |  | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**linking_strategy** | Option<[**models::IdentityProviderLinkingStrategy**](IdentityProviderLinkingStrategy.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**tenant_configuration** | Option<[**std::collections::HashMap<String, models::IdentityProviderTenantConfiguration>**](IdentityProviderTenantConfiguration.md)> |  | [optional]
**r#type** | Option<[**models::IdentityProviderType**](IdentityProviderType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


