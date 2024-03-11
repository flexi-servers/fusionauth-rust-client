/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// SendRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendRequest {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "bccAddresses", skip_serializing_if = "Option::is_none")]
    pub bcc_addresses: Option<Vec<String>>,
    #[serde(rename = "ccAddresses", skip_serializing_if = "Option::is_none")]
    pub cc_addresses: Option<Vec<String>>,
    #[serde(rename = "preferredLanguages", skip_serializing_if = "Option::is_none")]
    pub preferred_languages: Option<Vec<String>>,
    #[serde(rename = "requestData", skip_serializing_if = "Option::is_none")]
    pub request_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "toAddresses", skip_serializing_if = "Option::is_none")]
    pub to_addresses: Option<Vec<models::EmailAddress>>,
    #[serde(rename = "userIds", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<uuid::Uuid>>,
}

impl SendRequest {
    /// 
    pub fn new() -> SendRequest {
        SendRequest {
            application_id: None,
            bcc_addresses: None,
            cc_addresses: None,
            preferred_languages: None,
            request_data: None,
            to_addresses: None,
            user_ids: None,
        }
    }
}

