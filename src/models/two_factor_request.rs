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

/// TwoFactorRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorRequest {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "authenticatorId", skip_serializing_if = "Option::is_none")]
    pub authenticator_id: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "mobilePhone", skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "secretBase32Encoded", skip_serializing_if = "Option::is_none")]
    pub secret_base32_encoded: Option<String>,
    #[serde(rename = "twoFactorId", skip_serializing_if = "Option::is_none")]
    pub two_factor_id: Option<String>,
    #[serde(rename = "eventInfo", skip_serializing_if = "Option::is_none")]
    pub event_info: Option<Box<models::EventInfo>>,
}

impl TwoFactorRequest {
    /// 
    pub fn new() -> TwoFactorRequest {
        TwoFactorRequest {
            application_id: None,
            authenticator_id: None,
            code: None,
            email: None,
            method: None,
            mobile_phone: None,
            secret: None,
            secret_base32_encoded: None,
            two_factor_id: None,
            event_info: None,
        }
    }
}

