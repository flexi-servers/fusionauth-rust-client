/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum KeyAlgorithm {
    #[serde(rename = "ES256")]
    Es256,
    #[serde(rename = "ES384")]
    Es384,
    #[serde(rename = "ES512")]
    Es512,
    #[serde(rename = "HS256")]
    Hs256,
    #[serde(rename = "HS384")]
    Hs384,
    #[serde(rename = "HS512")]
    Hs512,
    #[serde(rename = "RS256")]
    Rs256,
    #[serde(rename = "RS384")]
    Rs384,
    #[serde(rename = "RS512")]
    Rs512,

}

impl ToString for KeyAlgorithm {
    fn to_string(&self) -> String {
        match self {
            Self::Es256 => String::from("ES256"),
            Self::Es384 => String::from("ES384"),
            Self::Es512 => String::from("ES512"),
            Self::Hs256 => String::from("HS256"),
            Self::Hs384 => String::from("HS384"),
            Self::Hs512 => String::from("HS512"),
            Self::Rs256 => String::from("RS256"),
            Self::Rs384 => String::from("RS384"),
            Self::Rs512 => String::from("RS512"),
        }
    }
}

impl Default for KeyAlgorithm {
    fn default() -> KeyAlgorithm {
        Self::Es256
    }
}

