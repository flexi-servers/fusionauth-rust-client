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

/// JwtVendRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtVendRequest {
    #[serde(rename = "claims", skip_serializing_if = "Option::is_none")]
    pub claims: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "keyId", skip_serializing_if = "Option::is_none")]
    pub key_id: Option<uuid::Uuid>,
    #[serde(rename = "timeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub time_to_live_in_seconds: Option<i32>,
}

impl JwtVendRequest {
    /// 
    pub fn new() -> JwtVendRequest {
        JwtVendRequest {
            claims: None,
            key_id: None,
            time_to_live_in_seconds: None,
        }
    }
}

