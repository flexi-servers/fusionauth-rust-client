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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookEventLog {
    #[serde(rename = "attempts", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Vec<models::WebhookAttemptLog>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Box<models::EventRequest>>,
    #[serde(rename = "eventResult", skip_serializing_if = "Option::is_none")]
    pub event_result: Option<models::WebhookEventResult>,
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<models::EventType>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastAttemptInstant", skip_serializing_if = "Option::is_none")]
    pub last_attempt_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "linkedObjectId", skip_serializing_if = "Option::is_none")]
    pub linked_object_id: Option<uuid::Uuid>,
    #[serde(rename = "sequence", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
    #[serde(rename = "failedAttempts", skip_serializing_if = "Option::is_none")]
    pub failed_attempts: Option<i32>,
    #[serde(rename = "successfulAttempts", skip_serializing_if = "Option::is_none")]
    pub successful_attempts: Option<i32>,
}

impl WebhookEventLog {
    pub fn new() -> WebhookEventLog {
        WebhookEventLog {
            attempts: None,
            data: None,
            event: None,
            event_result: None,
            event_type: None,
            id: None,
            insert_instant: None,
            last_attempt_instant: None,
            last_update_instant: None,
            linked_object_id: None,
            sequence: None,
            failed_attempts: None,
            successful_attempts: None,
        }
    }
}

