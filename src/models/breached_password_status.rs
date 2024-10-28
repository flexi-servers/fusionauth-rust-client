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

/// BreachedPasswordStatus : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BreachedPasswordStatus {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "ExactMatch")]
    ExactMatch,
    #[serde(rename = "SubAddressMatch")]
    SubAddressMatch,
    #[serde(rename = "PasswordOnly")]
    PasswordOnly,
    #[serde(rename = "CommonPassword")]
    CommonPassword,

}

impl std::fmt::Display for BreachedPasswordStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::ExactMatch => write!(f, "ExactMatch"),
            Self::SubAddressMatch => write!(f, "SubAddressMatch"),
            Self::PasswordOnly => write!(f, "PasswordOnly"),
            Self::CommonPassword => write!(f, "CommonPassword"),
        }
    }
}

impl Default for BreachedPasswordStatus {
    fn default() -> BreachedPasswordStatus {
        Self::None
    }
}

