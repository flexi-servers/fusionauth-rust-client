/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// TenantUserDeletePolicy : A Tenant-level policy for deleting Users.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantUserDeletePolicy {
    #[serde(rename = "unverified", skip_serializing_if = "Option::is_none")]
    pub unverified: Option<Box<models::TimeBasedDeletePolicy>>,
}

impl TenantUserDeletePolicy {
    /// A Tenant-level policy for deleting Users.
    pub fn new() -> TenantUserDeletePolicy {
        TenantUserDeletePolicy {
            unverified: None,
        }
    }
}

