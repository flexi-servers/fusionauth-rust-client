# SmsMessageTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_template** | Option<**String**> |  | [optional]
**localized_templates** | Option<[**serde_json::Value**](.md)> | Models a set of localized Strings that can be stored as JSON. | [optional]
**data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**name** | Option<**String**> |  | [optional]
**r#type** | Option<[**models::MessageType**](MessageType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


