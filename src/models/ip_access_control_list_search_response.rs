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

/// IpAccessControlListSearchResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAccessControlListSearchResponse {
    #[serde(rename = "ipAccessControlLists", skip_serializing_if = "Option::is_none")]
    pub ip_access_control_lists: Option<Vec<models::IpAccessControlList>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl IpAccessControlListSearchResponse {
    /// 
    pub fn new() -> IpAccessControlListSearchResponse {
        IpAccessControlListSearchResponse {
            ip_access_control_lists: None,
            total: None,
        }
    }
}

