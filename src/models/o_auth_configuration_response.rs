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

/// OAuthConfigurationResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthConfigurationResponse {
    #[serde(rename = "httpSessionMaxInactiveInterval", skip_serializing_if = "Option::is_none")]
    pub http_session_max_inactive_interval: Option<i32>,
    #[serde(rename = "logoutURL", skip_serializing_if = "Option::is_none")]
    pub logout_url: Option<String>,
    #[serde(rename = "oauthConfiguration", skip_serializing_if = "Option::is_none")]
    pub oauth_configuration: Option<Box<models::OAuth2Configuration>>,
}

impl OAuthConfigurationResponse {
    /// 
    pub fn new() -> OAuthConfigurationResponse {
        OAuthConfigurationResponse {
            http_session_max_inactive_interval: None,
            logout_url: None,
            oauth_configuration: None,
        }
    }
}

