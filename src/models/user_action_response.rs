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

/// UserActionResponse : User Action API response object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserActionResponse {
    #[serde(rename = "userAction", skip_serializing_if = "Option::is_none")]
    pub user_action: Option<Box<models::UserAction>>,
    #[serde(rename = "userActions", skip_serializing_if = "Option::is_none")]
    pub user_actions: Option<Vec<models::UserAction>>,
}

impl UserActionResponse {
    /// User Action API response object.
    pub fn new() -> UserActionResponse {
        UserActionResponse {
            user_action: None,
            user_actions: None,
        }
    }
}

