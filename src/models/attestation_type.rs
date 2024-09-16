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

/// AttestationType : Used to indicate what type of attestation was included in the authenticator response for a given WebAuthn credential at the time it was created
/// Used to indicate what type of attestation was included in the authenticator response for a given WebAuthn credential at the time it was created
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttestationType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "self")]
    VariantSelf,
    #[serde(rename = "attestationCa")]
    AttestationCa,
    #[serde(rename = "anonymizationCa")]
    AnonymizationCa,
    #[serde(rename = "none")]
    None,

}

impl std::fmt::Display for AttestationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::VariantSelf => write!(f, "self"),
            Self::AttestationCa => write!(f, "attestationCa"),
            Self::AnonymizationCa => write!(f, "anonymizationCa"),
            Self::None => write!(f, "none"),
        }
    }
}

impl Default for AttestationType {
    fn default() -> AttestationType {
        Self::Basic
    }
}

