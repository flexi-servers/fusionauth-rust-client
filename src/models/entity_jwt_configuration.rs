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

/// EntityJwtConfiguration : JWT Configuration for entities.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityJwtConfiguration {
    #[serde(rename = "accessTokenKeyId", skip_serializing_if = "Option::is_none")]
    pub access_token_key_id: Option<uuid::Uuid>,
    #[serde(rename = "timeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl EntityJwtConfiguration {
    /// JWT Configuration for entities.
    pub fn new() -> EntityJwtConfiguration {
        EntityJwtConfiguration {
            access_token_key_id: None,
            time_to_live_in_seconds: None,
            enabled: None,
        }
    }
}

