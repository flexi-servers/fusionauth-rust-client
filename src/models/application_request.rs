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

/// ApplicationRequest : The Application API request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationRequest {
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<models::Application>>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<models::ApplicationRole>>,
    #[serde(rename = "sourceApplicationId", skip_serializing_if = "Option::is_none")]
    pub source_application_id: Option<uuid::Uuid>,
    #[serde(rename = "eventInfo", skip_serializing_if = "Option::is_none")]
    pub event_info: Option<Box<models::EventInfo>>,
}

impl ApplicationRequest {
    /// The Application API request object.
    pub fn new() -> ApplicationRequest {
        ApplicationRequest {
            application: None,
            role: None,
            source_application_id: None,
            event_info: None,
        }
    }
}

