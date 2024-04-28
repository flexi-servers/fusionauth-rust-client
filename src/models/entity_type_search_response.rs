/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// EntityTypeSearchResponse : Search response for entity types.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityTypeSearchResponse {
    #[serde(rename = "entityTypes", skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<models::EntityType>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl EntityTypeSearchResponse {
    /// Search response for entity types.
    pub fn new() -> EntityTypeSearchResponse {
        EntityTypeSearchResponse {
            entity_types: None,
            total: None,
        }
    }
}

