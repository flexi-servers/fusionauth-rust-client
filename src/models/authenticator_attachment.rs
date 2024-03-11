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

/// AuthenticatorAttachment : Describes the <a href=\"https:www.w3.orgTRwebauthn-2#authenticator-attachment-modality\">authenticator attachment modality<a>.
/// Describes the <a href=\"https:www.w3.orgTRwebauthn-2#authenticator-attachment-modality\">authenticator attachment modality<a>.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthenticatorAttachment {
    #[serde(rename = "platform")]
    Platform,
    #[serde(rename = "crossPlatform")]
    CrossPlatform,

}

impl ToString for AuthenticatorAttachment {
    fn to_string(&self) -> String {
        match self {
            Self::Platform => String::from("platform"),
            Self::CrossPlatform => String::from("crossPlatform"),
        }
    }
}

impl Default for AuthenticatorAttachment {
    fn default() -> AuthenticatorAttachment {
        Self::Platform
    }
}

