/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

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

impl ToString for IdentityProviderLoginMethod {
    fn to_string(&self) -> String {
        match self {
            Self::UsePopup => String::from("UsePopup"),
            Self::UseRedirect => String::from("UseRedirect"),
            Self::UseVendorJavaScript => String::from("UseVendorJavaScript"),
        }
    }
}

impl Default for IdentityProviderLoginMethod {
    fn default() -> IdentityProviderLoginMethod {
        Self::UsePopup
    }
}

