/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.52.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserDeleteRequest : User API delete request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDeleteRequest {
    #[serde(rename = "dryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "hardDelete", skip_serializing_if = "Option::is_none")]
    pub hard_delete: Option<bool>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "queryString", skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "userIds", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "eventInfo", skip_serializing_if = "Option::is_none")]
    pub event_info: Option<Box<models::EventInfo>>,
}

impl UserDeleteRequest {
    /// User API delete request object.
    pub fn new() -> UserDeleteRequest {
        UserDeleteRequest {
            dry_run: None,
            hard_delete: None,
            limit: None,
            query: None,
            query_string: None,
            user_ids: None,
            event_info: None,
        }
    }
}

