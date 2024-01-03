/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TwoFactorStartRequest : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorStartRequest {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "loginId", skip_serializing_if = "Option::is_none")]
    pub login_id: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "trustChallenge", skip_serializing_if = "Option::is_none")]
    pub trust_challenge: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl TwoFactorStartRequest {
    /// 
    pub fn new() -> TwoFactorStartRequest {
        TwoFactorStartRequest {
            application_id: None,
            code: None,
            login_id: None,
            state: None,
            trust_challenge: None,
            user_id: None,
        }
    }
}


