/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LoginPreventedResponse : The summary of the action that is preventing login to be returned on the login response.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginPreventedResponse {
    #[serde(rename = "actionId", skip_serializing_if = "Option::is_none")]
    pub action_id: Option<uuid::Uuid>,
    #[serde(rename = "actionerUserId", skip_serializing_if = "Option::is_none")]
    pub actioner_user_id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
    #[serde(rename = "localizedName", skip_serializing_if = "Option::is_none")]
    pub localized_name: Option<String>,
    #[serde(rename = "localizedOption", skip_serializing_if = "Option::is_none")]
    pub localized_option: Option<String>,
    #[serde(rename = "localizedReason", skip_serializing_if = "Option::is_none")]
    pub localized_reason: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "option", skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "reasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}

impl LoginPreventedResponse {
    /// The summary of the action that is preventing login to be returned on the login response.
    pub fn new() -> LoginPreventedResponse {
        LoginPreventedResponse {
            action_id: None,
            actioner_user_id: None,
            expiry: None,
            localized_name: None,
            localized_option: None,
            localized_reason: None,
            name: None,
            option: None,
            reason: None,
            reason_code: None,
        }
    }
}


