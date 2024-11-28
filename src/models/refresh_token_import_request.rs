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

/// RefreshTokenImportRequest : Refresh Token Import request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RefreshTokenImportRequest {
    #[serde(rename = "refreshTokens", skip_serializing_if = "Option::is_none")]
    pub refresh_tokens: Option<Vec<models::RefreshToken>>,
    #[serde(rename = "validateDbConstraints", skip_serializing_if = "Option::is_none")]
    pub validate_db_constraints: Option<bool>,
}

impl RefreshTokenImportRequest {
    /// Refresh Token Import request.
    pub fn new() -> RefreshTokenImportRequest {
        RefreshTokenImportRequest {
            refresh_tokens: None,
            validate_db_constraints: None,
        }
    }
}

