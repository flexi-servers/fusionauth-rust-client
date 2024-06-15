/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ThemeSearchResponse : Search response for Themes
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeSearchResponse {
    #[serde(rename = "themes", skip_serializing_if = "Option::is_none")]
    pub themes: Option<Vec<models::Theme>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl ThemeSearchResponse {
    /// Search response for Themes
    pub fn new() -> ThemeSearchResponse {
        ThemeSearchResponse {
            themes: None,
            total: None,
        }
    }
}

