/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UniqueUsernameConfiguration {
    #[serde(rename = "numberOfDigits", skip_serializing_if = "Option::is_none")]
    pub number_of_digits: Option<i32>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<models::UniqueUsernameStrategy>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl UniqueUsernameConfiguration {
    pub fn new() -> UniqueUsernameConfiguration {
        UniqueUsernameConfiguration {
            number_of_digits: None,
            separator: None,
            strategy: None,
            enabled: None,
        }
    }
}

