/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

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

impl ToString for SecureGeneratorType {
    fn to_string(&self) -> String {
        match self {
            Self::RandomDigits => String::from("randomDigits"),
            Self::RandomBytes => String::from("randomBytes"),
            Self::RandomAlpha => String::from("randomAlpha"),
            Self::RandomAlphaNumeric => String::from("randomAlphaNumeric"),
        }
    }
}

impl Default for SecureGeneratorType {
    fn default() -> SecureGeneratorType {
        Self::RandomDigits
    }
}

