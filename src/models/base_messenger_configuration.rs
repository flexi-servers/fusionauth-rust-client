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

/// BaseMessengerConfiguration : Do not require a setter for 'type', it is defined by the concrete class and is not mutable
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseMessengerConfiguration {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "transport", skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::MessengerType>,
}

impl BaseMessengerConfiguration {
    /// Do not require a setter for 'type', it is defined by the concrete class and is not mutable
    pub fn new() -> BaseMessengerConfiguration {
        BaseMessengerConfiguration {
            data: None,
            debug: None,
            id: None,
            insert_instant: None,
            last_update_instant: None,
            name: None,
            transport: None,
            r#type: None,
        }
    }
}

