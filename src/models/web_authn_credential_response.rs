/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// WebAuthnCredentialResponse : WebAuthn Credential API response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnCredentialResponse {
    #[serde(rename = "credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<Box<models::WebAuthnCredential>>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<models::WebAuthnCredential>>,
}

impl WebAuthnCredentialResponse {
    /// WebAuthn Credential API response
    pub fn new() -> WebAuthnCredentialResponse {
        WebAuthnCredentialResponse {
            credential: None,
            credentials: None,
        }
    }
}

