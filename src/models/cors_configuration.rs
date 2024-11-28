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

/// CorsConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsConfiguration {
    #[serde(rename = "allowCredentials", skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
    #[serde(rename = "allowedHeaders", skip_serializing_if = "Option::is_none")]
    pub allowed_headers: Option<Vec<String>>,
    #[serde(rename = "allowedMethods", skip_serializing_if = "Option::is_none")]
    pub allowed_methods: Option<Vec<models::HttpMethod>>,
    #[serde(rename = "allowedOrigins", skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(rename = "exposedHeaders", skip_serializing_if = "Option::is_none")]
    pub exposed_headers: Option<Vec<String>>,
    #[serde(rename = "preflightMaxAgeInSeconds", skip_serializing_if = "Option::is_none")]
    pub preflight_max_age_in_seconds: Option<i32>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl CorsConfiguration {
    /// 
    pub fn new() -> CorsConfiguration {
        CorsConfiguration {
            allow_credentials: None,
            allowed_headers: None,
            allowed_methods: None,
            allowed_origins: None,
            debug: None,
            exposed_headers: None,
            preflight_max_age_in_seconds: None,
            enabled: None,
        }
    }
}

