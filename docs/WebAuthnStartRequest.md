# WebAuthnStartRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**credential_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**login_id** | Option<**String**> |  | [optional]
**state** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**workflow** | Option<[**models::WebAuthnWorkflow**](WebAuthnWorkflow.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


