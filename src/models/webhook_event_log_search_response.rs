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

/// WebhookEventLogSearchResponse : Webhook event log search response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookEventLogSearchResponse {
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "webhookEventLogs", skip_serializing_if = "Option::is_none")]
    pub webhook_event_logs: Option<Vec<models::WebhookEventLog>>,
}

impl WebhookEventLogSearchResponse {
    /// Webhook event log search response.
    pub fn new() -> WebhookEventLogSearchResponse {
        WebhookEventLogSearchResponse {
            total: None,
            webhook_event_logs: None,
        }
    }
}

