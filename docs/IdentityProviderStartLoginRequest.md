# IdentityProviderStartLoginRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**identity_provider_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**login_id** | Option<**String**> |  | [optional]
**state** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**application_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**ip_address** | Option<**String**> |  | [optional]
**meta_data** | Option<[**crate::models::MetaData**](MetaData.md)> |  | [optional]
**new_device** | Option<**bool**> |  | [optional]
**no_jwt** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


