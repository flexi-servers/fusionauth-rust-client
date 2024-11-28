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

/// UserActionReasonResponse : User Action Reason API response object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserActionReasonResponse {
    #[serde(rename = "userActionReason", skip_serializing_if = "Option::is_none")]
    pub user_action_reason: Option<Box<models::UserActionReason>>,
    #[serde(rename = "userActionReasons", skip_serializing_if = "Option::is_none")]
    pub user_action_reasons: Option<Vec<models::UserActionReason>>,
}

impl UserActionReasonResponse {
    /// User Action Reason API response object.
    pub fn new() -> UserActionReasonResponse {
        UserActionReasonResponse {
            user_action_reason: None,
            user_action_reasons: None,
        }
    }
}

