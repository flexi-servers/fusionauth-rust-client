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

/// LoginRecordSearchRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginRecordSearchRequest {
    #[serde(rename = "retrieveTotal", skip_serializing_if = "Option::is_none")]
    pub retrieve_total: Option<bool>,
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<Box<models::LoginRecordSearchCriteria>>,
}

impl LoginRecordSearchRequest {
    /// 
    pub fn new() -> LoginRecordSearchRequest {
        LoginRecordSearchRequest {
            retrieve_total: None,
            search: None,
        }
    }
}

