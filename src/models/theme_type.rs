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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ThemeType {
    #[serde(rename = "advanced")]
    Advanced,
    #[serde(rename = "simple")]
    Simple,

}

impl std::fmt::Display for ThemeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Advanced => write!(f, "advanced"),
            Self::Simple => write!(f, "simple"),
        }
    }
}

impl Default for ThemeType {
    fn default() -> ThemeType {
        Self::Advanced
    }
}
