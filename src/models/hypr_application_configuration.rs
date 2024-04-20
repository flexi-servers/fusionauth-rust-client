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

/// HyprApplicationConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyprApplicationConfiguration {
    #[serde(rename = "relyingPartyApplicationId", skip_serializing_if = "Option::is_none")]
    pub relying_party_application_id: Option<String>,
    #[serde(rename = "relyingPartyURL", skip_serializing_if = "Option::is_none")]
    pub relying_party_url: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "createRegistration", skip_serializing_if = "Option::is_none")]
    pub create_registration: Option<bool>,
}

impl HyprApplicationConfiguration {
    /// 
    pub fn new() -> HyprApplicationConfiguration {
        HyprApplicationConfiguration {
            relying_party_application_id: None,
            relying_party_url: None,
            data: None,
            create_registration: None,
        }
    }
}

