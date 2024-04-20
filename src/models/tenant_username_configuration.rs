/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// TenantUsernameConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantUsernameConfiguration {
    #[serde(rename = "unique", skip_serializing_if = "Option::is_none")]
    pub unique: Option<Box<models::UniqueUsernameConfiguration>>,
}

impl TenantUsernameConfiguration {
    /// 
    pub fn new() -> TenantUsernameConfiguration {
        TenantUsernameConfiguration {
            unique: None,
        }
    }
}

