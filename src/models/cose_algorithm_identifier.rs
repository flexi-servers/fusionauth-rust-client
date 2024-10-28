/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CoseAlgorithmIdentifier : A number identifying a cryptographic algorithm. Values should be registered with the <a  href=\"https:www.iana.orgassignmentscosecose.xhtml#algorithms\">IANA COSE Algorithms registry<a>
/// A number identifying a cryptographic algorithm. Values should be registered with the <a  href=\"https:www.iana.orgassignmentscosecose.xhtml#algorithms\">IANA COSE Algorithms registry<a>
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CoseAlgorithmIdentifier {
    #[serde(rename = "ES256")]
    Es256,
    #[serde(rename = "ES384")]
    Es384,
    #[serde(rename = "ES512")]
    Es512,
    #[serde(rename = "RS256")]
    Rs256,
    #[serde(rename = "RS384")]
    Rs384,
    #[serde(rename = "RS512")]
    Rs512,
    #[serde(rename = "PS256")]
    Ps256,
    #[serde(rename = "PS384")]
    Ps384,
    #[serde(rename = "PS512")]
    Ps512,

}

impl std::fmt::Display for CoseAlgorithmIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Es256 => write!(f, "ES256"),
            Self::Es384 => write!(f, "ES384"),
            Self::Es512 => write!(f, "ES512"),
            Self::Rs256 => write!(f, "RS256"),
            Self::Rs384 => write!(f, "RS384"),
            Self::Rs512 => write!(f, "RS512"),
            Self::Ps256 => write!(f, "PS256"),
            Self::Ps384 => write!(f, "PS384"),
            Self::Ps512 => write!(f, "PS512"),
        }
    }
}

impl Default for CoseAlgorithmIdentifier {
    fn default() -> CoseAlgorithmIdentifier {
        Self::Es256
    }
}

