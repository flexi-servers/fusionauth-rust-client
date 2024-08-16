/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Samlv2ApplicationConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Samlv2ApplicationConfiguration {
    #[serde(rename = "buttonImageURL", skip_serializing_if = "Option::is_none")]
    pub button_image_url: Option<String>,
    #[serde(rename = "buttonText", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "createRegistration", skip_serializing_if = "Option::is_none")]
    pub create_registration: Option<bool>,
}

impl Samlv2ApplicationConfiguration {
    /// 
    pub fn new() -> Samlv2ApplicationConfiguration {
        Samlv2ApplicationConfiguration {
            button_image_url: None,
            button_text: None,
            data: None,
            create_registration: None,
        }
    }
}

