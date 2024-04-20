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

/// DeviceResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceResponse {
    #[serde(rename = "device_code", skip_serializing_if = "Option::is_none")]
    pub device_code: Option<String>,
    #[serde(rename = "expires_in", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "user_code", skip_serializing_if = "Option::is_none")]
    pub user_code: Option<String>,
    #[serde(rename = "verification_uri", skip_serializing_if = "Option::is_none")]
    pub verification_uri: Option<String>,
    #[serde(rename = "verification_uri_complete", skip_serializing_if = "Option::is_none")]
    pub verification_uri_complete: Option<String>,
}

impl DeviceResponse {
    /// 
    pub fn new() -> DeviceResponse {
        DeviceResponse {
            device_code: None,
            expires_in: None,
            interval: None,
            user_code: None,
            verification_uri: None,
            verification_uri_complete: None,
        }
    }
}

