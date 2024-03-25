/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// EntityTypeRequest : Entity Type API request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityTypeRequest {
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<Box<models::EntityType>>,
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<Box<models::EntityTypePermission>>,
}

impl EntityTypeRequest {
    /// Entity Type API request object.
    pub fn new() -> EntityTypeRequest {
        EntityTypeRequest {
            entity_type: None,
            permission: None,
        }
    }
}

