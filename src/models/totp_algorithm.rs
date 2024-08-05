/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.52.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TotpAlgorithm {
    #[serde(rename = "HmacSHA1")]
    HmacSha1,
    #[serde(rename = "HmacSHA256")]
    HmacSha256,
    #[serde(rename = "HmacSHA512")]
    HmacSha512,

}

impl std::fmt::Display for TotpAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HmacSha1 => write!(f, "HmacSHA1"),
            Self::HmacSha256 => write!(f, "HmacSHA256"),
            Self::HmacSha512 => write!(f, "HmacSHA512"),
        }
    }
}

impl Default for TotpAlgorithm {
    fn default() -> TotpAlgorithm {
        Self::HmacSha1
    }
}

