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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FamilyRole {
    #[serde(rename = "Child")]
    Child,
    #[serde(rename = "Teen")]
    Teen,
    #[serde(rename = "Adult")]
    Adult,

}

impl ToString for FamilyRole {
    fn to_string(&self) -> String {
        match self {
            Self::Child => String::from("Child"),
            Self::Teen => String::from("Teen"),
            Self::Adult => String::from("Adult"),
        }
    }
}

impl Default for FamilyRole {
    fn default() -> FamilyRole {
        Self::Child
    }
}

