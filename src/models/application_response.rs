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

/// ApplicationResponse : The Application API response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationResponse {
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<models::Application>>,
    #[serde(rename = "applications", skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<models::Application>>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<models::ApplicationRole>>,
}

impl ApplicationResponse {
    /// The Application API response.
    pub fn new() -> ApplicationResponse {
        ApplicationResponse {
            application: None,
            applications: None,
            role: None,
        }
    }
}

