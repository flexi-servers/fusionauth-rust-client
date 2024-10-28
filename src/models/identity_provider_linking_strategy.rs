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

/// IdentityProviderLinkingStrategy : The IdP behavior when no user link has been made yet.
/// The IdP behavior when no user link has been made yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdentityProviderLinkingStrategy {
    #[serde(rename = "CreatePendingLink")]
    CreatePendingLink,
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "LinkAnonymously")]
    LinkAnonymously,
    #[serde(rename = "LinkByEmail")]
    LinkByEmail,
    #[serde(rename = "LinkByEmailForExistingUser")]
    LinkByEmailForExistingUser,
    #[serde(rename = "LinkByUsername")]
    LinkByUsername,
    #[serde(rename = "LinkByUsernameForExistingUser")]
    LinkByUsernameForExistingUser,
    #[serde(rename = "Unsupported")]
    Unsupported,

}

impl std::fmt::Display for IdentityProviderLinkingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CreatePendingLink => write!(f, "CreatePendingLink"),
            Self::Disabled => write!(f, "Disabled"),
            Self::LinkAnonymously => write!(f, "LinkAnonymously"),
            Self::LinkByEmail => write!(f, "LinkByEmail"),
            Self::LinkByEmailForExistingUser => write!(f, "LinkByEmailForExistingUser"),
            Self::LinkByUsername => write!(f, "LinkByUsername"),
            Self::LinkByUsernameForExistingUser => write!(f, "LinkByUsernameForExistingUser"),
            Self::Unsupported => write!(f, "Unsupported"),
        }
    }
}

impl Default for IdentityProviderLinkingStrategy {
    fn default() -> IdentityProviderLinkingStrategy {
        Self::CreatePendingLink
    }
}

