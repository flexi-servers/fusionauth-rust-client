/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SamlLogoutBehavior {
    #[serde(rename = "AllParticipants")]
    AllParticipants,
    #[serde(rename = "OnlyOriginator")]
    OnlyOriginator,

}

impl std::fmt::Display for SamlLogoutBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AllParticipants => write!(f, "AllParticipants"),
            Self::OnlyOriginator => write!(f, "OnlyOriginator"),
        }
    }
}

impl Default for SamlLogoutBehavior {
    fn default() -> SamlLogoutBehavior {
        Self::AllParticipants
    }
}

