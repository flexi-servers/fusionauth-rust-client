# UserConsent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**consent** | Option<[**crate::models::Consent**](Consent.md)> |  | [optional]
**consent_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**giver_user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**status** | Option<[**crate::models::ConsentStatus**](ConsentStatus.md)> |  | [optional]
**user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**values** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


