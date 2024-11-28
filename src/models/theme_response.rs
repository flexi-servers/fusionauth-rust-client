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

/// ThemeResponse : Theme API response object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeResponse {
    #[serde(rename = "theme", skip_serializing_if = "Option::is_none")]
    pub theme: Option<Box<models::Theme>>,
    #[serde(rename = "themes", skip_serializing_if = "Option::is_none")]
    pub themes: Option<Vec<models::Theme>>,
}

impl ThemeResponse {
    /// Theme API response object.
    pub fn new() -> ThemeResponse {
        ThemeResponse {
            theme: None,
            themes: None,
        }
    }
}

