/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ApplicationExternalIdentifierConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationExternalIdentifierConfiguration {
    #[serde(rename = "twoFactorTrustIdTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub two_factor_trust_id_time_to_live_in_seconds: Option<i32>,
}

impl ApplicationExternalIdentifierConfiguration {
    /// 
    pub fn new() -> ApplicationExternalIdentifierConfiguration {
        ApplicationExternalIdentifierConfiguration {
            two_factor_trust_id_time_to_live_in_seconds: None,
        }
    }
}

