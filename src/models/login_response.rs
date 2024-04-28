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

/// LoginResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<models::LoginPreventedResponse>>,
    #[serde(rename = "changePasswordId", skip_serializing_if = "Option::is_none")]
    pub change_password_id: Option<String>,
    #[serde(rename = "changePasswordReason", skip_serializing_if = "Option::is_none")]
    pub change_password_reason: Option<models::ChangePasswordReason>,
    #[serde(rename = "configurableMethods", skip_serializing_if = "Option::is_none")]
    pub configurable_methods: Option<Vec<String>>,
    #[serde(rename = "emailVerificationId", skip_serializing_if = "Option::is_none")]
    pub email_verification_id: Option<String>,
    #[serde(rename = "methods", skip_serializing_if = "Option::is_none")]
    pub methods: Option<Vec<models::TwoFactorMethod>>,
    #[serde(rename = "pendingIdPLinkId", skip_serializing_if = "Option::is_none")]
    pub pending_id_p_link_id: Option<String>,
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "refreshTokenId", skip_serializing_if = "Option::is_none")]
    pub refresh_token_id: Option<uuid::Uuid>,
    #[serde(rename = "registrationVerificationId", skip_serializing_if = "Option::is_none")]
    pub registration_verification_id: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "threatsDetected", skip_serializing_if = "Option::is_none")]
    pub threats_detected: Option<Vec<serde_json::Value>>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "tokenExpirationInstant", skip_serializing_if = "Option::is_none")]
    pub token_expiration_instant: Option<i64>,
    #[serde(rename = "trustToken", skip_serializing_if = "Option::is_none")]
    pub trust_token: Option<String>,
    #[serde(rename = "twoFactorId", skip_serializing_if = "Option::is_none")]
    pub two_factor_id: Option<String>,
    #[serde(rename = "twoFactorTrustId", skip_serializing_if = "Option::is_none")]
    pub two_factor_trust_id: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::User>>,
}

impl LoginResponse {
    /// 
    pub fn new() -> LoginResponse {
        LoginResponse {
            actions: None,
            change_password_id: None,
            change_password_reason: None,
            configurable_methods: None,
            email_verification_id: None,
            methods: None,
            pending_id_p_link_id: None,
            refresh_token: None,
            refresh_token_id: None,
            registration_verification_id: None,
            state: None,
            threats_detected: None,
            token: None,
            token_expiration_instant: None,
            trust_token: None,
            two_factor_id: None,
            two_factor_trust_id: None,
            user: None,
        }
    }
}

