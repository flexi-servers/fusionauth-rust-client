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

/// GroupResponse : Group API response object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupResponse {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<models::Group>>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<models::Group>>,
}

impl GroupResponse {
    /// Group API response object.
    pub fn new() -> GroupResponse {
        GroupResponse {
            group: None,
            groups: None,
        }
    }
}

