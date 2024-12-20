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

/// WebhookAttemptLog : A webhook call attempt log.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookAttemptLog {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "endInstant", skip_serializing_if = "Option::is_none")]
    pub end_instant: Option<i64>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "startInstant", skip_serializing_if = "Option::is_none")]
    pub start_instant: Option<i64>,
    #[serde(rename = "webhookCallResponse", skip_serializing_if = "Option::is_none")]
    pub webhook_call_response: Option<Box<models::WebhookCallResponse>>,
    #[serde(rename = "webhookEventLogId", skip_serializing_if = "Option::is_none")]
    pub webhook_event_log_id: Option<uuid::Uuid>,
    #[serde(rename = "webhookId", skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<uuid::Uuid>,
    #[serde(rename = "attemptResult", skip_serializing_if = "Option::is_none")]
    pub attempt_result: Option<models::WebhookAttemptResult>,
}

impl WebhookAttemptLog {
    /// A webhook call attempt log.
    pub fn new() -> WebhookAttemptLog {
        WebhookAttemptLog {
            data: None,
            end_instant: None,
            id: None,
            start_instant: None,
            webhook_call_response: None,
            webhook_event_log_id: None,
            webhook_id: None,
            attempt_result: None,
        }
    }
}

