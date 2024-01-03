/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserActionEvent : Models the user action Event.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserActionEvent {
    #[serde(rename = "applicationIds", skip_serializing_if = "Option::is_none")]
    pub application_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "actionId", skip_serializing_if = "Option::is_none")]
    pub action_id: Option<uuid::Uuid>,
    #[serde(rename = "actioneeUserId", skip_serializing_if = "Option::is_none")]
    pub actionee_user_id: Option<uuid::Uuid>,
    #[serde(rename = "actionerUserId", skip_serializing_if = "Option::is_none")]
    pub actioner_user_id: Option<uuid::Uuid>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<crate::models::Email>>,
    #[serde(rename = "emailedUser", skip_serializing_if = "Option::is_none")]
    pub emailed_user: Option<bool>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
    #[serde(rename = "localizedAction", skip_serializing_if = "Option::is_none")]
    pub localized_action: Option<String>,
    #[serde(rename = "localizedDuration", skip_serializing_if = "Option::is_none")]
    pub localized_duration: Option<String>,
    #[serde(rename = "localizedOption", skip_serializing_if = "Option::is_none")]
    pub localized_option: Option<String>,
    #[serde(rename = "localizedReason", skip_serializing_if = "Option::is_none")]
    pub localized_reason: Option<String>,
    #[serde(rename = "notifyUser", skip_serializing_if = "Option::is_none")]
    pub notify_user: Option<bool>,
    #[serde(rename = "option", skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<crate::models::UserActionPhase>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "reasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "createInstant", skip_serializing_if = "Option::is_none")]
    pub create_instant: Option<i64>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Box<crate::models::EventInfo>>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<uuid::Uuid>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::EventType>,
}

impl UserActionEvent {
    /// Models the user action Event.
    pub fn new() -> UserActionEvent {
        UserActionEvent {
            application_ids: None,
            action: None,
            action_id: None,
            actionee_user_id: None,
            actioner_user_id: None,
            comment: None,
            email: None,
            emailed_user: None,
            expiry: None,
            localized_action: None,
            localized_duration: None,
            localized_option: None,
            localized_reason: None,
            notify_user: None,
            option: None,
            phase: None,
            reason: None,
            reason_code: None,
            create_instant: None,
            id: None,
            info: None,
            tenant_id: None,
            r#type: None,
        }
    }
}


