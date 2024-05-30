/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// RefreshTokenRevocationPolicy : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RefreshTokenRevocationPolicy {
    #[serde(rename = "onLoginPrevented", skip_serializing_if = "Option::is_none")]
    pub on_login_prevented: Option<bool>,
    #[serde(rename = "onMultiFactorEnable", skip_serializing_if = "Option::is_none")]
    pub on_multi_factor_enable: Option<bool>,
    #[serde(rename = "onPasswordChanged", skip_serializing_if = "Option::is_none")]
    pub on_password_changed: Option<bool>,
}

impl RefreshTokenRevocationPolicy {
    /// 
    pub fn new() -> RefreshTokenRevocationPolicy {
        RefreshTokenRevocationPolicy {
            on_login_prevented: None,
            on_multi_factor_enable: None,
            on_password_changed: None,
        }
    }
}

