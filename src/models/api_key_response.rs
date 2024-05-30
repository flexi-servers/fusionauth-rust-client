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

/// ApiKeyResponse : Authentication key response object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<Box<models::ApiKey>>,
}

impl ApiKeyResponse {
    /// Authentication key response object.
    pub fn new() -> ApiKeyResponse {
        ApiKeyResponse {
            api_key: None,
        }
    }
}

