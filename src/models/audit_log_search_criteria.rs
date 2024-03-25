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

/// AuditLogSearchCriteria : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogSearchCriteria {
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "newValue", skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[serde(rename = "oldValue", skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "numberOfResults", skip_serializing_if = "Option::is_none")]
    pub number_of_results: Option<i32>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "startRow", skip_serializing_if = "Option::is_none")]
    pub start_row: Option<i32>,
}

impl AuditLogSearchCriteria {
    /// 
    pub fn new() -> AuditLogSearchCriteria {
        AuditLogSearchCriteria {
            end: None,
            message: None,
            new_value: None,
            old_value: None,
            reason: None,
            start: None,
            user: None,
            number_of_results: None,
            order_by: None,
            start_row: None,
        }
    }
}

