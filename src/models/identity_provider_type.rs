/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

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

impl ToString for IdentityProviderType {
    fn to_string(&self) -> String {
        match self {
            Self::Apple => String::from("Apple"),
            Self::EpicGames => String::from("EpicGames"),
            Self::ExternalJwt => String::from("ExternalJWT"),
            Self::Facebook => String::from("Facebook"),
            Self::Google => String::from("Google"),
            Self::Hypr => String::from("HYPR"),
            Self::LinkedIn => String::from("LinkedIn"),
            Self::Nintendo => String::from("Nintendo"),
            Self::OpenIdConnect => String::from("OpenIDConnect"),
            Self::Samlv2 => String::from("SAMLv2"),
            Self::Samlv2IdPInitiated => String::from("SAMLv2IdPInitiated"),
            Self::SonyPsn => String::from("SonyPSN"),
            Self::Steam => String::from("Steam"),
            Self::Twitch => String::from("Twitch"),
            Self::Twitter => String::from("Twitter"),
            Self::Xbox => String::from("Xbox"),
        }
    }
}

impl Default for IdentityProviderType {
    fn default() -> IdentityProviderType {
        Self::Apple
    }
}




