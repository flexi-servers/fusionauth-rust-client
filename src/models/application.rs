/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.54.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Application : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "accessControlConfiguration", skip_serializing_if = "Option::is_none")]
    pub access_control_configuration: Option<Box<models::ApplicationAccessControlConfiguration>>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "authenticationTokenConfiguration", skip_serializing_if = "Option::is_none")]
    pub authentication_token_configuration: Option<Box<models::AuthenticationTokenConfiguration>>,
    #[serde(rename = "cleanSpeakConfiguration", skip_serializing_if = "Option::is_none")]
    pub clean_speak_configuration: Option<Box<models::CleanSpeakConfiguration>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "emailConfiguration", skip_serializing_if = "Option::is_none")]
    pub email_configuration: Option<Box<models::ApplicationEmailConfiguration>>,
    #[serde(rename = "externalIdentifierConfiguration", skip_serializing_if = "Option::is_none")]
    pub external_identifier_configuration: Option<Box<models::ApplicationExternalIdentifierConfiguration>>,
    #[serde(rename = "formConfiguration", skip_serializing_if = "Option::is_none")]
    pub form_configuration: Option<Box<models::ApplicationFormConfiguration>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    #[serde(rename = "jwtConfiguration", skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<Box<models::JwtConfiguration>>,
    #[serde(rename = "lambdaConfiguration", skip_serializing_if = "Option::is_none")]
    pub lambda_configuration: Option<Box<models::LambdaConfiguration>>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "loginConfiguration", skip_serializing_if = "Option::is_none")]
    pub login_configuration: Option<Box<models::LoginConfiguration>>,
    #[serde(rename = "multiFactorConfiguration", skip_serializing_if = "Option::is_none")]
    pub multi_factor_configuration: Option<Box<models::ApplicationMultiFactorConfiguration>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "oauthConfiguration", skip_serializing_if = "Option::is_none")]
    pub oauth_configuration: Option<Box<models::OAuth2Configuration>>,
    #[serde(rename = "passwordlessConfiguration", skip_serializing_if = "Option::is_none")]
    pub passwordless_configuration: Option<Box<models::PasswordlessConfiguration>>,
    #[serde(rename = "registrationConfiguration", skip_serializing_if = "Option::is_none")]
    pub registration_configuration: Option<Box<models::RegistrationConfiguration>>,
    #[serde(rename = "registrationDeletePolicy", skip_serializing_if = "Option::is_none")]
    pub registration_delete_policy: Option<Box<models::ApplicationRegistrationDeletePolicy>>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<models::ApplicationRole>>,
    #[serde(rename = "samlv2Configuration", skip_serializing_if = "Option::is_none")]
    pub samlv2_configuration: Option<Box<models::Samlv2Configuration>>,
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<models::ApplicationOAuthScope>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<models::ObjectState>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<uuid::Uuid>,
    #[serde(rename = "themeId", skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<uuid::Uuid>,
    #[serde(rename = "unverified", skip_serializing_if = "Option::is_none")]
    pub unverified: Option<Box<models::RegistrationUnverifiedOptions>>,
    #[serde(rename = "verificationEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub verification_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "verificationStrategy", skip_serializing_if = "Option::is_none")]
    pub verification_strategy: Option<models::VerificationStrategy>,
    #[serde(rename = "verifyRegistration", skip_serializing_if = "Option::is_none")]
    pub verify_registration: Option<bool>,
    #[serde(rename = "webAuthnConfiguration", skip_serializing_if = "Option::is_none")]
    pub web_authn_configuration: Option<Box<models::ApplicationWebAuthnConfiguration>>,
}

impl Application {
    /// 
    pub fn new() -> Application {
        Application {
            access_control_configuration: None,
            active: None,
            authentication_token_configuration: None,
            clean_speak_configuration: None,
            data: None,
            email_configuration: None,
            external_identifier_configuration: None,
            form_configuration: None,
            id: None,
            insert_instant: None,
            jwt_configuration: None,
            lambda_configuration: None,
            last_update_instant: None,
            login_configuration: None,
            multi_factor_configuration: None,
            name: None,
            oauth_configuration: None,
            passwordless_configuration: None,
            registration_configuration: None,
            registration_delete_policy: None,
            roles: None,
            samlv2_configuration: None,
            scopes: None,
            state: None,
            tenant_id: None,
            theme_id: None,
            unverified: None,
            verification_email_template_id: None,
            verification_strategy: None,
            verify_registration: None,
            web_authn_configuration: None,
        }
    }
}

