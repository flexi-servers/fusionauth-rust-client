/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// SearchResults : Search results.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResults {
    #[serde(rename = "nextResults", skip_serializing_if = "Option::is_none")]
    pub next_results: Option<String>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<serde_json::Value>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "totalEqualToActual", skip_serializing_if = "Option::is_none")]
    pub total_equal_to_actual: Option<bool>,
}

impl SearchResults {
    /// Search results.
    pub fn new() -> SearchResults {
        SearchResults {
            next_results: None,
            results: None,
            total: None,
            total_equal_to_actual: None,
        }
    }
}

