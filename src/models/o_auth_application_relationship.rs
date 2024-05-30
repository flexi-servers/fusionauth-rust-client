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

/// OAuthApplicationRelationship : The application's relationship to the authorization server. First-party applications will be granted implicit permission for requested scopes.  Third-party applications will use the {@link OAuthScopeConsentMode} policy.
/// The application's relationship to the authorization server. First-party applications will be granted implicit permission for requested scopes.  Third-party applications will use the {@link OAuthScopeConsentMode} policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OAuthApplicationRelationship {
    #[serde(rename = "FirstParty")]
    FirstParty,
    #[serde(rename = "ThirdParty")]
    ThirdParty,

}

impl std::fmt::Display for OAuthApplicationRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FirstParty => write!(f, "FirstParty"),
            Self::ThirdParty => write!(f, "ThirdParty"),
        }
    }
}

impl Default for OAuthApplicationRelationship {
    fn default() -> OAuthApplicationRelationship {
        Self::FirstParty
    }
}

