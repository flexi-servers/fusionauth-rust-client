# UserRegistration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**preferred_languages** | Option<**Vec<String>**> |  | [optional]
**tokens** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**application_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**authentication_token** | Option<**String**> |  | [optional]
**clean_speak_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_login_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**roles** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**timezone** | Option<**String**> | Timezone Identifier | [optional]
**username** | Option<**String**> |  | [optional]
**username_status** | Option<[**crate::models::ContentStatus**](ContentStatus.md)> |  | [optional]
**verified** | Option<**bool**> |  | [optional]
**verified_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


