/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CanonicalizationMethod : XML canonicalization method enumeration. This is used for the IdP and SP side of FusionAuth SAML.
/// XML canonicalization method enumeration. This is used for the IdP and SP side of FusionAuth SAML.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CanonicalizationMethod {
    #[serde(rename = "exclusive")]
    Exclusive,
    #[serde(rename = "exclusive_with_comments")]
    ExclusiveWithComments,
    #[serde(rename = "inclusive")]
    Inclusive,
    #[serde(rename = "inclusive_with_comments")]
    InclusiveWithComments,

}

impl std::fmt::Display for CanonicalizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Exclusive => write!(f, "exclusive"),
            Self::ExclusiveWithComments => write!(f, "exclusive_with_comments"),
            Self::Inclusive => write!(f, "inclusive"),
            Self::InclusiveWithComments => write!(f, "inclusive_with_comments"),
        }
    }
}

impl Default for CanonicalizationMethod {
    fn default() -> CanonicalizationMethod {
        Self::Exclusive
    }
}

