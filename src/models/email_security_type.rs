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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmailSecurityType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "SSL")]
    Ssl,
    #[serde(rename = "TLS")]
    Tls,

}

impl std::fmt::Display for EmailSecurityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Ssl => write!(f, "SSL"),
            Self::Tls => write!(f, "TLS"),
        }
    }
}

impl Default for EmailSecurityType {
    fn default() -> EmailSecurityType {
        Self::None
    }
}

