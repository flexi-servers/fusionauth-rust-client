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

/// BaseElasticSearchCriteria : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseElasticSearchCriteria {
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
    #[serde(rename = "numberOfResults", skip_serializing_if = "Option::is_none")]
    pub number_of_results: Option<i32>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "startRow", skip_serializing_if = "Option::is_none")]
    pub start_row: Option<i32>,
}

impl BaseElasticSearchCriteria {
    /// 
    pub fn new() -> BaseElasticSearchCriteria {
        BaseElasticSearchCriteria {
            accurate_total: None,
            ids: None,
            next_results: None,
            query: None,
            query_string: None,
            sort_fields: None,
            number_of_results: None,
            order_by: None,
            start_row: None,
        }
    }
}

