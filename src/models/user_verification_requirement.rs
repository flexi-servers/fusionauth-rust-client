/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserVerificationRequirement : Used to express whether the Relying Party requires <a href=\"https:www.w3.orgTRwebauthn-2#user-verification\">user verification<a> for the  current operation.
/// Used to express whether the Relying Party requires <a href=\"https:www.w3.orgTRwebauthn-2#user-verification\">user verification<a> for the  current operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserVerificationRequirement {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "preferred")]
    Preferred,
    #[serde(rename = "discouraged")]
    Discouraged,

}

impl std::fmt::Display for UserVerificationRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Required => write!(f, "required"),
            Self::Preferred => write!(f, "preferred"),
            Self::Discouraged => write!(f, "discouraged"),
        }
    }
}

impl Default for UserVerificationRequirement {
    fn default() -> UserVerificationRequirement {
        Self::Required
    }
}

