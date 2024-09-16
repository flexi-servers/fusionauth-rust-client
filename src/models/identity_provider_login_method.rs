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

/// IdentityProviderLoginMethod : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdentityProviderLoginMethod {
    #[serde(rename = "UsePopup")]
    UsePopup,
    #[serde(rename = "UseRedirect")]
    UseRedirect,
    #[serde(rename = "UseVendorJavaScript")]
    UseVendorJavaScript,

}

impl std::fmt::Display for IdentityProviderLoginMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UsePopup => write!(f, "UsePopup"),
            Self::UseRedirect => write!(f, "UseRedirect"),
            Self::UseVendorJavaScript => write!(f, "UseVendorJavaScript"),
        }
    }
}

impl Default for IdentityProviderLoginMethod {
    fn default() -> IdentityProviderLoginMethod {
        Self::UsePopup
    }
}

