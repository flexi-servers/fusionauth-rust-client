/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DeviceInfo : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceInfo {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastAccessedAddress", skip_serializing_if = "Option::is_none")]
    pub last_accessed_address: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastAccessedInstant", skip_serializing_if = "Option::is_none")]
    pub last_accessed_instant: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl DeviceInfo {
    /// 
    pub fn new() -> DeviceInfo {
        DeviceInfo {
            description: None,
            last_accessed_address: None,
            last_accessed_instant: None,
            name: None,
            r#type: None,
        }
    }
}

