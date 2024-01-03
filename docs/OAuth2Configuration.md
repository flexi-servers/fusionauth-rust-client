# OAuth2Configuration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authorized_origin_urls** | Option<**Vec<String>**> |  | [optional]
**authorized_redirect_urls** | Option<**Vec<String>**> |  | [optional]
**authorized_url_validation_policy** | Option<[**crate::models::Oauth2AuthorizedUrlValidationPolicy**](Oauth2AuthorizedURLValidationPolicy.md)> |  | [optional]
**client_authentication_policy** | Option<[**crate::models::ClientAuthenticationPolicy**](ClientAuthenticationPolicy.md)> |  | [optional]
**client_id** | Option<**String**> |  | [optional]
**client_secret** | Option<**String**> |  | [optional]
**debug** | Option<**bool**> |  | [optional]
**device_verification_url** | Option<**String**> |  | [optional]
**enabled_grants** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**generate_refresh_tokens** | Option<**bool**> |  | [optional]
**logout_behavior** | Option<[**crate::models::LogoutBehavior**](LogoutBehavior.md)> |  | [optional]
**logout_url** | Option<**String**> |  | [optional]
**proof_key_for_code_exchange_policy** | Option<[**crate::models::ProofKeyForCodeExchangePolicy**](ProofKeyForCodeExchangePolicy.md)> |  | [optional]
**require_client_authentication** | Option<**bool**> |  | [optional]
**require_registration** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


