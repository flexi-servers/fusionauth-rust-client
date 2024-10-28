/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WebhookSignatureConfiguration : Configuration for signing webhooks.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookSignatureConfiguration {
    #[serde(rename = "signingKeyId", skip_serializing_if = "Option::is_none")]
    pub signing_key_id: Option<uuid::Uuid>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl WebhookSignatureConfiguration {
    /// Configuration for signing webhooks.
    pub fn new() -> WebhookSignatureConfiguration {
        WebhookSignatureConfiguration {
            signing_key_id: None,
            enabled: None,
        }
    }
}

