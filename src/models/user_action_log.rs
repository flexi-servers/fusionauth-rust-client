/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserActionLog : A log for an action that was taken on a User.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserActionLog {
    #[serde(rename = "actioneeUserId", skip_serializing_if = "Option::is_none")]
    pub actionee_user_id: Option<uuid::Uuid>,
    #[serde(rename = "actionerUserId", skip_serializing_if = "Option::is_none")]
    pub actioner_user_id: Option<uuid::Uuid>,
    #[serde(rename = "applicationIds", skip_serializing_if = "Option::is_none")]
    pub application_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "emailUserOnEnd", skip_serializing_if = "Option::is_none")]
    pub email_user_on_end: Option<bool>,
    #[serde(rename = "endEventSent", skip_serializing_if = "Option::is_none")]
    pub end_event_sent: Option<bool>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Box<crate::models::LogHistory>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    #[serde(rename = "localizedName", skip_serializing_if = "Option::is_none")]
    pub localized_name: Option<String>,
    #[serde(rename = "localizedOption", skip_serializing_if = "Option::is_none")]
    pub localized_option: Option<String>,
    #[serde(rename = "localizedReason", skip_serializing_if = "Option::is_none")]
    pub localized_reason: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "notifyUserOnEnd", skip_serializing_if = "Option::is_none")]
    pub notify_user_on_end: Option<bool>,
    #[serde(rename = "option", skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "reasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(rename = "userActionId", skip_serializing_if = "Option::is_none")]
    pub user_action_id: Option<uuid::Uuid>,
}

impl UserActionLog {
    /// A log for an action that was taken on a User.
    pub fn new() -> UserActionLog {
        UserActionLog {
            actionee_user_id: None,
            actioner_user_id: None,
            application_ids: None,
            comment: None,
            email_user_on_end: None,
            end_event_sent: None,
            expiry: None,
            history: None,
            id: None,
            insert_instant: None,
            localized_name: None,
            localized_option: None,
            localized_reason: None,
            name: None,
            notify_user_on_end: None,
            option: None,
            reason: None,
            reason_code: None,
            user_action_id: None,
        }
    }
}


