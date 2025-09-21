# Tenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**access_control_configuration** | Option<[**models::TenantAccessControlConfiguration**](TenantAccessControlConfiguration.md)> |  | [optional]
**captcha_configuration** | Option<[**models::TenantCaptchaConfiguration**](TenantCaptchaConfiguration.md)> |  | [optional]
**configured** | Option<**bool**> |  | [optional]
**connector_policies** | Option<[**Vec<models::ConnectorPolicy>**](ConnectorPolicy.md)> |  | [optional]
**email_configuration** | Option<[**models::EmailConfiguration**](EmailConfiguration.md)> |  | [optional]
**event_configuration** | Option<[**models::EventConfiguration**](EventConfiguration.md)> |  | [optional]
**external_identifier_configuration** | Option<[**models::ExternalIdentifierConfiguration**](ExternalIdentifierConfiguration.md)> |  | [optional]
**failed_authentication_configuration** | Option<[**models::FailedAuthenticationConfiguration**](FailedAuthenticationConfiguration.md)> |  | [optional]
**family_configuration** | Option<[**models::FamilyConfiguration**](FamilyConfiguration.md)> |  | [optional]
**form_configuration** | Option<[**models::TenantFormConfiguration**](TenantFormConfiguration.md)> |  | [optional]
**http_session_max_inactive_interval** | Option<**i32**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**issuer** | Option<**String**> |  | [optional]
**jwt_configuration** | Option<[**models::JwtConfiguration**](JWTConfiguration.md)> |  | [optional]
**lambda_configuration** | Option<[**models::TenantLambdaConfiguration**](TenantLambdaConfiguration.md)> |  | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**login_configuration** | Option<[**models::TenantLoginConfiguration**](TenantLoginConfiguration.md)> |  | [optional]
**logout_url** | Option<**String**> |  | [optional]
**maximum_password_age** | Option<[**models::MaximumPasswordAge**](MaximumPasswordAge.md)> |  | [optional]
**minimum_password_age** | Option<[**models::MinimumPasswordAge**](MinimumPasswordAge.md)> |  | [optional]
**multi_factor_configuration** | Option<[**models::TenantMultiFactorConfiguration**](TenantMultiFactorConfiguration.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**oauth_configuration** | Option<[**models::TenantOAuth2Configuration**](TenantOAuth2Configuration.md)> |  | [optional]
**password_encryption_configuration** | Option<[**models::PasswordEncryptionConfiguration**](PasswordEncryptionConfiguration.md)> |  | [optional]
**password_validation_rules** | Option<[**models::PasswordValidationRules**](PasswordValidationRules.md)> |  | [optional]
**phone_configuration** | Option<[**models::TenantPhoneConfiguration**](TenantPhoneConfiguration.md)> |  | [optional]
**rate_limit_configuration** | Option<[**models::TenantRateLimitConfiguration**](TenantRateLimitConfiguration.md)> |  | [optional]
**registration_configuration** | Option<[**models::TenantRegistrationConfiguration**](TenantRegistrationConfiguration.md)> |  | [optional]
**scim_server_configuration** | Option<[**models::TenantScimServerConfiguration**](TenantSCIMServerConfiguration.md)> |  | [optional]
**sso_configuration** | Option<[**models::TenantSsoConfiguration**](TenantSSOConfiguration.md)> |  | [optional]
**state** | Option<[**models::ObjectState**](ObjectState.md)> |  | [optional]
**theme_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**user_delete_policy** | Option<[**models::TenantUserDeletePolicy**](TenantUserDeletePolicy.md)> |  | [optional]
**username_configuration** | Option<[**models::TenantUsernameConfiguration**](TenantUsernameConfiguration.md)> |  | [optional]
**web_authn_configuration** | Option<[**models::TenantWebAuthnConfiguration**](TenantWebAuthnConfiguration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


