/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationEmailConfiguration {
    #[serde(rename = "emailUpdateEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub email_update_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "emailVerificationEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub email_verification_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "emailVerifiedEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub email_verified_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "forgotPasswordEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub forgot_password_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "loginIdInUseOnCreateEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub login_id_in_use_on_create_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "loginIdInUseOnUpdateEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub login_id_in_use_on_update_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "loginNewDeviceEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub login_new_device_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "loginSuspiciousEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub login_suspicious_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "passwordResetSuccessEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub password_reset_success_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "passwordUpdateEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub password_update_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "passwordlessEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub passwordless_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "setPasswordEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub set_password_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "twoFactorMethodAddEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub two_factor_method_add_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "twoFactorMethodRemoveEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub two_factor_method_remove_email_template_id: Option<uuid::Uuid>,
}

impl ApplicationEmailConfiguration {
    pub fn new() -> ApplicationEmailConfiguration {
        ApplicationEmailConfiguration {
            email_update_email_template_id: None,
            email_verification_email_template_id: None,
            email_verified_email_template_id: None,
            forgot_password_email_template_id: None,
            login_id_in_use_on_create_email_template_id: None,
            login_id_in_use_on_update_email_template_id: None,
            login_new_device_email_template_id: None,
            login_suspicious_email_template_id: None,
            password_reset_success_email_template_id: None,
            password_update_email_template_id: None,
            passwordless_email_template_id: None,
            set_password_email_template_id: None,
            two_factor_method_add_email_template_id: None,
            two_factor_method_remove_email_template_id: None,
        }
    }
}

