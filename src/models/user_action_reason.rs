/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserActionReason : Models action reasons.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserActionReason {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    /// Models a set of localized Strings that can be stored as JSON.
    #[serde(rename = "localizedTexts", skip_serializing_if = "Option::is_none")]
    pub localized_texts: Option<serde_json::Value>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl UserActionReason {
    /// Models action reasons.
    pub fn new() -> UserActionReason {
        UserActionReason {
            code: None,
            id: None,
            insert_instant: None,
            last_update_instant: None,
            localized_texts: None,
            text: None,
        }
    }
}


