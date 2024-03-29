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

impl ToString for FormControl {
    fn to_string(&self) -> String {
        match self {
            Self::Checkbox => String::from("checkbox"),
            Self::Number => String::from("number"),
            Self::Password => String::from("password"),
            Self::Radio => String::from("radio"),
            Self::Select => String::from("select"),
            Self::Textarea => String::from("textarea"),
            Self::Text => String::from("text"),
        }
    }
}

impl Default for FormControl {
    fn default() -> FormControl {
        Self::Checkbox
    }
}

