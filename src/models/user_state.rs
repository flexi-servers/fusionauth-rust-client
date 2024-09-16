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

/// UserState : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserState {
    #[serde(rename = "Authenticated")]
    Authenticated,
    #[serde(rename = "AuthenticatedNotRegistered")]
    AuthenticatedNotRegistered,
    #[serde(rename = "AuthenticatedNotVerified")]
    AuthenticatedNotVerified,
    #[serde(rename = "AuthenticatedRegistrationNotVerified")]
    AuthenticatedRegistrationNotVerified,

}

impl std::fmt::Display for UserState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Authenticated => write!(f, "Authenticated"),
            Self::AuthenticatedNotRegistered => write!(f, "AuthenticatedNotRegistered"),
            Self::AuthenticatedNotVerified => write!(f, "AuthenticatedNotVerified"),
            Self::AuthenticatedRegistrationNotVerified => write!(f, "AuthenticatedRegistrationNotVerified"),
        }
    }
}

impl Default for UserState {
    fn default() -> UserState {
        Self::Authenticated
    }
}

