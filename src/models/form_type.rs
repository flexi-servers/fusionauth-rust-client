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

/// FormType : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormType {
    #[serde(rename = "registration")]
    Registration,
    #[serde(rename = "adminRegistration")]
    AdminRegistration,
    #[serde(rename = "adminUser")]
    AdminUser,
    #[serde(rename = "selfServiceUser")]
    SelfServiceUser,

}

impl std::fmt::Display for FormType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Registration => write!(f, "registration"),
            Self::AdminRegistration => write!(f, "adminRegistration"),
            Self::AdminUser => write!(f, "adminUser"),
            Self::SelfServiceUser => write!(f, "selfServiceUser"),
        }
    }
}

impl Default for FormType {
    fn default() -> FormType {
        Self::Registration
    }
}

