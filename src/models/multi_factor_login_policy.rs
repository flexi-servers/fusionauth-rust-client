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

/// MultiFactorLoginPolicy : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MultiFactorLoginPolicy {
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "Required")]
    Required,

}

impl std::fmt::Display for MultiFactorLoginPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Disabled => write!(f, "Disabled"),
            Self::Enabled => write!(f, "Enabled"),
            Self::Required => write!(f, "Required"),
        }
    }
}

impl Default for MultiFactorLoginPolicy {
    fn default() -> MultiFactorLoginPolicy {
        Self::Disabled
    }
}

