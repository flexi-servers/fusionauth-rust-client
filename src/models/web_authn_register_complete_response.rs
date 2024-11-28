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

/// WebAuthnRegisterCompleteResponse : API response for completing WebAuthn credential registration or assertion
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnRegisterCompleteResponse {
    #[serde(rename = "credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<Box<models::WebAuthnCredential>>,
}

impl WebAuthnRegisterCompleteResponse {
    /// API response for completing WebAuthn credential registration or assertion
    pub fn new() -> WebAuthnRegisterCompleteResponse {
        WebAuthnRegisterCompleteResponse {
            credential: None,
        }
    }
}

