/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.52.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EntityType : Models an entity type that has a specific set of permissions. These are global objects and can be used across tenants.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityType {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    #[serde(rename = "jwtConfiguration", skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<Box<models::EntityJwtConfiguration>>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<models::EntityTypePermission>>,
}

impl EntityType {
    /// Models an entity type that has a specific set of permissions. These are global objects and can be used across tenants.
    pub fn new() -> EntityType {
        EntityType {
            data: None,
            id: None,
            insert_instant: None,
            jwt_configuration: None,
            last_update_instant: None,
            name: None,
            permissions: None,
        }
    }
}

