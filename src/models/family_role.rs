/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

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

impl std::fmt::Display for FamilyRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Child => write!(f, "Child"),
            Self::Teen => write!(f, "Teen"),
            Self::Adult => write!(f, "Adult"),
        }
    }
}

impl Default for FamilyRole {
    fn default() -> FamilyRole {
        Self::Child
    }
}

