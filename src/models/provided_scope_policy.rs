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

/// ProvidedScopePolicy : The handling policy for scopes provided by FusionAuth
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvidedScopePolicy {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<models::Requirable>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<models::Requirable>>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<Box<models::Requirable>>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<Box<models::Requirable>>,
}

impl ProvidedScopePolicy {
    /// The handling policy for scopes provided by FusionAuth
    pub fn new() -> ProvidedScopePolicy {
        ProvidedScopePolicy {
            address: None,
            email: None,
            phone: None,
            profile: None,
        }
    }
}

