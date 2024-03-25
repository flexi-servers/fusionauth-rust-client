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

/// ApplicationMultiFactorTrustPolicy : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApplicationMultiFactorTrustPolicy {
    #[serde(rename = "Any")]
    Any,
    #[serde(rename = "This")]
    This,
    #[serde(rename = "None")]
    None,

}

impl ToString for ApplicationMultiFactorTrustPolicy {
    fn to_string(&self) -> String {
        match self {
            Self::Any => String::from("Any"),
            Self::This => String::from("This"),
            Self::None => String::from("None"),
        }
    }
}

impl Default for ApplicationMultiFactorTrustPolicy {
    fn default() -> ApplicationMultiFactorTrustPolicy {
        Self::Any
    }
}

