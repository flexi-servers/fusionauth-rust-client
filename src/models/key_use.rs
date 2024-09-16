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

/// KeyUse : The use type of a key.
/// The use type of a key.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum KeyUse {
    #[serde(rename = "SignOnly")]
    SignOnly,
    #[serde(rename = "SignAndVerify")]
    SignAndVerify,
    #[serde(rename = "VerifyOnly")]
    VerifyOnly,

}

impl std::fmt::Display for KeyUse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SignOnly => write!(f, "SignOnly"),
            Self::SignAndVerify => write!(f, "SignAndVerify"),
            Self::VerifyOnly => write!(f, "VerifyOnly"),
        }
    }
}

impl Default for KeyUse {
    fn default() -> KeyUse {
        Self::SignOnly
    }
}

