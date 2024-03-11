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

/// FormResponse : Form response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormResponse {
    #[serde(rename = "form", skip_serializing_if = "Option::is_none")]
    pub form: Option<Box<models::Form>>,
    #[serde(rename = "forms", skip_serializing_if = "Option::is_none")]
    pub forms: Option<Vec<models::Form>>,
}

impl FormResponse {
    /// Form response.
    pub fn new() -> FormResponse {
        FormResponse {
            form: None,
            forms: None,
        }
    }
}

