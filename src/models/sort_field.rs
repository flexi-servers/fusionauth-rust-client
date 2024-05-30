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

/// SortField : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SortField {
    #[serde(rename = "missing", skip_serializing_if = "Option::is_none")]
    pub missing: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<models::Sort>,
}

impl SortField {
    /// 
    pub fn new() -> SortField {
        SortField {
            missing: None,
            name: None,
            order: None,
        }
    }
}

