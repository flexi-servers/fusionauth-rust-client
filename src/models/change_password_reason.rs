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

/// ChangePasswordReason : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChangePasswordReason {
    #[serde(rename = "Administrative")]
    Administrative,
    #[serde(rename = "Breached")]
    Breached,
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "Validation")]
    Validation,

}

impl ToString for ChangePasswordReason {
    fn to_string(&self) -> String {
        match self {
            Self::Administrative => String::from("Administrative"),
            Self::Breached => String::from("Breached"),
            Self::Expired => String::from("Expired"),
            Self::Validation => String::from("Validation"),
        }
    }
}

impl Default for ChangePasswordReason {
    fn default() -> ChangePasswordReason {
        Self::Administrative
    }
}

