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

/// WebhookEventLogResponse : Webhook event log response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookEventLogResponse {
    #[serde(rename = "webhookEventLog", skip_serializing_if = "Option::is_none")]
    pub webhook_event_log: Option<Box<models::WebhookEventLog>>,
}

impl WebhookEventLogResponse {
    /// Webhook event log response.
    pub fn new() -> WebhookEventLogResponse {
        WebhookEventLogResponse {
            webhook_event_log: None,
        }
    }
}

