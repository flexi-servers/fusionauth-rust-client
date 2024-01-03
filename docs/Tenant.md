# Tenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**access_control_configuration** | Option<[**crate::models::TenantAccessControlConfiguration**](TenantAccessControlConfiguration.md)> |  | [optional]
**captcha_configuration** | Option<[**crate::models::TenantCaptchaConfiguration**](TenantCaptchaConfiguration.md)> |  | [optional]
**configured** | Option<**bool**> |  | [optional]
**connector_policies** | Option<[**Vec<crate::models::ConnectorPolicy>**](ConnectorPolicy.md)> |  | [optional]
**email_configuration** | Option<[**crate::models::EmailConfiguration**](EmailConfiguration.md)> |  | [optional]
**event_configuration** | Option<[**crate::models::EventConfiguration**](EventConfiguration.md)> |  | [optional]
**external_identifier_configuration** | Option<[**crate::models::ExternalIdentifierConfiguration**](ExternalIdentifierConfiguration.md)> |  | [optional]
**failed_authentication_configuration** | Option<[**crate::models::FailedAuthenticationConfiguration**](FailedAuthenticationConfiguration.md)> |  | [optional]
**family_configuration** | Option<[**crate::models::FamilyConfiguration**](FamilyConfiguration.md)> |  | [optional]
**form_configuration** | Option<[**crate::models::TenantFormConfiguration**](TenantFormConfiguration.md)> |  | [optional]
**http_session_max_inactive_interval** | Option<**i32**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**issuer** | Option<**String**> |  | [optional]
**jwt_configuration** | Option<[**crate::models::JwtConfiguration**](JWTConfiguration.md)> |  | [optional]
**lambda_configuration** | Option<[**crate::models::TenantLambdaConfiguration**](TenantLambdaConfiguration.md)> |  | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**login_configuration** | Option<[**crate::models::TenantLoginConfiguration**](TenantLoginConfiguration.md)> |  | [optional]
**logout_url** | Option<**String**> |  | [optional]
**maximum_password_age** | Option<[**crate::models::MaximumPasswordAge**](MaximumPasswordAge.md)> |  | [optional]
**minimum_password_age** | Option<[**crate::models::MinimumPasswordAge**](MinimumPasswordAge.md)> |  | [optional]
**multi_factor_configuration** | Option<[**crate::models::TenantMultiFactorConfiguration**](TenantMultiFactorConfiguration.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**oauth_configuration** | Option<[**crate::models::TenantOAuth2Configuration**](TenantOAuth2Configuration.md)> |  | [optional]
**password_encryption_configuration** | Option<[**crate::models::PasswordEncryptionConfiguration**](PasswordEncryptionConfiguration.md)> |  | [optional]
**password_validation_rules** | Option<[**crate::models::PasswordValidationRules**](PasswordValidationRules.md)> |  | [optional]
**rate_limit_configuration** | Option<[**crate::models::TenantRateLimitConfiguration**](TenantRateLimitConfiguration.md)> |  | [optional]
**registration_configuration** | Option<[**crate::models::TenantRegistrationConfiguration**](TenantRegistrationConfiguration.md)> |  | [optional]
**scim_server_configuration** | Option<[**crate::models::TenantScimServerConfiguration**](TenantSCIMServerConfiguration.md)> |  | [optional]
**sso_configuration** | Option<[**crate::models::TenantSsoConfiguration**](TenantSSOConfiguration.md)> |  | [optional]
**state** | Option<[**crate::models::ObjectState**](ObjectState.md)> |  | [optional]
**theme_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**user_delete_policy** | Option<[**crate::models::TenantUserDeletePolicy**](TenantUserDeletePolicy.md)> |  | [optional]
**username_configuration** | Option<[**crate::models::TenantUsernameConfiguration**](TenantUsernameConfiguration.md)> |  | [optional]
**web_authn_configuration** | Option<[**crate::models::TenantWebAuthnConfiguration**](TenantWebAuthnConfiguration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


