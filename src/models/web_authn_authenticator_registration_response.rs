/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebAuthnAuthenticatorRegistrationResponse : The <i>authenticator's<i> response for the registration ceremony in its encoded format



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnAuthenticatorRegistrationResponse {
    #[serde(rename = "attestationObject", skip_serializing_if = "Option::is_none")]
    pub attestation_object: Option<String>,
    #[serde(rename = "clientDataJSON", skip_serializing_if = "Option::is_none")]
    pub client_data_json: Option<String>,
}

impl WebAuthnAuthenticatorRegistrationResponse {
    /// The <i>authenticator's<i> response for the registration ceremony in its encoded format
    pub fn new() -> WebAuthnAuthenticatorRegistrationResponse {
        WebAuthnAuthenticatorRegistrationResponse {
            attestation_object: None,
            client_data_json: None,
        }
    }
}


