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

/// ReactorRequest : Request for managing FusionAuth Reactor and licenses.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReactorRequest {
    #[serde(rename = "license", skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(rename = "licenseId", skip_serializing_if = "Option::is_none")]
    pub license_id: Option<String>,
}

impl ReactorRequest {
    /// Request for managing FusionAuth Reactor and licenses.
    pub fn new() -> ReactorRequest {
        ReactorRequest {
            license: None,
            license_id: None,
        }
    }
}

