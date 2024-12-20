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

/// ConsentResponse : API response for consent.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsentResponse {
    #[serde(rename = "consent", skip_serializing_if = "Option::is_none")]
    pub consent: Option<Box<models::Consent>>,
    #[serde(rename = "consents", skip_serializing_if = "Option::is_none")]
    pub consents: Option<Vec<models::Consent>>,
}

impl ConsentResponse {
    /// API response for consent.
    pub fn new() -> ConsentResponse {
        ConsentResponse {
            consent: None,
            consents: None,
        }
    }
}

