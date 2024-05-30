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

/// CaptchaMethod : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CaptchaMethod {
    #[serde(rename = "GoogleRecaptchaV2")]
    GoogleRecaptchaV2,
    #[serde(rename = "GoogleRecaptchaV3")]
    GoogleRecaptchaV3,
    #[serde(rename = "HCaptcha")]
    HCaptcha,
    #[serde(rename = "HCaptchaEnterprise")]
    HCaptchaEnterprise,

}

impl std::fmt::Display for CaptchaMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GoogleRecaptchaV2 => write!(f, "GoogleRecaptchaV2"),
            Self::GoogleRecaptchaV3 => write!(f, "GoogleRecaptchaV3"),
            Self::HCaptcha => write!(f, "HCaptcha"),
            Self::HCaptchaEnterprise => write!(f, "HCaptchaEnterprise"),
        }
    }
}

impl Default for CaptchaMethod {
    fn default() -> CaptchaMethod {
        Self::GoogleRecaptchaV2
    }
}

