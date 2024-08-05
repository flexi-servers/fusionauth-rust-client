/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.52.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WebhookSearchCriteria : Search criteria for webhooks.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookSearchCriteria {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<uuid::Uuid>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "numberOfResults", skip_serializing_if = "Option::is_none")]
    pub number_of_results: Option<i32>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "startRow", skip_serializing_if = "Option::is_none")]
    pub start_row: Option<i32>,
}

impl WebhookSearchCriteria {
    /// Search criteria for webhooks.
    pub fn new() -> WebhookSearchCriteria {
        WebhookSearchCriteria {
            description: None,
            tenant_id: None,
            url: None,
            number_of_results: None,
            order_by: None,
            start_row: None,
        }
    }
}

