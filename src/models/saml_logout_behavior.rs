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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SamlLogoutBehavior {
    #[serde(rename = "AllParticipants")]
    AllParticipants,
    #[serde(rename = "OnlyOriginator")]
    OnlyOriginator,

}

impl ToString for SamlLogoutBehavior {
    fn to_string(&self) -> String {
        match self {
            Self::AllParticipants => String::from("AllParticipants"),
            Self::OnlyOriginator => String::from("OnlyOriginator"),
        }
    }
}

impl Default for SamlLogoutBehavior {
    fn default() -> SamlLogoutBehavior {
        Self::AllParticipants
    }
}

