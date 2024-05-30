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

/// FormDataType : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormDataType {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "consent")]
    Consent,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,

}

impl std::fmt::Display for FormDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::Consent => write!(f, "consent"),
            Self::Date => write!(f, "date"),
            Self::Email => write!(f, "email"),
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),
        }
    }
}

impl Default for FormDataType {
    fn default() -> FormDataType {
        Self::Bool
    }
}

