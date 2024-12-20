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

/// Oauth2AuthorizedUrlValidationPolicy : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Oauth2AuthorizedUrlValidationPolicy {
    #[serde(rename = "AllowWildcards")]
    AllowWildcards,
    #[serde(rename = "ExactMatch")]
    ExactMatch,

}

impl std::fmt::Display for Oauth2AuthorizedUrlValidationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AllowWildcards => write!(f, "AllowWildcards"),
            Self::ExactMatch => write!(f, "ExactMatch"),
        }
    }
}

impl Default for Oauth2AuthorizedUrlValidationPolicy {
    fn default() -> Oauth2AuthorizedUrlValidationPolicy {
        Self::AllowWildcards
    }
}

