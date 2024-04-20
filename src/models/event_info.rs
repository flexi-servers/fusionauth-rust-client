/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// EventInfo : Information about a user event (login, register, etc) that helps identify the source of the event (location, device type, OS, etc).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventInfo {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "deviceDescription", skip_serializing_if = "Option::is_none")]
    pub device_description: Option<String>,
    #[serde(rename = "deviceName", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "deviceType", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<models::Location>>,
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

impl EventInfo {
    /// Information about a user event (login, register, etc) that helps identify the source of the event (location, device type, OS, etc).
    pub fn new() -> EventInfo {
        EventInfo {
            data: None,
            device_description: None,
            device_name: None,
            device_type: None,
            ip_address: None,
            location: None,
            os: None,
            user_agent: None,
        }
    }
}

