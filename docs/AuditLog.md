# AuditLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**id** | Option<**i64**> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**insert_user** | Option<**String**> |  | [optional]
**message** | Option<**String**> |  | [optional]
**new_value** | Option<[**serde_json::Value**](.md)> |  | [optional]
**old_value** | Option<[**serde_json::Value**](.md)> |  | [optional]
**reason** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


