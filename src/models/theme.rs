/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// Theme : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "defaultMessages", skip_serializing_if = "Option::is_none")]
    pub default_messages: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    /// Models a set of localized Strings that can be stored as JSON.
    #[serde(rename = "localizedMessages", skip_serializing_if = "Option::is_none")]
    pub localized_messages: Option<serde_json::Value>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "stylesheet", skip_serializing_if = "Option::is_none")]
    pub stylesheet: Option<String>,
    #[serde(rename = "templates", skip_serializing_if = "Option::is_none")]
    pub templates: Option<Box<models::Templates>>,
}

impl Theme {
    /// 
    pub fn new() -> Theme {
        Theme {
            data: None,
            default_messages: None,
            id: None,
            insert_instant: None,
            last_update_instant: None,
            localized_messages: None,
            name: None,
            stylesheet: None,
            templates: None,
        }
    }
}

