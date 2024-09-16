/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// OAuthScopeConsentMode : Controls the policy for requesting user permission to grant access to requested scopes during an OAuth workflow  for a third-party application.
/// Controls the policy for requesting user permission to grant access to requested scopes during an OAuth workflow  for a third-party application.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OAuthScopeConsentMode {
    #[serde(rename = "AlwaysPrompt")]
    AlwaysPrompt,
    #[serde(rename = "RememberDecision")]
    RememberDecision,
    #[serde(rename = "NeverPrompt")]
    NeverPrompt,

}

impl std::fmt::Display for OAuthScopeConsentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AlwaysPrompt => write!(f, "AlwaysPrompt"),
            Self::RememberDecision => write!(f, "RememberDecision"),
            Self::NeverPrompt => write!(f, "NeverPrompt"),
        }
    }
}

impl Default for OAuthScopeConsentMode {
    fn default() -> OAuthScopeConsentMode {
        Self::AlwaysPrompt
    }
}

