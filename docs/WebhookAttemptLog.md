# WebhookAttemptLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**end_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**start_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**webhook_call_response** | Option<[**models::WebhookCallResponse**](WebhookCallResponse.md)> |  | [optional]
**webhook_event_log_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**webhook_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**attempt_result** | Option<[**models::WebhookAttemptResult**](WebhookAttemptResult.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


