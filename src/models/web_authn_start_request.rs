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

/// WebAuthnStartRequest : API request to start a WebAuthn authentication ceremony
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnStartRequest {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "credentialId", skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<uuid::Uuid>,
    #[serde(rename = "loginId", skip_serializing_if = "Option::is_none")]
    pub login_id: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(rename = "workflow", skip_serializing_if = "Option::is_none")]
    pub workflow: Option<models::WebAuthnWorkflow>,
}

impl WebAuthnStartRequest {
    /// API request to start a WebAuthn authentication ceremony
    pub fn new() -> WebAuthnStartRequest {
        WebAuthnStartRequest {
            application_id: None,
            credential_id: None,
            login_id: None,
            state: None,
            user_id: None,
            workflow: None,
        }
    }
}

