# UserAction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | Option<**bool**> |  | [optional]
**cancel_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**end_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**include_email_in_event_json** | Option<**bool**> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**localized_names** | Option<[**serde_json::Value**](.md)> | Models a set of localized Strings that can be stored as JSON. | [optional]
**modify_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**options** | Option<[**Vec<crate::models::UserActionOption>**](UserActionOption.md)> |  | [optional]
**prevent_login** | Option<**bool**> |  | [optional]
**send_end_event** | Option<**bool**> |  | [optional]
**start_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**temporal** | Option<**bool**> |  | [optional]
**transaction_type** | Option<[**crate::models::TransactionType**](TransactionType.md)> |  | [optional]
**user_emailing_enabled** | Option<**bool**> |  | [optional]
**user_notifications_enabled** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


