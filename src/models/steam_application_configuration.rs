/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SteamApplicationConfiguration : 



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SteamApplicationConfiguration {
    #[serde(rename = "apiMode", skip_serializing_if = "Option::is_none")]
    pub api_mode: Option<crate::models::SteamApiMode>,
    #[serde(rename = "buttonText", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "webAPIKey", skip_serializing_if = "Option::is_none")]
    pub web_api_key: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "createRegistration", skip_serializing_if = "Option::is_none")]
    pub create_registration: Option<bool>,
}

impl SteamApplicationConfiguration {
    /// 
    pub fn new() -> SteamApplicationConfiguration {
        SteamApplicationConfiguration {
            api_mode: None,
            button_text: None,
            client_id: None,
            scope: None,
            web_api_key: None,
            data: None: None,
            create_registration: None,
        }
    }
}


