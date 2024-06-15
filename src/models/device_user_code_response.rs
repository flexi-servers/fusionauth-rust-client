/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DeviceUserCodeResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceUserCodeResponse {
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "deviceInfo", skip_serializing_if = "Option::is_none")]
    pub device_info: Option<Box<models::DeviceInfo>>,
    #[serde(rename = "expires_in", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
    #[serde(rename = "pendingIdPLink", skip_serializing_if = "Option::is_none")]
    pub pending_id_p_link: Option<Box<models::PendingIdPLink>>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<uuid::Uuid>,
    #[serde(rename = "user_code", skip_serializing_if = "Option::is_none")]
    pub user_code: Option<String>,
}

impl DeviceUserCodeResponse {
    /// 
    pub fn new() -> DeviceUserCodeResponse {
        DeviceUserCodeResponse {
            client_id: None,
            device_info: None,
            expires_in: None,
            pending_id_p_link: None,
            scope: None,
            tenant_id: None,
            user_code: None,
        }
    }
}

