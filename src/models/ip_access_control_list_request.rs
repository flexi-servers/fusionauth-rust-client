/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IpAccessControlListRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAccessControlListRequest {
    #[serde(rename = "ipAccessControlList", skip_serializing_if = "Option::is_none")]
    pub ip_access_control_list: Option<Box<models::IpAccessControlList>>,
}

impl IpAccessControlListRequest {
    /// 
    pub fn new() -> IpAccessControlListRequest {
        IpAccessControlListRequest {
            ip_access_control_list: None,
        }
    }
}

