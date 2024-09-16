/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LambdaRequest : Lambda API request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LambdaRequest {
    #[serde(rename = "lambda", skip_serializing_if = "Option::is_none")]
    pub lambda: Option<Box<models::Lambda>>,
}

impl LambdaRequest {
    /// Lambda API request object.
    pub fn new() -> LambdaRequest {
        LambdaRequest {
            lambda: None,
        }
    }
}

