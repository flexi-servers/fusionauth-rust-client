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

/// UserPasswordResetStartEvent : Models the User Password Reset Start Event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserPasswordResetStartEvent {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::User>>,
}

impl UserPasswordResetStartEvent {
    /// Models the User Password Reset Start Event.
    pub fn new() -> UserPasswordResetStartEvent {
        UserPasswordResetStartEvent {
            user: None,
        }
    }
}

