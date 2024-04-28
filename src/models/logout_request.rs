/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// LogoutRequest : Request for the Logout API that can be used as an alternative to URL parameters.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogoutRequest {
    #[serde(rename = "global", skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "eventInfo", skip_serializing_if = "Option::is_none")]
    pub event_info: Option<Box<models::EventInfo>>,
}

impl LogoutRequest {
    /// Request for the Logout API that can be used as an alternative to URL parameters.
    pub fn new() -> LogoutRequest {
        LogoutRequest {
            global: None,
            refresh_token: None,
            event_info: None,
        }
    }
}

