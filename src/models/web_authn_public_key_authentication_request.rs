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

/// WebAuthnPublicKeyAuthenticationRequest : Request to authenticate with WebAuthn
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnPublicKeyAuthenticationRequest {
    #[serde(rename = "clientExtensionResults", skip_serializing_if = "Option::is_none")]
    pub client_extension_results: Option<Box<models::WebAuthnExtensionsClientOutputs>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "rpId", skip_serializing_if = "Option::is_none")]
    pub rp_id: Option<String>,
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<Box<models::WebAuthnAuthenticatorAuthenticationResponse>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl WebAuthnPublicKeyAuthenticationRequest {
    /// Request to authenticate with WebAuthn
    pub fn new() -> WebAuthnPublicKeyAuthenticationRequest {
        WebAuthnPublicKeyAuthenticationRequest {
            client_extension_results: None,
            id: None,
            rp_id: None,
            response: None,
            r#type: None,
        }
    }
}

