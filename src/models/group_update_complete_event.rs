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

/// GroupUpdateCompleteEvent : Models the Group Update Complete Event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupUpdateCompleteEvent {
    #[serde(rename = "original", skip_serializing_if = "Option::is_none")]
    pub original: Option<Box<models::Group>>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<models::Group>>,
}

impl GroupUpdateCompleteEvent {
    /// Models the Group Update Complete Event.
    pub fn new() -> GroupUpdateCompleteEvent {
        GroupUpdateCompleteEvent {
            original: None,
            group: None,
        }
    }
}

