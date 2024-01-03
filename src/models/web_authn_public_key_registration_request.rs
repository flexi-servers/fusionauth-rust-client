/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebAuthnPublicKeyRegistrationRequest : Request to register a new public key with WebAuthn



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnPublicKeyRegistrationRequest {
    #[serde(rename = "clientExtensionResults", skip_serializing_if = "Option::is_none")]
    pub client_extension_results: Option<Box<crate::models::WebAuthnExtensionsClientOutputs>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "rpId", skip_serializing_if = "Option::is_none")]
    pub rp_id: Option<String>,
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<Box<crate::models::WebAuthnAuthenticatorRegistrationResponse>>,
    #[serde(rename = "transports", skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl WebAuthnPublicKeyRegistrationRequest {
    /// Request to register a new public key with WebAuthn
    pub fn new() -> WebAuthnPublicKeyRegistrationRequest {
        WebAuthnPublicKeyRegistrationRequest {
            client_extension_results: None,
            id: None,
            rp_id: None,
            response: None,
            transports: None,
            r#type: None,
        }
    }
}

