/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CleanSpeakConfiguration : CleanSpeak configuration at the system and application level.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CleanSpeakConfiguration {
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename = "applicationIds", skip_serializing_if = "Option::is_none")]
    pub application_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "usernameModeration", skip_serializing_if = "Option::is_none")]
    pub username_moderation: Option<Box<models::UsernameModeration>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl CleanSpeakConfiguration {
    /// CleanSpeak configuration at the system and application level.
    pub fn new() -> CleanSpeakConfiguration {
        CleanSpeakConfiguration {
            api_key: None,
            application_ids: None,
            url: None,
            username_moderation: None,
            enabled: None,
        }
    }
}

