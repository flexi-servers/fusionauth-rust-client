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

/// TenantRateLimitConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantRateLimitConfiguration {
    #[serde(rename = "failedLogin", skip_serializing_if = "Option::is_none")]
    pub failed_login: Option<Box<models::RateLimitedRequestConfiguration>>,
    #[serde(rename = "forgotPassword", skip_serializing_if = "Option::is_none")]
    pub forgot_password: Option<Box<models::RateLimitedRequestConfiguration>>,
    #[serde(rename = "sendEmailVerification", skip_serializing_if = "Option::is_none")]
    pub send_email_verification: Option<Box<models::RateLimitedRequestConfiguration>>,
    #[serde(rename = "sendPasswordless", skip_serializing_if = "Option::is_none")]
    pub send_passwordless: Option<Box<models::RateLimitedRequestConfiguration>>,
    #[serde(rename = "sendRegistrationVerification", skip_serializing_if = "Option::is_none")]
    pub send_registration_verification: Option<Box<models::RateLimitedRequestConfiguration>>,
    #[serde(rename = "sendTwoFactor", skip_serializing_if = "Option::is_none")]
    pub send_two_factor: Option<Box<models::RateLimitedRequestConfiguration>>,
}

impl TenantRateLimitConfiguration {
    /// 
    pub fn new() -> TenantRateLimitConfiguration {
        TenantRateLimitConfiguration {
            failed_login: None,
            forgot_password: None,
            send_email_verification: None,
            send_passwordless: None,
            send_registration_verification: None,
            send_two_factor: None,
        }
    }
}

