/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// ConsentStatus : Models a consent.
/// Models a consent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConsentStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Revoked")]
    Revoked,

}

impl ToString for ConsentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("Active"),
            Self::Revoked => String::from("Revoked"),
        }
    }
}

impl Default for ConsentStatus {
    fn default() -> ConsentStatus {
        Self::Active
    }
}

