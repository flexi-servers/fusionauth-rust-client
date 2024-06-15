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

/// LoginReportResponse : Response for the login report.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginReportResponse {
    #[serde(rename = "hourlyCounts", skip_serializing_if = "Option::is_none")]
    pub hourly_counts: Option<Vec<models::Count>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl LoginReportResponse {
    /// Response for the login report.
    pub fn new() -> LoginReportResponse {
        LoginReportResponse {
            hourly_counts: None,
            total: None,
        }
    }
}

