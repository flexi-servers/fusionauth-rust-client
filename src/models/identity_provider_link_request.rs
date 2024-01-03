/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityProviderLinkRequest : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderLinkRequest {
    #[serde(rename = "identityProviderLink", skip_serializing_if = "Option::is_none")]
    pub identity_provider_link: Option<Box<crate::models::IdentityProviderLink>>,
    #[serde(rename = "pendingIdPLinkId", skip_serializing_if = "Option::is_none")]
    pub pending_id_p_link_id: Option<String>,
    #[serde(rename = "eventInfo", skip_serializing_if = "Option::is_none")]
    pub event_info: Option<Box<crate::models::EventInfo>>,
}

impl IdentityProviderLinkRequest {
    /// 
    pub fn new() -> IdentityProviderLinkRequest {
        IdentityProviderLinkRequest {
            identity_provider_link: None,
            pending_id_p_link_id: None,
            event_info: None,
        }
    }
}


