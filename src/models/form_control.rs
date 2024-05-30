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

/// FormControl : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormControl {
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "textarea")]
    Textarea,
    #[serde(rename = "text")]
    Text,

}

impl std::fmt::Display for FormControl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Checkbox => write!(f, "checkbox"),
            Self::Number => write!(f, "number"),
            Self::Password => write!(f, "password"),
            Self::Radio => write!(f, "radio"),
            Self::Select => write!(f, "select"),
            Self::Textarea => write!(f, "textarea"),
            Self::Text => write!(f, "text"),
        }
    }
}

impl Default for FormControl {
    fn default() -> FormControl {
        Self::Checkbox
    }
}

