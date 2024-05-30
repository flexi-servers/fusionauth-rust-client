/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DailyActiveUserReportResponse : Response for the daily active user report.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DailyActiveUserReportResponse {
    #[serde(rename = "dailyActiveUsers", skip_serializing_if = "Option::is_none")]
    pub daily_active_users: Option<Vec<models::Count>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl DailyActiveUserReportResponse {
    /// Response for the daily active user report.
    pub fn new() -> DailyActiveUserReportResponse {
        DailyActiveUserReportResponse {
            daily_active_users: None,
            total: None,
        }
    }
}

