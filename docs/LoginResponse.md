# LoginResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actions** | Option<[**Vec<crate::models::LoginPreventedResponse>**](LoginPreventedResponse.md)> |  | [optional]
**change_password_id** | Option<**String**> |  | [optional]
**change_password_reason** | Option<[**crate::models::ChangePasswordReason**](ChangePasswordReason.md)> |  | [optional]
**configurable_methods** | Option<**Vec<String>**> |  | [optional]
**email_verification_id** | Option<**String**> |  | [optional]
**methods** | Option<[**Vec<crate::models::TwoFactorMethod>**](TwoFactorMethod.md)> |  | [optional]
**pending_id_p_link_id** | Option<**String**> |  | [optional]
**refresh_token** | Option<**String**> |  | [optional]
**refresh_token_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**registration_verification_id** | Option<**String**> |  | [optional]
**state** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**threats_detected** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**token** | Option<**String**> |  | [optional]
**token_expiration_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**trust_token** | Option<**String**> |  | [optional]
**two_factor_id** | Option<**String**> |  | [optional]
**two_factor_trust_id** | Option<**String**> |  | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


