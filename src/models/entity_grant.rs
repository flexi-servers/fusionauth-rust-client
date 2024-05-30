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

/// EntityGrant : A grant for an entity to a user or another entity.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityGrant {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<Box<models::Entity>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<serde_json::Value>>,
    #[serde(rename = "recipientEntityId", skip_serializing_if = "Option::is_none")]
    pub recipient_entity_id: Option<uuid::Uuid>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl EntityGrant {
    /// A grant for an entity to a user or another entity.
    pub fn new() -> EntityGrant {
        EntityGrant {
            data: None,
            entity: None,
            id: None,
            insert_instant: None,
            last_update_instant: None,
            permissions: None,
            recipient_entity_id: None,
            user_id: None,
        }
    }
}

