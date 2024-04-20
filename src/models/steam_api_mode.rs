/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// SteamApiMode : Steam API modes.
/// Steam API modes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SteamApiMode {
    #[serde(rename = "Public")]
    Public,
    #[serde(rename = "Partner")]
    Partner,

}

impl ToString for SteamApiMode {
    fn to_string(&self) -> String {
        match self {
            Self::Public => String::from("Public"),
            Self::Partner => String::from("Partner"),
        }
    }
}

impl Default for SteamApiMode {
    fn default() -> SteamApiMode {
        Self::Public
    }
}

