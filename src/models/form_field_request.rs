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

/// FormFieldRequest : The FormField API request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormFieldRequest {
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<Box<models::FormField>>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<models::FormField>>,
}

impl FormFieldRequest {
    /// The FormField API request object.
    pub fn new() -> FormFieldRequest {
        FormFieldRequest {
            field: None,
            fields: None,
        }
    }
}

