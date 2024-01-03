# OpenIdConnectIdentityProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domains** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**button_image_url** | Option<**String**> |  | [optional]
**button_text** | Option<**String**> |  | [optional]
**oauth2** | Option<[**crate::models::IdentityProviderOauth2Configuration**](IdentityProviderOauth2Configuration.md)> |  | [optional]
**post_request** | Option<**bool**> |  | [optional]
**data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**application_configuration** | Option<[**::std::collections::HashMap<String, crate::models::OpenIdConnectApplicationConfiguration>**](OpenIdConnectApplicationConfiguration.md)> |  | [optional]
**debug** | Option<**bool**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**lambda_configuration** | Option<[**crate::models::ProviderLambdaConfiguration**](ProviderLambdaConfiguration.md)> |  | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**linking_strategy** | Option<[**crate::models::IdentityProviderLinkingStrategy**](IdentityProviderLinkingStrategy.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**tenant_configuration** | Option<[**::std::collections::HashMap<String, crate::models::IdentityProviderTenantConfiguration>**](IdentityProviderTenantConfiguration.md)> |  | [optional]
**r#type** | Option<[**crate::models::IdentityProviderType**](IdentityProviderType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


