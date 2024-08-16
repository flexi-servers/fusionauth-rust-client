/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LambdaType : The types of lambdas that indicate how they are invoked by FusionAuth.
/// The types of lambdas that indicate how they are invoked by FusionAuth.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LambdaType {
    #[serde(rename = "JWTPopulate")]
    JwtPopulate,
    #[serde(rename = "OpenIDReconcile")]
    OpenIdReconcile,
    #[serde(rename = "SAMLv2Reconcile")]
    Samlv2Reconcile,
    #[serde(rename = "SAMLv2Populate")]
    Samlv2Populate,
    #[serde(rename = "AppleReconcile")]
    AppleReconcile,
    #[serde(rename = "ExternalJWTReconcile")]
    ExternalJwtReconcile,
    #[serde(rename = "FacebookReconcile")]
    FacebookReconcile,
    #[serde(rename = "GoogleReconcile")]
    GoogleReconcile,
    #[serde(rename = "HYPRReconcile")]
    HyprReconcile,
    #[serde(rename = "TwitterReconcile")]
    TwitterReconcile,
    #[serde(rename = "LDAPConnectorReconcile")]
    LdapConnectorReconcile,
    #[serde(rename = "LinkedInReconcile")]
    LinkedInReconcile,
    #[serde(rename = "EpicGamesReconcile")]
    EpicGamesReconcile,
    #[serde(rename = "NintendoReconcile")]
    NintendoReconcile,
    #[serde(rename = "SonyPSNReconcile")]
    SonyPsnReconcile,
    #[serde(rename = "SteamReconcile")]
    SteamReconcile,
    #[serde(rename = "TwitchReconcile")]
    TwitchReconcile,
    #[serde(rename = "XboxReconcile")]
    XboxReconcile,
    #[serde(rename = "ClientCredentialsJWTPopulate")]
    ClientCredentialsJwtPopulate,
    #[serde(rename = "SCIMServerGroupRequestConverter")]
    ScimServerGroupRequestConverter,
    #[serde(rename = "SCIMServerGroupResponseConverter")]
    ScimServerGroupResponseConverter,
    #[serde(rename = "SCIMServerUserRequestConverter")]
    ScimServerUserRequestConverter,
    #[serde(rename = "SCIMServerUserResponseConverter")]
    ScimServerUserResponseConverter,
    #[serde(rename = "SelfServiceRegistrationValidation")]
    SelfServiceRegistrationValidation,
    #[serde(rename = "UserInfoPopulate")]
    UserInfoPopulate,

}

impl std::fmt::Display for LambdaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JwtPopulate => write!(f, "JWTPopulate"),
            Self::OpenIdReconcile => write!(f, "OpenIDReconcile"),
            Self::Samlv2Reconcile => write!(f, "SAMLv2Reconcile"),
            Self::Samlv2Populate => write!(f, "SAMLv2Populate"),
            Self::AppleReconcile => write!(f, "AppleReconcile"),
            Self::ExternalJwtReconcile => write!(f, "ExternalJWTReconcile"),
            Self::FacebookReconcile => write!(f, "FacebookReconcile"),
            Self::GoogleReconcile => write!(f, "GoogleReconcile"),
            Self::HyprReconcile => write!(f, "HYPRReconcile"),
            Self::TwitterReconcile => write!(f, "TwitterReconcile"),
            Self::LdapConnectorReconcile => write!(f, "LDAPConnectorReconcile"),
            Self::LinkedInReconcile => write!(f, "LinkedInReconcile"),
            Self::EpicGamesReconcile => write!(f, "EpicGamesReconcile"),
            Self::NintendoReconcile => write!(f, "NintendoReconcile"),
            Self::SonyPsnReconcile => write!(f, "SonyPSNReconcile"),
            Self::SteamReconcile => write!(f, "SteamReconcile"),
            Self::TwitchReconcile => write!(f, "TwitchReconcile"),
            Self::XboxReconcile => write!(f, "XboxReconcile"),
            Self::ClientCredentialsJwtPopulate => write!(f, "ClientCredentialsJWTPopulate"),
            Self::ScimServerGroupRequestConverter => write!(f, "SCIMServerGroupRequestConverter"),
            Self::ScimServerGroupResponseConverter => write!(f, "SCIMServerGroupResponseConverter"),
            Self::ScimServerUserRequestConverter => write!(f, "SCIMServerUserRequestConverter"),
            Self::ScimServerUserResponseConverter => write!(f, "SCIMServerUserResponseConverter"),
            Self::SelfServiceRegistrationValidation => write!(f, "SelfServiceRegistrationValidation"),
            Self::UserInfoPopulate => write!(f, "UserInfoPopulate"),
        }
    }
}

impl Default for LambdaType {
    fn default() -> LambdaType {
        Self::JwtPopulate
    }
}

