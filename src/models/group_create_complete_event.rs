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

/// GroupCreateCompleteEvent : Models the Group Created Event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupCreateCompleteEvent {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<models::Group>>,
}

impl GroupCreateCompleteEvent {
    /// Models the Group Created Event.
    pub fn new() -> GroupCreateCompleteEvent {
        GroupCreateCompleteEvent {
            group: None,
        }
    }
}

