# Consent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**consent_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**country_minimum_age_for_self_consent** | Option<[**serde_json::Value**](.md)> | Models a set of localized Integers that can be stored as JSON. | [optional]
**default_minimum_age_for_self_consent** | Option<**i32**> |  | [optional]
**email_plus** | Option<[**models::EmailPlus**](EmailPlus.md)> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**multiple_values_allowed** | Option<**bool**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**values** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


