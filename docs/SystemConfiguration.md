# SystemConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audit_log_configuration** | Option<[**models::AuditLogConfiguration**](AuditLogConfiguration.md)> |  | [optional]
**cors_configuration** | Option<[**models::CorsConfiguration**](CORSConfiguration.md)> |  | [optional]
**data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**event_log_configuration** | Option<[**models::EventLogConfiguration**](EventLogConfiguration.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**login_record_configuration** | Option<[**models::LoginRecordConfiguration**](LoginRecordConfiguration.md)> |  | [optional]
**report_timezone** | Option<**String**> | Timezone Identifier | [optional]
**trusted_proxy_configuration** | Option<[**models::SystemTrustedProxyConfiguration**](SystemTrustedProxyConfiguration.md)> |  | [optional]
**ui_configuration** | Option<[**models::UiConfiguration**](UIConfiguration.md)> |  | [optional]
**webhook_event_log_configuration** | Option<[**models::WebhookEventLogConfiguration**](WebhookEventLogConfiguration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


