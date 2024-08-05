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

/// TwoFactorMethod : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorMethod {
    #[serde(rename = "authenticator", skip_serializing_if = "Option::is_none")]
    pub authenticator: Option<Box<models::AuthenticatorConfiguration>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUsed", skip_serializing_if = "Option::is_none")]
    pub last_used: Option<bool>,
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "mobilePhone", skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl TwoFactorMethod {
    /// 
    pub fn new() -> TwoFactorMethod {
        TwoFactorMethod {
            authenticator: None,
            email: None,
            id: None,
            last_used: None,
            method: None,
            mobile_phone: None,
            secret: None,
        }
    }
}

