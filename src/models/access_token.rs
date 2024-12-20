/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.54.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AccessToken : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessToken {
    #[serde(rename = "expires_in", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
    #[serde(rename = "id_token", skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    #[serde(rename = "refresh_token", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "refresh_token_id", skip_serializing_if = "Option::is_none")]
    pub refresh_token_id: Option<uuid::Uuid>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "token_type", skip_serializing_if = "Option::is_none")]
    pub token_type: Option<models::TokenType>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl AccessToken {
    /// 
    pub fn new() -> AccessToken {
        AccessToken {
            expires_in: None,
            id_token: None,
            refresh_token: None,
            refresh_token_id: None,
            scope: None,
            access_token: None,
            token_type: None,
            user_id: None,
        }
    }
}

