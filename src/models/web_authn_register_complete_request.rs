/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebAuthnRegisterCompleteRequest : Request to complete the WebAuthn registration ceremony for a new credential,.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnRegisterCompleteRequest {
    #[serde(rename = "credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<Box<crate::models::WebAuthnPublicKeyRegistrationRequest>>,
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "rpId", skip_serializing_if = "Option::is_none")]
    pub rp_id: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl WebAuthnRegisterCompleteRequest {
    /// Request to complete the WebAuthn registration ceremony for a new credential,.
    pub fn new() -> WebAuthnRegisterCompleteRequest {
        WebAuthnRegisterCompleteRequest {
            credential: None,
            origin: None,
            rp_id: None,
            user_id: None,
        }
    }
}


