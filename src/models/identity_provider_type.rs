/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.54.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IdentityProviderType : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdentityProviderType {
    #[serde(rename = "Apple")]
    Apple,
    #[serde(rename = "EpicGames")]
    EpicGames,
    #[serde(rename = "ExternalJWT")]
    ExternalJwt,
    #[serde(rename = "Facebook")]
    Facebook,
    #[serde(rename = "Google")]
    Google,
    #[serde(rename = "HYPR")]
    Hypr,
    #[serde(rename = "LinkedIn")]
    LinkedIn,
    #[serde(rename = "Nintendo")]
    Nintendo,
    #[serde(rename = "OpenIDConnect")]
    OpenIdConnect,
    #[serde(rename = "SAMLv2")]
    Samlv2,
    #[serde(rename = "SAMLv2IdPInitiated")]
    Samlv2IdPInitiated,
    #[serde(rename = "SonyPSN")]
    SonyPsn,
    #[serde(rename = "Steam")]
    Steam,
    #[serde(rename = "Twitch")]
    Twitch,
    #[serde(rename = "Twitter")]
    Twitter,
    #[serde(rename = "Xbox")]
    Xbox,

}

impl std::fmt::Display for IdentityProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Apple => write!(f, "Apple"),
            Self::EpicGames => write!(f, "EpicGames"),
            Self::ExternalJwt => write!(f, "ExternalJWT"),
            Self::Facebook => write!(f, "Facebook"),
            Self::Google => write!(f, "Google"),
            Self::Hypr => write!(f, "HYPR"),
            Self::LinkedIn => write!(f, "LinkedIn"),
            Self::Nintendo => write!(f, "Nintendo"),
            Self::OpenIdConnect => write!(f, "OpenIDConnect"),
            Self::Samlv2 => write!(f, "SAMLv2"),
            Self::Samlv2IdPInitiated => write!(f, "SAMLv2IdPInitiated"),
            Self::SonyPsn => write!(f, "SonyPSN"),
            Self::Steam => write!(f, "Steam"),
            Self::Twitch => write!(f, "Twitch"),
            Self::Twitter => write!(f, "Twitter"),
            Self::Xbox => write!(f, "Xbox"),
        }
    }
}

impl Default for IdentityProviderType {
    fn default() -> IdentityProviderType {
        Self::Apple
    }
}

