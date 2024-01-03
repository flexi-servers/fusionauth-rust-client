# JwtConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token_key_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**id_token_key_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**refresh_token_expiration_policy** | Option<[**crate::models::RefreshTokenExpirationPolicy**](RefreshTokenExpirationPolicy.md)> |  | [optional]
**refresh_token_revocation_policy** | Option<[**crate::models::RefreshTokenRevocationPolicy**](RefreshTokenRevocationPolicy.md)> |  | [optional]
**refresh_token_sliding_window_configuration** | Option<[**crate::models::RefreshTokenSlidingWindowConfiguration**](RefreshTokenSlidingWindowConfiguration.md)> |  | [optional]
**refresh_token_time_to_live_in_minutes** | Option<**i32**> |  | [optional]
**refresh_token_usage_policy** | Option<[**crate::models::RefreshTokenUsagePolicy**](RefreshTokenUsagePolicy.md)> |  | [optional]
**time_to_live_in_seconds** | Option<**i32**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


