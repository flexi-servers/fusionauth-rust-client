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

/// FailedAuthenticationConfiguration : Configuration for the behavior of failed login attempts. This helps us protect against brute force password attacks.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedAuthenticationConfiguration {
    #[serde(rename = "actionCancelPolicy", skip_serializing_if = "Option::is_none")]
    pub action_cancel_policy: Option<Box<models::FailedAuthenticationActionCancelPolicy>>,
    #[serde(rename = "actionDuration", skip_serializing_if = "Option::is_none")]
    pub action_duration: Option<i64>,
    #[serde(rename = "actionDurationUnit", skip_serializing_if = "Option::is_none")]
    pub action_duration_unit: Option<models::ExpiryUnit>,
    #[serde(rename = "emailUser", skip_serializing_if = "Option::is_none")]
    pub email_user: Option<bool>,
    #[serde(rename = "resetCountInSeconds", skip_serializing_if = "Option::is_none")]
    pub reset_count_in_seconds: Option<i32>,
    #[serde(rename = "tooManyAttempts", skip_serializing_if = "Option::is_none")]
    pub too_many_attempts: Option<i32>,
    #[serde(rename = "userActionId", skip_serializing_if = "Option::is_none")]
    pub user_action_id: Option<uuid::Uuid>,
}

impl FailedAuthenticationConfiguration {
    /// Configuration for the behavior of failed login attempts. This helps us protect against brute force password attacks.
    pub fn new() -> FailedAuthenticationConfiguration {
        FailedAuthenticationConfiguration {
            action_cancel_policy: None,
            action_duration: None,
            action_duration_unit: None,
            email_user: None,
            reset_count_in_seconds: None,
            too_many_attempts: None,
            user_action_id: None,
        }
    }
}

