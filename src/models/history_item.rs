/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryItem {
    #[serde(rename = "actionerUserId", skip_serializing_if = "Option::is_none")]
    pub actioner_user_id: Option<uuid::Uuid>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "createInstant", skip_serializing_if = "Option::is_none")]
    pub create_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
}

impl HistoryItem {
    pub fn new() -> HistoryItem {
        HistoryItem {
            actioner_user_id: None,
            comment: None,
            create_instant: None,
            expiry: None,
        }
    }
}

