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

/// LambdaSearchResponse : Lambda search response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LambdaSearchResponse {
    #[serde(rename = "lambdas", skip_serializing_if = "Option::is_none")]
    pub lambdas: Option<Vec<models::Lambda>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl LambdaSearchResponse {
    /// Lambda search response
    pub fn new() -> LambdaSearchResponse {
        LambdaSearchResponse {
            lambdas: None,
            total: None,
        }
    }
}

