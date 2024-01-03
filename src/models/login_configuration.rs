/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginConfiguration {
    #[serde(rename = "allowTokenRefresh", skip_serializing_if = "Option::is_none")]
    pub allow_token_refresh: Option<bool>,
    #[serde(rename = "generateRefreshTokens", skip_serializing_if = "Option::is_none")]
    pub generate_refresh_tokens: Option<bool>,
    #[serde(rename = "requireAuthentication", skip_serializing_if = "Option::is_none")]
    pub require_authentication: Option<bool>,
}

impl LoginConfiguration {
    pub fn new() -> LoginConfiguration {
        LoginConfiguration {
            allow_token_refresh: None,
            generate_refresh_tokens: None,
            require_authentication: None,
        }
    }
}

