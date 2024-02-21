/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserActionPhase : The phases of a time-based user action.

/// The phases of a time-based user action.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserActionPhase {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "modify")]
    Modify,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "end")]
    End,

}

impl ToString for UserActionPhase {
    fn to_string(&self) -> String {
        match self {
            Self::Start => String::from("start"),
            Self::Modify => String::from("modify"),
            Self::Cancel => String::from("cancel"),
            Self::End => String::from("end"),
        }
    }
}

impl Default for UserActionPhase {
    fn default() -> UserActionPhase {
        Self::Start
    }
}




