/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// IdentityProviderTenantConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderTenantConfiguration {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "limitUserLinkCount", skip_serializing_if = "Option::is_none")]
    pub limit_user_link_count: Option<Box<models::IdentityProviderLimitUserLinkingPolicy>>,
}

impl IdentityProviderTenantConfiguration {
    /// 
    pub fn new() -> IdentityProviderTenantConfiguration {
        IdentityProviderTenantConfiguration {
            data: None,
            limit_user_link_count: None,
        }
    }
}

