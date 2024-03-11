# SecureIdentity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**breached_password_last_checked_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**breached_password_status** | Option<[**models::BreachedPasswordStatus**](BreachedPasswordStatus.md)> |  | [optional]
**connector_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**encryption_scheme** | Option<**String**> |  | [optional]
**factor** | Option<**i32**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**last_login_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**password** | Option<**String**> |  | [optional]
**password_change_reason** | Option<[**models::ChangePasswordReason**](ChangePasswordReason.md)> |  | [optional]
**password_change_required** | Option<**bool**> |  | [optional]
**password_last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**salt** | Option<**String**> |  | [optional]
**unique_username** | Option<**String**> |  | [optional]
**username** | Option<**String**> |  | [optional]
**username_status** | Option<[**models::ContentStatus**](ContentStatus.md)> |  | [optional]
**verified** | Option<**bool**> |  | [optional]
**verified_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


