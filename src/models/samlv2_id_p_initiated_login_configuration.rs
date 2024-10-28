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

/// Samlv2IdPInitiatedLoginConfiguration : IdP Initiated login configuration
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Samlv2IdPInitiatedLoginConfiguration {
    #[serde(rename = "nameIdFormat", skip_serializing_if = "Option::is_none")]
    pub name_id_format: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Samlv2IdPInitiatedLoginConfiguration {
    /// IdP Initiated login configuration
    pub fn new() -> Samlv2IdPInitiatedLoginConfiguration {
        Samlv2IdPInitiatedLoginConfiguration {
            name_id_format: None,
            enabled: None,
        }
    }
}

