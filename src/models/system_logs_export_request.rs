/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SystemLogsExportRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemLogsExportRequest {
    #[serde(rename = "includeArchived", skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    #[serde(rename = "lastNBytes", skip_serializing_if = "Option::is_none")]
    pub last_n_bytes: Option<i32>,
    #[serde(rename = "dateTimeSecondsFormat", skip_serializing_if = "Option::is_none")]
    pub date_time_seconds_format: Option<String>,
    /// Timezone Identifier
    #[serde(rename = "zoneId", skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

impl SystemLogsExportRequest {
    /// 
    pub fn new() -> SystemLogsExportRequest {
        SystemLogsExportRequest {
            include_archived: None,
            last_n_bytes: None,
            date_time_seconds_format: None,
            zone_id: None,
        }
    }
}

