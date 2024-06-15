/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GroupRequest : Group API request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupRequest {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<models::Group>>,
    #[serde(rename = "roleIds", skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<uuid::Uuid>>,
}

impl GroupRequest {
    /// Group API request object.
    pub fn new() -> GroupRequest {
        GroupRequest {
            group: None,
            role_ids: None,
        }
    }
}

