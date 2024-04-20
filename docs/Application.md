# Application

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_control_configuration** | Option<[**models::ApplicationAccessControlConfiguration**](ApplicationAccessControlConfiguration.md)> |  | [optional]
**active** | Option<**bool**> |  | [optional]
**authentication_token_configuration** | Option<[**models::AuthenticationTokenConfiguration**](AuthenticationTokenConfiguration.md)> |  | [optional]
**clean_speak_configuration** | Option<[**models::CleanSpeakConfiguration**](CleanSpeakConfiguration.md)> |  | [optional]
**data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**email_configuration** | Option<[**models::ApplicationEmailConfiguration**](ApplicationEmailConfiguration.md)> |  | [optional]
**external_identifier_configuration** | Option<[**models::ApplicationExternalIdentifierConfiguration**](ApplicationExternalIdentifierConfiguration.md)> |  | [optional]
**form_configuration** | Option<[**models::ApplicationFormConfiguration**](ApplicationFormConfiguration.md)> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**jwt_configuration** | Option<[**models::JwtConfiguration**](JWTConfiguration.md)> |  | [optional]
**lambda_configuration** | Option<[**models::LambdaConfiguration**](LambdaConfiguration.md)> |  | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**login_configuration** | Option<[**models::LoginConfiguration**](LoginConfiguration.md)> |  | [optional]
**multi_factor_configuration** | Option<[**models::ApplicationMultiFactorConfiguration**](ApplicationMultiFactorConfiguration.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**oauth_configuration** | Option<[**models::OAuth2Configuration**](OAuth2Configuration.md)> |  | [optional]
**passwordless_configuration** | Option<[**models::PasswordlessConfiguration**](PasswordlessConfiguration.md)> |  | [optional]
**registration_configuration** | Option<[**models::RegistrationConfiguration**](RegistrationConfiguration.md)> |  | [optional]
**registration_delete_policy** | Option<[**models::ApplicationRegistrationDeletePolicy**](ApplicationRegistrationDeletePolicy.md)> |  | [optional]
**roles** | Option<[**Vec<models::ApplicationRole>**](ApplicationRole.md)> |  | [optional]
**samlv2_configuration** | Option<[**models::Samlv2Configuration**](SAMLv2Configuration.md)> |  | [optional]
**scopes** | Option<[**Vec<models::ApplicationOAuthScope>**](ApplicationOAuthScope.md)> |  | [optional]
**state** | Option<[**models::ObjectState**](ObjectState.md)> |  | [optional]
**tenant_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**theme_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**unverified** | Option<[**models::RegistrationUnverifiedOptions**](RegistrationUnverifiedOptions.md)> |  | [optional]
**verification_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**verification_strategy** | Option<[**models::VerificationStrategy**](VerificationStrategy.md)> |  | [optional]
**verify_registration** | Option<**bool**> |  | [optional]
**web_authn_configuration** | Option<[**models::ApplicationWebAuthnConfiguration**](ApplicationWebAuthnConfiguration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


