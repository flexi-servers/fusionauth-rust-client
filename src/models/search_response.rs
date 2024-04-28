/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// SearchResponse : Search API response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "nextResults", skip_serializing_if = "Option::is_none")]
    pub next_results: Option<String>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<models::User>>,
    #[serde(rename = "expandable", skip_serializing_if = "Option::is_none")]
    pub expandable: Option<Vec<String>>,
}

impl SearchResponse {
    /// Search API response.
    pub fn new() -> SearchResponse {
        SearchResponse {
            total: None,
            next_results: None,
            users: None,
            expandable: None,
        }
    }
}

