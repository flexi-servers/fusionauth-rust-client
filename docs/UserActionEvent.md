# UserActionEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**action** | Option<**String**> |  | [optional]
**action_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**actionee_user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**actioner_user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**email** | Option<[**models::Email**](Email.md)> |  | [optional]
**emailed_user** | Option<**bool**> |  | [optional]
**expiry** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**localized_action** | Option<**String**> |  | [optional]
**localized_duration** | Option<**String**> |  | [optional]
**localized_option** | Option<**String**> |  | [optional]
**localized_reason** | Option<**String**> |  | [optional]
**notify_user** | Option<**bool**> |  | [optional]
**option** | Option<**String**> |  | [optional]
**phase** | Option<[**models::UserActionPhase**](UserActionPhase.md)> |  | [optional]
**reason** | Option<**String**> |  | [optional]
**reason_code** | Option<**String**> |  | [optional]
**create_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**info** | Option<[**models::EventInfo**](EventInfo.md)> |  | [optional]
**tenant_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**r#type** | Option<[**models::EventType**](EventType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


