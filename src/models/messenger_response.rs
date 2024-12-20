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

/// MessengerResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessengerResponse {
    #[serde(rename = "messenger", skip_serializing_if = "Option::is_none")]
    pub messenger: Option<Box<models::BaseMessengerConfiguration>>,
    #[serde(rename = "messengers", skip_serializing_if = "Option::is_none")]
    pub messengers: Option<Vec<models::BaseMessengerConfiguration>>,
}

impl MessengerResponse {
    /// 
    pub fn new() -> MessengerResponse {
        MessengerResponse {
            messenger: None,
            messengers: None,
        }
    }
}

