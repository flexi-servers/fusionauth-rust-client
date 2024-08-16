/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FacebookApplicationConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacebookApplicationConfiguration {
    #[serde(rename = "appId", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "buttonText", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(rename = "loginMethod", skip_serializing_if = "Option::is_none")]
    pub login_method: Option<models::IdentityProviderLoginMethod>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "createRegistration", skip_serializing_if = "Option::is_none")]
    pub create_registration: Option<bool>,
}

impl FacebookApplicationConfiguration {
    /// 
    pub fn new() -> FacebookApplicationConfiguration {
        FacebookApplicationConfiguration {
            app_id: None,
            button_text: None,
            client_secret: None,
            fields: None,
            login_method: None,
            permissions: None,
            data: None,
            create_registration: None,
        }
    }
}

