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

/// TenantResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantResponse {
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Box<models::Tenant>>,
    #[serde(rename = "tenants", skip_serializing_if = "Option::is_none")]
    pub tenants: Option<Vec<models::Tenant>>,
}

impl TenantResponse {
    /// 
    pub fn new() -> TenantResponse {
        TenantResponse {
            tenant: None,
            tenants: None,
        }
    }
}

