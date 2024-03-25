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
pub struct EventConfigurationData {
    #[serde(rename = "transactionType", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<models::TransactionType>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl EventConfigurationData {
    pub fn new() -> EventConfigurationData {
        EventConfigurationData {
            transaction_type: None,
            enabled: None,
        }
    }
}

