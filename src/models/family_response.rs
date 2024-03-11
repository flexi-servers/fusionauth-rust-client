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

/// FamilyResponse : API response for managing families and members.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FamilyResponse {
    #[serde(rename = "families", skip_serializing_if = "Option::is_none")]
    pub families: Option<Vec<models::Family>>,
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<Box<models::Family>>,
}

impl FamilyResponse {
    /// API response for managing families and members.
    pub fn new() -> FamilyResponse {
        FamilyResponse {
            families: None,
            family: None,
        }
    }
}

