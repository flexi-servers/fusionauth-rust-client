/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.52.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UnknownScopePolicy : Policy for handling unknown OAuth scopes in the request
/// Policy for handling unknown OAuth scopes in the request
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnknownScopePolicy {
    #[serde(rename = "Allow")]
    Allow,
    #[serde(rename = "Remove")]
    Remove,
    #[serde(rename = "Reject")]
    Reject,

}

impl std::fmt::Display for UnknownScopePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Allow => write!(f, "Allow"),
            Self::Remove => write!(f, "Remove"),
            Self::Reject => write!(f, "Reject"),
        }
    }
}

impl Default for UnknownScopePolicy {
    fn default() -> UnknownScopePolicy {
        Self::Allow
    }
}

