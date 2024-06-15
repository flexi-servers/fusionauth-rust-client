/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LoginRecordExportRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginRecordExportRequest {
    #[serde(rename = "criteria", skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Box<models::LoginRecordSearchCriteria>>,
    #[serde(rename = "dateTimeSecondsFormat", skip_serializing_if = "Option::is_none")]
    pub date_time_seconds_format: Option<String>,
    /// Timezone Identifier
    #[serde(rename = "zoneId", skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

impl LoginRecordExportRequest {
    /// 
    pub fn new() -> LoginRecordExportRequest {
        LoginRecordExportRequest {
            criteria: None,
            date_time_seconds_format: None,
            zone_id: None,
        }
    }
}

