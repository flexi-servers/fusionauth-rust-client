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

/// TenantUnverifiedConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantUnverifiedConfiguration {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<models::UnverifiedBehavior>,
    #[serde(rename = "whenGated", skip_serializing_if = "Option::is_none")]
    pub when_gated: Option<Box<models::RegistrationUnverifiedOptions>>,
}

impl TenantUnverifiedConfiguration {
    /// 
    pub fn new() -> TenantUnverifiedConfiguration {
        TenantUnverifiedConfiguration {
            email: None,
            when_gated: None,
        }
    }
}

