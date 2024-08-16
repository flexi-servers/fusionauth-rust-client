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

/// TenantLoginConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantLoginConfiguration {
    #[serde(rename = "requireAuthentication", skip_serializing_if = "Option::is_none")]
    pub require_authentication: Option<bool>,
}

impl TenantLoginConfiguration {
    /// 
    pub fn new() -> TenantLoginConfiguration {
        TenantLoginConfiguration {
            require_authentication: None,
        }
    }
}

