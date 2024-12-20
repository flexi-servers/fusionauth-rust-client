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

/// EventRequest : Container for the event information. This is the JSON that is sent from FusionAuth to webhooks.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventRequest {
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Box<models::BaseEvent>>,
}

impl EventRequest {
    /// Container for the event information. This is the JSON that is sent from FusionAuth to webhooks.
    pub fn new() -> EventRequest {
        EventRequest {
            event: None,
        }
    }
}

