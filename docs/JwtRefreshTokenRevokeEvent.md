# JwtRefreshTokenRevokeEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | Option<[**models::User**](User.md)> |  | [optional]
**application_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**application_time_to_live_in_seconds** | Option<**std::collections::HashMap<String, i32>**> |  | [optional]
**refresh_token** | Option<[**models::RefreshToken**](RefreshToken.md)> |  | [optional]
**user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**create_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**info** | Option<[**models::EventInfo**](EventInfo.md)> |  | [optional]
**tenant_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**r#type** | Option<[**models::EventType**](EventType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


