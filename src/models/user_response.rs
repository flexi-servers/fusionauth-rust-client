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

/// UserResponse : User API response object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "emailVerificationId", skip_serializing_if = "Option::is_none")]
    pub email_verification_id: Option<String>,
    #[serde(rename = "emailVerificationOneTimeCode", skip_serializing_if = "Option::is_none")]
    pub email_verification_one_time_code: Option<String>,
    #[serde(rename = "registrationVerificationIds", skip_serializing_if = "Option::is_none")]
    pub registration_verification_ids: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "registrationVerificationOneTimeCodes", skip_serializing_if = "Option::is_none")]
    pub registration_verification_one_time_codes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "tokenExpirationInstant", skip_serializing_if = "Option::is_none")]
    pub token_expiration_instant: Option<i64>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::User>>,
}

impl UserResponse {
    /// User API response object.
    pub fn new() -> UserResponse {
        UserResponse {
            email_verification_id: None,
            email_verification_one_time_code: None,
            registration_verification_ids: None,
            registration_verification_one_time_codes: None,
            token: None,
            token_expiration_instant: None,
            user: None,
        }
    }
}

