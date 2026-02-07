# RegistrationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**refresh_token** | Option<**String**> |  | [optional]
**refresh_token_id** | Option<**uuid::Uuid**> |  | [optional]
**registration** | Option<[**models::UserRegistration**](UserRegistration.md)> |  | [optional]
**registration_verification_id** | Option<**String**> |  | [optional]
**registration_verification_one_time_code** | Option<**String**> |  | [optional]
**token** | Option<**String**> |  | [optional]
**token_expiration_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**user** | Option<[**models::User**](User.md)> |  | [optional]
**verification_ids** | Option<[**Vec<models::VerificationId>**](VerificationId.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


