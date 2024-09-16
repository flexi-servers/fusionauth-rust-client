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

/// WebhookAttemptResult : The possible states of an individual webhook attempt to a single endpoint.
/// The possible states of an individual webhook attempt to a single endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebhookAttemptResult {
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "Failure")]
    Failure,
    #[serde(rename = "Unknown")]
    Unknown,

}

impl std::fmt::Display for WebhookAttemptResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "Success"),
            Self::Failure => write!(f, "Failure"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Default for WebhookAttemptResult {
    fn default() -> WebhookAttemptResult {
        Self::Success
    }
}

