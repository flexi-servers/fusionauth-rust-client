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

/// VerificationStrategy : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VerificationStrategy {
    #[serde(rename = "ClickableLink")]
    ClickableLink,
    #[serde(rename = "FormField")]
    FormField,

}

impl std::fmt::Display for VerificationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ClickableLink => write!(f, "ClickableLink"),
            Self::FormField => write!(f, "FormField"),
        }
    }
}

impl Default for VerificationStrategy {
    fn default() -> VerificationStrategy {
        Self::ClickableLink
    }
}

