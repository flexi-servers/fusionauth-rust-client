/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// EntityRequest : Entity API request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityRequest {
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<Box<models::Entity>>,
}

impl EntityRequest {
    /// Entity API request object.
    pub fn new() -> EntityRequest {
        EntityRequest {
            entity: None,
        }
    }
}

