/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NintendoApplicationConfiguration : 



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NintendoApplicationConfiguration {
    #[serde(rename = "buttonText", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "emailClaim", skip_serializing_if = "Option::is_none")]
    pub email_claim: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "uniqueIdClaim", skip_serializing_if = "Option::is_none")]
    pub unique_id_claim: Option<String>,
    #[serde(rename = "usernameClaim", skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "createRegistration", skip_serializing_if = "Option::is_none")]
    pub create_registration: Option<bool>,
}

impl NintendoApplicationConfiguration {
    /// 
    pub fn new() -> NintendoApplicationConfiguration {
        NintendoApplicationConfiguration {
            button_text: None,
            client_id: None,
            client_secret: None,
            email_claim: None,
            scope: None,
            unique_id_claim: None,
            username_claim: None,
            data: None: None,
            create_registration: None,
        }
    }
}


