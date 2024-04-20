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

/// JwtRefreshResponse : API response for refreshing a JWT with a Refresh Token.  <p>  Using a different response object from RefreshTokenResponse because the retrieve response will return an object for refreshToken, and this is a  string.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtRefreshResponse {
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "refreshTokenId", skip_serializing_if = "Option::is_none")]
    pub refresh_token_id: Option<uuid::Uuid>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl JwtRefreshResponse {
    /// API response for refreshing a JWT with a Refresh Token.  <p>  Using a different response object from RefreshTokenResponse because the retrieve response will return an object for refreshToken, and this is a  string.
    pub fn new() -> JwtRefreshResponse {
        JwtRefreshResponse {
            refresh_token: None,
            refresh_token_id: None,
            token: None,
        }
    }
}

