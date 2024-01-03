# SystemConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audit_log_configuration** | Option<[**crate::models::AuditLogConfiguration**](AuditLogConfiguration.md)> |  | [optional]
**cors_configuration** | Option<[**crate::models::CorsConfiguration**](CORSConfiguration.md)> |  | [optional]
**data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**event_log_configuration** | Option<[**crate::models::EventLogConfiguration**](EventLogConfiguration.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**login_record_configuration** | Option<[**crate::models::LoginRecordConfiguration**](LoginRecordConfiguration.md)> |  | [optional]
**report_timezone** | Option<**String**> | Timezone Identifier | [optional]
**ui_configuration** | Option<[**crate::models::UiConfiguration**](UIConfiguration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


