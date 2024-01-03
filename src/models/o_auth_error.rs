/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OAuthError : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuthError {
    #[serde(rename = "change_password_id", skip_serializing_if = "Option::is_none")]
    pub change_password_id: Option<String>,
    #[serde(rename = "error_description", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::models::OAuthErrorType>,
    #[serde(rename = "error_uri", skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<String>,
    #[serde(rename = "two_factor_methods", skip_serializing_if = "Option::is_none")]
    pub two_factor_methods: Option<Vec<crate::models::TwoFactorMethod>>,
    #[serde(rename = "error_reason", skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<crate::models::OAuthErrorReason>,
    #[serde(rename = "two_factor_id", skip_serializing_if = "Option::is_none")]
    pub two_factor_id: Option<String>,
}

impl OAuthError {
    /// 
    pub fn new() -> OAuthError {
        OAuthError {
            change_password_id: None,
            error_description: None,
            error: None,
            error_uri: None,
            two_factor_methods: None,
            error_reason: None,
            two_factor_id: None,
        }
    }
}


