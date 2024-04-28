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

/// LambdaSearchCriteria : Search criteria for Lambdas
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LambdaSearchCriteria {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::LambdaType>,
    #[serde(rename = "numberOfResults", skip_serializing_if = "Option::is_none")]
    pub number_of_results: Option<i32>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "startRow", skip_serializing_if = "Option::is_none")]
    pub start_row: Option<i32>,
}

impl LambdaSearchCriteria {
    /// Search criteria for Lambdas
    pub fn new() -> LambdaSearchCriteria {
        LambdaSearchCriteria {
            body: None,
            name: None,
            r#type: None,
            number_of_results: None,
            order_by: None,
            start_row: None,
        }
    }
}

