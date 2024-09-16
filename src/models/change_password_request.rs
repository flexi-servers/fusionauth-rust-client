/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ChangePasswordRequest : Change password request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "changePasswordId", skip_serializing_if = "Option::is_none")]
    pub change_password_id: Option<String>,
    #[serde(rename = "currentPassword", skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    #[serde(rename = "loginId", skip_serializing_if = "Option::is_none")]
    pub login_id: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "trustChallenge", skip_serializing_if = "Option::is_none")]
    pub trust_challenge: Option<String>,
    #[serde(rename = "trustToken", skip_serializing_if = "Option::is_none")]
    pub trust_token: Option<String>,
    #[serde(rename = "eventInfo", skip_serializing_if = "Option::is_none")]
    pub event_info: Option<Box<models::EventInfo>>,
}

impl ChangePasswordRequest {
    /// Change password request object.
    pub fn new() -> ChangePasswordRequest {
        ChangePasswordRequest {
            application_id: None,
            change_password_id: None,
            current_password: None,
            login_id: None,
            password: None,
            refresh_token: None,
            trust_challenge: None,
            trust_token: None,
            event_info: None,
        }
    }
}

