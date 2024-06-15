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

/// FailedAuthenticationActionCancelPolicy : A policy to configure if and when the user-action is canceled prior to the expiration of the action.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedAuthenticationActionCancelPolicy {
    #[serde(rename = "onPasswordReset", skip_serializing_if = "Option::is_none")]
    pub on_password_reset: Option<bool>,
}

impl FailedAuthenticationActionCancelPolicy {
    /// A policy to configure if and when the user-action is canceled prior to the expiration of the action.
    pub fn new() -> FailedAuthenticationActionCancelPolicy {
        FailedAuthenticationActionCancelPolicy {
            on_password_reset: None,
        }
    }
}

