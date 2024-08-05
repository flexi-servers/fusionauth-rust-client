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

/// UserRequest : User API request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRequest {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "currentPassword", skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    #[serde(rename = "disableDomainBlock", skip_serializing_if = "Option::is_none")]
    pub disable_domain_block: Option<bool>,
    #[serde(rename = "sendSetPasswordEmail", skip_serializing_if = "Option::is_none")]
    pub send_set_password_email: Option<bool>,
    #[serde(rename = "skipVerification", skip_serializing_if = "Option::is_none")]
    pub skip_verification: Option<bool>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::User>>,
    #[serde(rename = "eventInfo", skip_serializing_if = "Option::is_none")]
    pub event_info: Option<Box<models::EventInfo>>,
}

impl UserRequest {
    /// User API request object.
    pub fn new() -> UserRequest {
        UserRequest {
            application_id: None,
            current_password: None,
            disable_domain_block: None,
            send_set_password_email: None,
            skip_verification: None,
            user: None,
            event_info: None,
        }
    }
}

