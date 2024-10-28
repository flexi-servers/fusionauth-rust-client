/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserRegistrationVerifiedEvent : Models the User Registration Verified Event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRegistrationVerifiedEvent {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "registration", skip_serializing_if = "Option::is_none")]
    pub registration: Option<Box<models::UserRegistration>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::User>>,
}

impl UserRegistrationVerifiedEvent {
    /// Models the User Registration Verified Event.
    pub fn new() -> UserRegistrationVerifiedEvent {
        UserRegistrationVerifiedEvent {
            application_id: None,
            registration: None,
            user: None,
        }
    }
}

