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

/// EntitySearchCriteria : This class is the entity query. It provides a build pattern as well as public fields for use on forms and in actions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntitySearchCriteria {
    #[serde(rename = "accurateTotal", skip_serializing_if = "Option::is_none")]
    pub accurate_total: Option<bool>,
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "nextResults", skip_serializing_if = "Option::is_none")]
    pub next_results: Option<String>,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "queryString", skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "sortFields", skip_serializing_if = "Option::is_none")]
    pub sort_fields: Option<Vec<models::SortField>>,
}

impl EntitySearchCriteria {
    /// This class is the entity query. It provides a build pattern as well as public fields for use on forms and in actions.
    pub fn new() -> EntitySearchCriteria {
        EntitySearchCriteria {
            accurate_total: None,
            ids: None,
            next_results: None,
            query: None,
            query_string: None,
            sort_fields: None,
        }
    }
}

