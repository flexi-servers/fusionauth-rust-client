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

/// SendResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendResponse {
    #[serde(rename = "anonymousResults", skip_serializing_if = "Option::is_none")]
    pub anonymous_results: Option<std::collections::HashMap<String, models::EmailTemplateErrors>>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<std::collections::HashMap<String, models::EmailTemplateErrors>>,
}

impl SendResponse {
    /// 
    pub fn new() -> SendResponse {
        SendResponse {
            anonymous_results: None,
            results: None,
        }
    }
}

