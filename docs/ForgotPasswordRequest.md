# ForgotPasswordRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_id** | Option<**uuid::Uuid**> |  | [optional]
**change_password_id** | Option<**String**> |  | [optional]
**login_id** | Option<**String**> |  | [optional]
**login_id_types** | Option<**Vec<String>**> |  | [optional]
**send_forgot_password_email** | Option<**bool**> |  | [optional]
**send_forgot_password_message** | Option<**bool**> |  | [optional]
**state** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**username** | Option<**String**> |  | [optional]
**event_info** | Option<[**models::EventInfo**](EventInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


