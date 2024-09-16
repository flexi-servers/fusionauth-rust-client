/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserConsentResponse : API response for User consent.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserConsentResponse {
    #[serde(rename = "userConsent", skip_serializing_if = "Option::is_none")]
    pub user_consent: Option<Box<models::UserConsent>>,
    #[serde(rename = "userConsents", skip_serializing_if = "Option::is_none")]
    pub user_consents: Option<Vec<models::UserConsent>>,
}

impl UserConsentResponse {
    /// API response for User consent.
    pub fn new() -> UserConsentResponse {
        UserConsentResponse {
            user_consent: None,
            user_consents: None,
        }
    }
}

