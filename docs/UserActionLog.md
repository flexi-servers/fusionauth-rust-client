# UserActionLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actionee_user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**actioner_user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**application_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**email_user_on_end** | Option<**bool**> |  | [optional]
**end_event_sent** | Option<**bool**> |  | [optional]
**expiry** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**history** | Option<[**crate::models::LogHistory**](LogHistory.md)> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**localized_name** | Option<**String**> |  | [optional]
**localized_option** | Option<**String**> |  | [optional]
**localized_reason** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**notify_user_on_end** | Option<**bool**> |  | [optional]
**option** | Option<**String**> |  | [optional]
**reason** | Option<**String**> |  | [optional]
**reason_code** | Option<**String**> |  | [optional]
**user_action_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


