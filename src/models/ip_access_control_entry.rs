/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IpAccessControlEntry : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAccessControlEntry {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<models::IpAccessControlEntryAction>,
    #[serde(rename = "endIPAddress", skip_serializing_if = "Option::is_none")]
    pub end_ip_address: Option<String>,
    #[serde(rename = "startIPAddress", skip_serializing_if = "Option::is_none")]
    pub start_ip_address: Option<String>,
}

impl IpAccessControlEntry {
    /// 
    pub fn new() -> IpAccessControlEntry {
        IpAccessControlEntry {
            action: None,
            end_ip_address: None,
            start_ip_address: None,
        }
    }
}

