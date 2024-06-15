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

/// WebAuthnAuthenticatorAuthenticationResponse : The <i>authenticator's<i> response for the authentication ceremony in its encoded format
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnAuthenticatorAuthenticationResponse {
    #[serde(rename = "authenticatorData", skip_serializing_if = "Option::is_none")]
    pub authenticator_data: Option<String>,
    #[serde(rename = "clientDataJSON", skip_serializing_if = "Option::is_none")]
    pub client_data_json: Option<String>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "userHandle", skip_serializing_if = "Option::is_none")]
    pub user_handle: Option<String>,
}

impl WebAuthnAuthenticatorAuthenticationResponse {
    /// The <i>authenticator's<i> response for the authentication ceremony in its encoded format
    pub fn new() -> WebAuthnAuthenticatorAuthenticationResponse {
        WebAuthnAuthenticatorAuthenticationResponse {
            authenticator_data: None,
            client_data_json: None,
            signature: None,
            user_handle: None,
        }
    }
}

