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

/// OAuthScopeHandlingPolicy : Controls the policy for whether OAuth workflows will more strictly adhere to the OAuth and OIDC specification  or run in backwards compatibility mode.
/// Controls the policy for whether OAuth workflows will more strictly adhere to the OAuth and OIDC specification  or run in backwards compatibility mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OAuthScopeHandlingPolicy {
    #[serde(rename = "Compatibility")]
    Compatibility,
    #[serde(rename = "Strict")]
    Strict,

}

impl std::fmt::Display for OAuthScopeHandlingPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Compatibility => write!(f, "Compatibility"),
            Self::Strict => write!(f, "Strict"),
        }
    }
}

impl Default for OAuthScopeHandlingPolicy {
    fn default() -> OAuthScopeHandlingPolicy {
        Self::Compatibility
    }
}

