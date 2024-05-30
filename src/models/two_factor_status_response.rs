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

/// TwoFactorStatusResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorStatusResponse {
    #[serde(rename = "trusts", skip_serializing_if = "Option::is_none")]
    pub trusts: Option<Vec<models::TwoFactorTrust>>,
    #[serde(rename = "twoFactorTrustId", skip_serializing_if = "Option::is_none")]
    pub two_factor_trust_id: Option<String>,
}

impl TwoFactorStatusResponse {
    /// 
    pub fn new() -> TwoFactorStatusResponse {
        TwoFactorStatusResponse {
            trusts: None,
            two_factor_trust_id: None,
        }
    }
}

