# GenericConnectorConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_url** | Option<**String**> |  | [optional]
**connect_timeout** | Option<**i32**> |  | [optional]
**headers** | Option<[**serde_json::Value**](.md)> | Type for webhook headers. | [optional]
**http_authentication_password** | Option<**String**> |  | [optional]
**http_authentication_username** | Option<**String**> |  | [optional]
**read_timeout** | Option<**i32**> |  | [optional]
**ssl_certificate_key_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**debug** | Option<**bool**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**name** | Option<**String**> |  | [optional]
**r#type** | Option<[**crate::models::ConnectorType**](ConnectorType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


