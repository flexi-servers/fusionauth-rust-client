/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.52.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EmailConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailConfiguration {
    #[serde(rename = "additionalHeaders", skip_serializing_if = "Option::is_none")]
    pub additional_headers: Option<Vec<models::EmailHeader>>,
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(rename = "defaultFromEmail", skip_serializing_if = "Option::is_none")]
    pub default_from_email: Option<String>,
    #[serde(rename = "defaultFromName", skip_serializing_if = "Option::is_none")]
    pub default_from_name: Option<String>,
    #[serde(rename = "emailUpdateEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub email_update_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "emailVerifiedEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub email_verified_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "forgotPasswordEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub forgot_password_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "implicitEmailVerificationAllowed", skip_serializing_if = "Option::is_none")]
    pub implicit_email_verification_allowed: Option<bool>,
    #[serde(rename = "loginIdInUseOnCreateEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub login_id_in_use_on_create_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "loginIdInUseOnUpdateEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub login_id_in_use_on_update_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "loginNewDeviceEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub login_new_device_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "loginSuspiciousEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub login_suspicious_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "passwordResetSuccessEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub password_reset_success_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "passwordUpdateEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub password_update_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "passwordlessEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub passwordless_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[serde(rename = "security", skip_serializing_if = "Option::is_none")]
    pub security: Option<models::EmailSecurityType>,
    #[serde(rename = "setPasswordEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub set_password_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "twoFactorMethodAddEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub two_factor_method_add_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "twoFactorMethodRemoveEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub two_factor_method_remove_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "unverified", skip_serializing_if = "Option::is_none")]
    pub unverified: Option<Box<models::EmailUnverifiedOptions>>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "verificationEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub verification_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "verificationStrategy", skip_serializing_if = "Option::is_none")]
    pub verification_strategy: Option<models::VerificationStrategy>,
    #[serde(rename = "verifyEmail", skip_serializing_if = "Option::is_none")]
    pub verify_email: Option<bool>,
    #[serde(rename = "verifyEmailWhenChanged", skip_serializing_if = "Option::is_none")]
    pub verify_email_when_changed: Option<bool>,
}

impl EmailConfiguration {
    /// 
    pub fn new() -> EmailConfiguration {
        EmailConfiguration {
            additional_headers: None,
            debug: None,
            default_from_email: None,
            default_from_name: None,
            email_update_email_template_id: None,
            email_verified_email_template_id: None,
            forgot_password_email_template_id: None,
            host: None,
            implicit_email_verification_allowed: None,
            login_id_in_use_on_create_email_template_id: None,
            login_id_in_use_on_update_email_template_id: None,
            login_new_device_email_template_id: None,
            login_suspicious_email_template_id: None,
            password: None,
            password_reset_success_email_template_id: None,
            password_update_email_template_id: None,
            passwordless_email_template_id: None,
            port: None,
            properties: None,
            security: None,
            set_password_email_template_id: None,
            two_factor_method_add_email_template_id: None,
            two_factor_method_remove_email_template_id: None,
            unverified: None,
            username: None,
            verification_email_template_id: None,
            verification_strategy: None,
            verify_email: None,
            verify_email_when_changed: None,
        }
    }
}

