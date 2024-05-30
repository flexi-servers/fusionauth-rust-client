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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IdentityProviderField {
    #[serde(rename="Steam")]
    Steam(Box<models::SteamIdentityProvider>),
    #[serde(rename="Xbox")]
    Xbox(Box<models::XboxIdentityProvider>),
    #[serde(rename="LinkedIn")]
    LinkedIn(Box<models::LinkedInIdentityProvider>),
    #[serde(rename="ExternalJWT")]
    ExternalJwt(Box<models::ExternalJwtIdentityProvider>),
    #[serde(rename="SAMLv2")]
    Samlv2(Box<models::Samlv2IdentityProvider>),
    #[serde(rename="Facebook")]
    Facebook(Box<models::FacebookIdentityProvider>),
    #[serde(rename="SAMLv2IdPInitiated")]
    Samlv2IdPInitiated(Box<models::Samlv2IdPInitiatedIdentityProvider>),
    #[serde(rename="Apple")]
    Apple(Box<models::AppleIdentityProvider>),
    #[serde(rename="OpenIdConnect")]
    OpenIdConnect(Box<models::OpenIdConnectIdentityProvider>),
    #[serde(rename="Google")]
    Google(Box<models::GoogleIdentityProvider>),
    #[serde(rename="SonyPSN")]
    SonyPsn(Box<models::SonyPsnIdentityProvider>),
    #[serde(rename="Twitch")]
    Twitch(Box<models::TwitchIdentityProvider>),
    #[serde(rename="EpicGames")]
    EpicGames(Box<models::EpicGamesIdentityProvider>),
    #[serde(rename="Nintendo")]
    Nintendo(Box<models::NintendoIdentityProvider>),
    #[serde(rename="Twitter")]
    Twitter(Box<models::TwitterIdentityProvider>),
    #[serde(rename="HYPR")]
    Hypr(Box<models::HyprIdentityProvider>),
}

impl Default for IdentityProviderField {
    fn default() -> Self {
        Self::Steam(Default::default())
    }
}


