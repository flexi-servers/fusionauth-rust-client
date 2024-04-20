/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// KeyResponse : Key API response object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyResponse {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<Box<models::Key>>,
    #[serde(rename = "keys", skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<models::Key>>,
}

impl KeyResponse {
    /// Key API response object.
    pub fn new() -> KeyResponse {
        KeyResponse {
            key: None,
            keys: None,
        }
    }
}

