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
pub enum BreachAction {
    #[serde(rename = "Off")]
    Off,
    #[serde(rename = "RecordOnly")]
    RecordOnly,
    #[serde(rename = "NotifyUser")]
    NotifyUser,
    #[serde(rename = "RequireChange")]
    RequireChange,

}

impl ToString for BreachAction {
    fn to_string(&self) -> String {
        match self {
            Self::Off => String::from("Off"),
            Self::RecordOnly => String::from("RecordOnly"),
            Self::NotifyUser => String::from("NotifyUser"),
            Self::RequireChange => String::from("RequireChange"),
        }
    }
}

impl Default for BreachAction {
    fn default() -> BreachAction {
        Self::Off
    }
}

