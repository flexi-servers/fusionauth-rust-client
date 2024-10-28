/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Templates {
    #[serde(rename = "accountEdit", skip_serializing_if = "Option::is_none")]
    pub account_edit: Option<String>,
    #[serde(rename = "accountIndex", skip_serializing_if = "Option::is_none")]
    pub account_index: Option<String>,
    #[serde(rename = "accountTwoFactorDisable", skip_serializing_if = "Option::is_none")]
    pub account_two_factor_disable: Option<String>,
    #[serde(rename = "accountTwoFactorEnable", skip_serializing_if = "Option::is_none")]
    pub account_two_factor_enable: Option<String>,
    #[serde(rename = "accountTwoFactorIndex", skip_serializing_if = "Option::is_none")]
    pub account_two_factor_index: Option<String>,
    #[serde(rename = "accountWebAuthnAdd", skip_serializing_if = "Option::is_none")]
    pub account_web_authn_add: Option<String>,
    #[serde(rename = "accountWebAuthnDelete", skip_serializing_if = "Option::is_none")]
    pub account_web_authn_delete: Option<String>,
    #[serde(rename = "accountWebAuthnIndex", skip_serializing_if = "Option::is_none")]
    pub account_web_authn_index: Option<String>,
    #[serde(rename = "confirmationRequired", skip_serializing_if = "Option::is_none")]
    pub confirmation_required: Option<String>,
    #[serde(rename = "emailComplete", skip_serializing_if = "Option::is_none")]
    pub email_complete: Option<String>,
    #[serde(rename = "emailSent", skip_serializing_if = "Option::is_none")]
    pub email_sent: Option<String>,
    #[serde(rename = "emailVerificationRequired", skip_serializing_if = "Option::is_none")]
    pub email_verification_required: Option<String>,
    #[serde(rename = "emailVerify", skip_serializing_if = "Option::is_none")]
    pub email_verify: Option<String>,
    #[serde(rename = "helpers", skip_serializing_if = "Option::is_none")]
    pub helpers: Option<String>,
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(rename = "oauth2Authorize", skip_serializing_if = "Option::is_none")]
    pub oauth2_authorize: Option<String>,
    #[serde(rename = "oauth2AuthorizedNotRegistered", skip_serializing_if = "Option::is_none")]
    pub oauth2_authorized_not_registered: Option<String>,
    #[serde(rename = "oauth2ChildRegistrationNotAllowed", skip_serializing_if = "Option::is_none")]
    pub oauth2_child_registration_not_allowed: Option<String>,
    #[serde(rename = "oauth2ChildRegistrationNotAllowedComplete", skip_serializing_if = "Option::is_none")]
    pub oauth2_child_registration_not_allowed_complete: Option<String>,
    #[serde(rename = "oauth2CompleteRegistration", skip_serializing_if = "Option::is_none")]
    pub oauth2_complete_registration: Option<String>,
    #[serde(rename = "oauth2Consent", skip_serializing_if = "Option::is_none")]
    pub oauth2_consent: Option<String>,
    #[serde(rename = "oauth2Device", skip_serializing_if = "Option::is_none")]
    pub oauth2_device: Option<String>,
    #[serde(rename = "oauth2DeviceComplete", skip_serializing_if = "Option::is_none")]
    pub oauth2_device_complete: Option<String>,
    #[serde(rename = "oauth2Error", skip_serializing_if = "Option::is_none")]
    pub oauth2_error: Option<String>,
    #[serde(rename = "oauth2Logout", skip_serializing_if = "Option::is_none")]
    pub oauth2_logout: Option<String>,
    #[serde(rename = "oauth2Passwordless", skip_serializing_if = "Option::is_none")]
    pub oauth2_passwordless: Option<String>,
    #[serde(rename = "oauth2Register", skip_serializing_if = "Option::is_none")]
    pub oauth2_register: Option<String>,
    #[serde(rename = "oauth2StartIdPLink", skip_serializing_if = "Option::is_none")]
    pub oauth2_start_id_p_link: Option<String>,
    #[serde(rename = "oauth2TwoFactor", skip_serializing_if = "Option::is_none")]
    pub oauth2_two_factor: Option<String>,
    #[serde(rename = "oauth2TwoFactorEnable", skip_serializing_if = "Option::is_none")]
    pub oauth2_two_factor_enable: Option<String>,
    #[serde(rename = "oauth2TwoFactorEnableComplete", skip_serializing_if = "Option::is_none")]
    pub oauth2_two_factor_enable_complete: Option<String>,
    #[serde(rename = "oauth2TwoFactorMethods", skip_serializing_if = "Option::is_none")]
    pub oauth2_two_factor_methods: Option<String>,
    #[serde(rename = "oauth2Wait", skip_serializing_if = "Option::is_none")]
    pub oauth2_wait: Option<String>,
    #[serde(rename = "oauth2WebAuthn", skip_serializing_if = "Option::is_none")]
    pub oauth2_web_authn: Option<String>,
    #[serde(rename = "oauth2WebAuthnReauth", skip_serializing_if = "Option::is_none")]
    pub oauth2_web_authn_reauth: Option<String>,
    #[serde(rename = "oauth2WebAuthnReauthEnable", skip_serializing_if = "Option::is_none")]
    pub oauth2_web_authn_reauth_enable: Option<String>,
    #[serde(rename = "passwordChange", skip_serializing_if = "Option::is_none")]
    pub password_change: Option<String>,
    #[serde(rename = "passwordComplete", skip_serializing_if = "Option::is_none")]
    pub password_complete: Option<String>,
    #[serde(rename = "passwordForgot", skip_serializing_if = "Option::is_none")]
    pub password_forgot: Option<String>,
    #[serde(rename = "passwordSent", skip_serializing_if = "Option::is_none")]
    pub password_sent: Option<String>,
    #[serde(rename = "registrationComplete", skip_serializing_if = "Option::is_none")]
    pub registration_complete: Option<String>,
    #[serde(rename = "registrationSent", skip_serializing_if = "Option::is_none")]
    pub registration_sent: Option<String>,
    #[serde(rename = "registrationVerificationRequired", skip_serializing_if = "Option::is_none")]
    pub registration_verification_required: Option<String>,
    #[serde(rename = "registrationVerify", skip_serializing_if = "Option::is_none")]
    pub registration_verify: Option<String>,
    #[serde(rename = "samlv2Logout", skip_serializing_if = "Option::is_none")]
    pub samlv2_logout: Option<String>,
    #[serde(rename = "unauthorized", skip_serializing_if = "Option::is_none")]
    pub unauthorized: Option<String>,
    #[serde(rename = "emailSend", skip_serializing_if = "Option::is_none")]
    pub email_send: Option<String>,
    #[serde(rename = "registrationSend", skip_serializing_if = "Option::is_none")]
    pub registration_send: Option<String>,
}

impl Templates {
    pub fn new() -> Templates {
        Templates {
            account_edit: None,
            account_index: None,
            account_two_factor_disable: None,
            account_two_factor_enable: None,
            account_two_factor_index: None,
            account_web_authn_add: None,
            account_web_authn_delete: None,
            account_web_authn_index: None,
            confirmation_required: None,
            email_complete: None,
            email_sent: None,
            email_verification_required: None,
            email_verify: None,
            helpers: None,
            index: None,
            oauth2_authorize: None,
            oauth2_authorized_not_registered: None,
            oauth2_child_registration_not_allowed: None,
            oauth2_child_registration_not_allowed_complete: None,
            oauth2_complete_registration: None,
            oauth2_consent: None,
            oauth2_device: None,
            oauth2_device_complete: None,
            oauth2_error: None,
            oauth2_logout: None,
            oauth2_passwordless: None,
            oauth2_register: None,
            oauth2_start_id_p_link: None,
            oauth2_two_factor: None,
            oauth2_two_factor_enable: None,
            oauth2_two_factor_enable_complete: None,
            oauth2_two_factor_methods: None,
            oauth2_wait: None,
            oauth2_web_authn: None,
            oauth2_web_authn_reauth: None,
            oauth2_web_authn_reauth_enable: None,
            password_change: None,
            password_complete: None,
            password_forgot: None,
            password_sent: None,
            registration_complete: None,
            registration_sent: None,
            registration_verification_required: None,
            registration_verify: None,
            samlv2_logout: None,
            unauthorized: None,
            email_send: None,
            registration_send: None,
        }
    }
}

