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

/// Algorithm : Available JSON Web Algorithms (JWA) as described in RFC 7518 available for this JWT implementation.
/// Available JSON Web Algorithms (JWA) as described in RFC 7518 available for this JWT implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Algorithm {
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
    #[serde(rename = "PS256")]
    Ps256,
    #[serde(rename = "PS384")]
    Ps384,
    #[serde(rename = "PS512")]
    Ps512,
    #[serde(rename = "RS256")]
    Rs256,
    #[serde(rename = "RS384")]
    Rs384,
    #[serde(rename = "RS512")]
    Rs512,
    #[serde(rename = "none")]
    None,

}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Es256 => write!(f, "ES256"),
            Self::Es384 => write!(f, "ES384"),
            Self::Es512 => write!(f, "ES512"),
            Self::Hs256 => write!(f, "HS256"),
            Self::Hs384 => write!(f, "HS384"),
            Self::Hs512 => write!(f, "HS512"),
            Self::Ps256 => write!(f, "PS256"),
            Self::Ps384 => write!(f, "PS384"),
            Self::Ps512 => write!(f, "PS512"),
            Self::Rs256 => write!(f, "RS256"),
            Self::Rs384 => write!(f, "RS384"),
            Self::Rs512 => write!(f, "RS512"),
            Self::None => write!(f, "none"),
        }
    }
}

impl Default for Algorithm {
    fn default() -> Algorithm {
        Self::Es256
    }
}

