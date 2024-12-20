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

/// ActionRequest : The user action request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRequest {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<models::ActionData>>,
    #[serde(rename = "broadcast", skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<bool>,
    #[serde(rename = "eventInfo", skip_serializing_if = "Option::is_none")]
    pub event_info: Option<Box<models::EventInfo>>,
}

impl ActionRequest {
    /// The user action request object.
    pub fn new() -> ActionRequest {
        ActionRequest {
            action: None,
            broadcast: None,
            event_info: None,
        }
    }
}

