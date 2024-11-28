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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClientAuthenticationMethod {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "client_secret_basic")]
    ClientSecretBasic,
    #[serde(rename = "client_secret_post")]
    ClientSecretPost,

}

impl std::fmt::Display for ClientAuthenticationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::ClientSecretBasic => write!(f, "client_secret_basic"),
            Self::ClientSecretPost => write!(f, "client_secret_post"),
        }
    }
}

impl Default for ClientAuthenticationMethod {
    fn default() -> ClientAuthenticationMethod {
        Self::None
    }
}

