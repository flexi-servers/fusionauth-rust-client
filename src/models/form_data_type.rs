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

impl ToString for FormDataType {
    fn to_string(&self) -> String {
        match self {
            Self::Bool => String::from("bool"),
            Self::Consent => String::from("consent"),
            Self::Date => String::from("date"),
            Self::Email => String::from("email"),
            Self::Number => String::from("number"),
            Self::String => String::from("string"),
        }
    }
}

impl Default for FormDataType {
    fn default() -> FormDataType {
        Self::Bool
    }
}

