# WebhookEventLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attempts** | Option<[**Vec<models::WebhookAttemptLog>**](WebhookAttemptLog.md)> |  | [optional]
**data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**event** | Option<[**models::EventRequest**](EventRequest.md)> |  | [optional]
**event_result** | Option<[**models::WebhookEventResult**](WebhookEventResult.md)> |  | [optional]
**event_type** | Option<[**models::EventType**](EventType.md)> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_attempt_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**linked_object_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**sequence** | Option<**i64**> |  | [optional]
**failed_attempts** | Option<**i32**> |  | [optional]
**successful_attempts** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


