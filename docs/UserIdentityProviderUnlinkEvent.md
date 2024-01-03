# UserIdentityProviderUnlinkEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identity_provider_link** | Option<[**crate::models::IdentityProviderLink**](IdentityProviderLink.md)> |  | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**create_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**info** | Option<[**crate::models::EventInfo**](EventInfo.md)> |  | [optional]
**tenant_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**r#type** | Option<[**crate::models::EventType**](EventType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


