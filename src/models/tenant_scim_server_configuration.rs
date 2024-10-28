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

/// TenantScimServerConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantScimServerConfiguration {
    #[serde(rename = "clientEntityTypeId", skip_serializing_if = "Option::is_none")]
    pub client_entity_type_id: Option<uuid::Uuid>,
    #[serde(rename = "schemas", skip_serializing_if = "Option::is_none")]
    pub schemas: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "serverEntityTypeId", skip_serializing_if = "Option::is_none")]
    pub server_entity_type_id: Option<uuid::Uuid>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl TenantScimServerConfiguration {
    /// 
    pub fn new() -> TenantScimServerConfiguration {
        TenantScimServerConfiguration {
            client_entity_type_id: None,
            schemas: None,
            server_entity_type_id: None,
            enabled: None,
        }
    }
}

