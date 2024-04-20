/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

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

impl ToString for MultiFactorLoginPolicy {
    fn to_string(&self) -> String {
        match self {
            Self::Disabled => String::from("Disabled"),
            Self::Enabled => String::from("Enabled"),
            Self::Required => String::from("Required"),
        }
    }
}

impl Default for MultiFactorLoginPolicy {
    fn default() -> MultiFactorLoginPolicy {
        Self::Disabled
    }
}

