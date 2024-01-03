# Webhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connect_timeout** | Option<**i32**> |  | [optional]
**data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**events_enabled** | Option<**::std::collections::HashMap<String, bool>**> |  | [optional]
**global** | Option<**bool**> |  | [optional]
**headers** | Option<[**serde_json::Value**](.md)> | Type for webhook headers. | [optional]
**http_authentication_password** | Option<**String**> |  | [optional]
**http_authentication_username** | Option<**String**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**read_timeout** | Option<**i32**> |  | [optional]
**signature_configuration** | Option<[**crate::models::WebhookSignatureConfiguration**](WebhookSignatureConfiguration.md)> |  | [optional]
**ssl_certificate** | Option<**String**> |  | [optional]
**ssl_certificate_key_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**tenant_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


