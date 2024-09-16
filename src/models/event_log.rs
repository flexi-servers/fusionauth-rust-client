/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EventLog : Event log used internally by FusionAuth to help developers debug hooks, Webhooks, email templates, etc.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventLog {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::EventLogType>,
}

impl EventLog {
    /// Event log used internally by FusionAuth to help developers debug hooks, Webhooks, email templates, etc.
    pub fn new() -> EventLog {
        EventLog {
            id: None,
            insert_instant: None,
            message: None,
            r#type: None,
        }
    }
}

