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

/// Samlv2DestinationAssertionPolicy : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Samlv2DestinationAssertionPolicy {
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "AllowAlternates")]
    AllowAlternates,

}

impl std::fmt::Display for Samlv2DestinationAssertionPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Enabled => write!(f, "Enabled"),
            Self::Disabled => write!(f, "Disabled"),
            Self::AllowAlternates => write!(f, "AllowAlternates"),
        }
    }
}

impl Default for Samlv2DestinationAssertionPolicy {
    fn default() -> Samlv2DestinationAssertionPolicy {
        Self::Enabled
    }
}

