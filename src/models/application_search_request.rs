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

/// ApplicationSearchRequest : Search request for Applications
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationSearchRequest {
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<Box<models::ApplicationSearchCriteria>>,
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

impl ApplicationSearchRequest {
    /// Search request for Applications
    pub fn new() -> ApplicationSearchRequest {
        ApplicationSearchRequest {
            search: None,
            expand: None,
        }
    }
}

