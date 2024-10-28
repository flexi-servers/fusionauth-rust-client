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

/// PasswordlessSendRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordlessSendRequest {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "loginId", skip_serializing_if = "Option::is_none")]
    pub login_id: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl PasswordlessSendRequest {
    /// 
    pub fn new() -> PasswordlessSendRequest {
        PasswordlessSendRequest {
            application_id: None,
            code: None,
            login_id: None,
            state: None,
        }
    }
}

