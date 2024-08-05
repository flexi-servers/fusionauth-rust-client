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

/// EventConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventConfiguration {
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<std::collections::HashMap<String, models::EventConfigurationData>>,
}

impl EventConfiguration {
    /// 
    pub fn new() -> EventConfiguration {
        EventConfiguration {
            events: None,
        }
    }
}

