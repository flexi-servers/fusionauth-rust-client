/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.54.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Errors : Standard error domain object that can also be used as the response from an API call.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Errors {
    #[serde(rename = "fieldErrors", skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<Vec<models::Error>>,
    #[serde(rename = "generalErrors", skip_serializing_if = "Option::is_none")]
    pub general_errors: Option<Vec<models::Error>>,
}

impl Errors {
    /// Standard error domain object that can also be used as the response from an API call.
    pub fn new() -> Errors {
        Errors {
            field_errors: None,
            general_errors: None,
        }
    }
}

