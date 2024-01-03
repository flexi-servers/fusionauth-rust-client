# FormField

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**confirm** | Option<**bool**> |  | [optional]
**consent_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**control** | Option<[**crate::models::FormControl**](FormControl.md)> |  | [optional]
**data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**key** | Option<**String**> |  | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**name** | Option<**String**> |  | [optional]
**options** | Option<**Vec<String>**> |  | [optional]
**required** | Option<**bool**> |  | [optional]
**r#type** | Option<[**crate::models::FormDataType**](FormDataType.md)> |  | [optional]
**validator** | Option<[**crate::models::FormFieldValidator**](FormFieldValidator.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


