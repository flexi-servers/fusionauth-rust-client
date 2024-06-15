/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IdentityProviderLinkResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderLinkResponse {
    #[serde(rename = "identityProviderLink", skip_serializing_if = "Option::is_none")]
    pub identity_provider_link: Option<Box<models::IdentityProviderLink>>,
    #[serde(rename = "identityProviderLinks", skip_serializing_if = "Option::is_none")]
    pub identity_provider_links: Option<Vec<models::IdentityProviderLink>>,
}

impl IdentityProviderLinkResponse {
    /// 
    pub fn new() -> IdentityProviderLinkResponse {
        IdentityProviderLinkResponse {
            identity_provider_link: None,
            identity_provider_links: None,
        }
    }
}

