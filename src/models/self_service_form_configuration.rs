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

/// SelfServiceFormConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfServiceFormConfiguration {
    #[serde(rename = "requireCurrentPasswordOnPasswordChange", skip_serializing_if = "Option::is_none")]
    pub require_current_password_on_password_change: Option<bool>,
}

impl SelfServiceFormConfiguration {
    /// 
    pub fn new() -> SelfServiceFormConfiguration {
        SelfServiceFormConfiguration {
            require_current_password_on_password_change: None,
        }
    }
}

