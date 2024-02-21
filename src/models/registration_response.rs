/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RegistrationResponse : Registration API request object.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationResponse {
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "registration", skip_serializing_if = "Option::is_none")]
    pub registration: Option<Box<crate::models::UserRegistration>>,
    #[serde(rename = "registrationVerificationId", skip_serializing_if = "Option::is_none")]
    pub registration_verification_id: Option<String>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "tokenExpirationInstant", skip_serializing_if = "Option::is_none")]
    pub token_expiration_instant: Option<i64>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
}

impl RegistrationResponse {
    /// Registration API request object.
    pub fn new() -> RegistrationResponse {
        RegistrationResponse {
            refresh_token: None,
            registration: None,
            registration_verification_id: None,
            token: None,
            token_expiration_instant: None,
            user: None,
        }
    }
}


