# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**preferred_languages** | Option<**Vec<String>**> |  | [optional]
**active** | Option<**bool**> |  | [optional]
**birth_date** | Option<**String**> | A date without a time-zone in the ISO-8601 calendar system, such as 2007-12-03. | [optional]
**clean_speak_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**email** | Option<**String**> |  | [optional]
**expiry** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**first_name** | Option<**String**> |  | [optional]
**full_name** | Option<**String**> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_name** | Option<**String**> |  | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**middle_name** | Option<**String**> |  | [optional]
**mobile_phone** | Option<**String**> |  | [optional]
**parent_email** | Option<**String**> |  | [optional]
**phone_number** | Option<**String**> |  | [optional]
**tenant_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**timezone** | Option<**String**> | Timezone Identifier | [optional]
**two_factor** | Option<[**models::UserTwoFactorConfiguration**](UserTwoFactorConfiguration.md)> |  | [optional]
**memberships** | Option<[**Vec<models::GroupMember>**](GroupMember.md)> |  | [optional]
**registrations** | Option<[**Vec<models::UserRegistration>**](UserRegistration.md)> |  | [optional]
**identities** | Option<[**Vec<models::UserIdentity>**](UserIdentity.md)> |  | [optional]
**breached_password_last_checked_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**breached_password_status** | Option<[**models::BreachedPasswordStatus**](BreachedPasswordStatus.md)> |  | [optional]
**connector_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**encryption_scheme** | Option<**String**> |  | [optional]
**factor** | Option<**i32**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**last_login_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**password** | Option<**String**> |  | [optional]
**password_change_reason** | Option<[**models::ChangePasswordReason**](ChangePasswordReason.md)> |  | [optional]
**password_change_required** | Option<**bool**> |  | [optional]
**password_last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**salt** | Option<**String**> |  | [optional]
**unique_username** | Option<**String**> |  | [optional]
**username** | Option<**String**> |  | [optional]
**username_status** | Option<[**models::ContentStatus**](ContentStatus.md)> |  | [optional]
**verified** | Option<**bool**> |  | [optional]
**verified_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


