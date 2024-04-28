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

/// Enableable : Something that can be enabled and thus also disabled.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Enableable {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Enableable {
    /// Something that can be enabled and thus also disabled.
    pub fn new() -> Enableable {
        Enableable {
            enabled: None,
        }
    }
}

