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

/// VerifyEmailResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyEmailResponse {
    #[serde(rename = "oneTimeCode", skip_serializing_if = "Option::is_none")]
    pub one_time_code: Option<String>,
    #[serde(rename = "verificationId", skip_serializing_if = "Option::is_none")]
    pub verification_id: Option<String>,
}

impl VerifyEmailResponse {
    /// 
    pub fn new() -> VerifyEmailResponse {
        VerifyEmailResponse {
            one_time_code: None,
            verification_id: None,
        }
    }
}

