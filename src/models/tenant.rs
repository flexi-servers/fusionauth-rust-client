/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Tenant : 



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tenant {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "accessControlConfiguration", skip_serializing_if = "Option::is_none")]
    pub access_control_configuration: Option<Box<crate::models::TenantAccessControlConfiguration>>,
    #[serde(rename = "captchaConfiguration", skip_serializing_if = "Option::is_none")]
    pub captcha_configuration: Option<Box<crate::models::TenantCaptchaConfiguration>>,
    #[serde(rename = "configured", skip_serializing_if = "Option::is_none")]
    pub configured: Option<bool>,
    #[serde(rename = "connectorPolicies", skip_serializing_if = "Option::is_none")]
    pub connector_policies: Option<Vec<crate::models::ConnectorPolicy>>,
    #[serde(rename = "emailConfiguration", skip_serializing_if = "Option::is_none")]
    pub email_configuration: Option<Box<crate::models::EmailConfiguration>>,
    #[serde(rename = "eventConfiguration", skip_serializing_if = "Option::is_none")]
    pub event_configuration: Option<Box<crate::models::EventConfiguration>>,
    #[serde(rename = "externalIdentifierConfiguration", skip_serializing_if = "Option::is_none")]
    pub external_identifier_configuration: Option<Box<crate::models::ExternalIdentifierConfiguration>>,
    #[serde(rename = "failedAuthenticationConfiguration", skip_serializing_if = "Option::is_none")]
    pub failed_authentication_configuration: Option<Box<crate::models::FailedAuthenticationConfiguration>>,
    #[serde(rename = "familyConfiguration", skip_serializing_if = "Option::is_none")]
    pub family_configuration: Option<Box<crate::models::FamilyConfiguration>>,
    #[serde(rename = "formConfiguration", skip_serializing_if = "Option::is_none")]
    pub form_configuration: Option<Box<crate::models::TenantFormConfiguration>>,
    #[serde(rename = "httpSessionMaxInactiveInterval", skip_serializing_if = "Option::is_none")]
    pub http_session_max_inactive_interval: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "jwtConfiguration", skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<Box<crate::models::JwtConfiguration>>,
    #[serde(rename = "lambdaConfiguration", skip_serializing_if = "Option::is_none")]
    pub lambda_configuration: Option<Box<crate::models::TenantLambdaConfiguration>>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "loginConfiguration", skip_serializing_if = "Option::is_none")]
    pub login_configuration: Option<Box<crate::models::TenantLoginConfiguration>>,
    #[serde(rename = "logoutURL", skip_serializing_if = "Option::is_none")]
    pub logout_url: Option<String>,
    #[serde(rename = "maximumPasswordAge", skip_serializing_if = "Option::is_none")]
    pub maximum_password_age: Option<Box<crate::models::MaximumPasswordAge>>,
    #[serde(rename = "minimumPasswordAge", skip_serializing_if = "Option::is_none")]
    pub minimum_password_age: Option<Box<crate::models::MinimumPasswordAge>>,
    #[serde(rename = "multiFactorConfiguration", skip_serializing_if = "Option::is_none")]
    pub multi_factor_configuration: Option<Box<crate::models::TenantMultiFactorConfiguration>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "oauthConfiguration", skip_serializing_if = "Option::is_none")]
    pub oauth_configuration: Option<Box<crate::models::TenantOAuth2Configuration>>,
    #[serde(rename = "passwordEncryptionConfiguration", skip_serializing_if = "Option::is_none")]
    pub password_encryption_configuration: Option<Box<crate::models::PasswordEncryptionConfiguration>>,
    #[serde(rename = "passwordValidationRules", skip_serializing_if = "Option::is_none")]
    pub password_validation_rules: Option<Box<crate::models::PasswordValidationRules>>,
    #[serde(rename = "rateLimitConfiguration", skip_serializing_if = "Option::is_none")]
    pub rate_limit_configuration: Option<Box<crate::models::TenantRateLimitConfiguration>>,
    #[serde(rename = "registrationConfiguration", skip_serializing_if = "Option::is_none")]
    pub registration_configuration: Option<Box<crate::models::TenantRegistrationConfiguration>>,
    #[serde(rename = "scimServerConfiguration", skip_serializing_if = "Option::is_none")]
    pub scim_server_configuration: Option<Box<crate::models::TenantScimServerConfiguration>>,
    #[serde(rename = "ssoConfiguration", skip_serializing_if = "Option::is_none")]
    pub sso_configuration: Option<Box<crate::models::TenantSsoConfiguration>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::ObjectState>,
    #[serde(rename = "themeId", skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<uuid::Uuid>,
    #[serde(rename = "userDeletePolicy", skip_serializing_if = "Option::is_none")]
    pub user_delete_policy: Option<Box<crate::models::TenantUserDeletePolicy>>,
    #[serde(rename = "usernameConfiguration", skip_serializing_if = "Option::is_none")]
    pub username_configuration: Option<Box<crate::models::TenantUsernameConfiguration>>,
    #[serde(rename = "webAuthnConfiguration", skip_serializing_if = "Option::is_none")]
    pub web_authn_configuration: Option<Box<crate::models::TenantWebAuthnConfiguration>>,
}

impl Tenant {
    /// 
    pub fn new() -> Tenant {
        Tenant {
            data: None,
            access_control_configuration: None,
            captcha_configuration: None,
            configured: None,
            connector_policies: None,
            email_configuration: None,
            event_configuration: None,
            external_identifier_configuration: None,
            failed_authentication_configuration: None,
            family_configuration: None,
            form_configuration: None,
            http_session_max_inactive_interval: None,
            id: None,
            insert_instant: None,
            issuer: None,
            jwt_configuration: None,
            lambda_configuration: None,
            last_update_instant: None,
            login_configuration: None,
            logout_url: None,
            maximum_password_age: None,
            minimum_password_age: None,
            multi_factor_configuration: None,
            name: None,
            oauth_configuration: None,
            password_encryption_configuration: None,
            password_validation_rules: None,
            rate_limit_configuration: None,
            registration_configuration: None,
            scim_server_configuration: None,
            sso_configuration: None,
            state: None,
            theme_id: None,
            user_delete_policy: None,
            username_configuration: None,
            web_authn_configuration: None,
        }
    }
}


