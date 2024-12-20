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

/// SecureGeneratorType : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecureGeneratorType {
    #[serde(rename = "randomDigits")]
    RandomDigits,
    #[serde(rename = "randomBytes")]
    RandomBytes,
    #[serde(rename = "randomAlpha")]
    RandomAlpha,
    #[serde(rename = "randomAlphaNumeric")]
    RandomAlphaNumeric,

}

impl std::fmt::Display for SecureGeneratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::RandomDigits => write!(f, "randomDigits"),
            Self::RandomBytes => write!(f, "randomBytes"),
            Self::RandomAlpha => write!(f, "randomAlpha"),
            Self::RandomAlphaNumeric => write!(f, "randomAlphaNumeric"),
        }
    }
}

impl Default for SecureGeneratorType {
    fn default() -> SecureGeneratorType {
        Self::RandomDigits
    }
}

