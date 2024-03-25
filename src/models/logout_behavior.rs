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

/// LogoutBehavior : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogoutBehavior {
    #[serde(rename = "RedirectOnly")]
    RedirectOnly,
    #[serde(rename = "AllApplications")]
    AllApplications,

}

impl ToString for LogoutBehavior {
    fn to_string(&self) -> String {
        match self {
            Self::RedirectOnly => String::from("RedirectOnly"),
            Self::AllApplications => String::from("AllApplications"),
        }
    }
}

impl Default for LogoutBehavior {
    fn default() -> LogoutBehavior {
        Self::RedirectOnly
    }
}

