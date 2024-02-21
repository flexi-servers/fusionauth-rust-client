/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityProviderPendingLinkResponse : 



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderPendingLinkResponse {
    #[serde(rename = "identityProviderTenantConfiguration", skip_serializing_if = "Option::is_none")]
    pub identity_provider_tenant_configuration: Option<Box<crate::models::IdentityProviderTenantConfiguration>>,
    #[serde(rename = "linkCount", skip_serializing_if = "Option::is_none")]
    pub link_count: Option<i32>,
    #[serde(rename = "pendingIdPLink", skip_serializing_if = "Option::is_none")]
    pub pending_id_p_link: Option<Box<crate::models::PendingIdPLink>>,
}

impl IdentityProviderPendingLinkResponse {
    /// 
    pub fn new() -> IdentityProviderPendingLinkResponse {
        IdentityProviderPendingLinkResponse {
            identity_provider_tenant_configuration: None,
            link_count: None,
            pending_id_p_link: None,
        }
    }
}


