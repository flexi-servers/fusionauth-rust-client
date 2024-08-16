/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserAction : An action that can be executed on a user (discipline or reward potentially).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAction {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "cancelEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub cancel_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "endEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub end_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "includeEmailInEventJSON", skip_serializing_if = "Option::is_none")]
    pub include_email_in_event_json: Option<bool>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    /// Models a set of localized Strings that can be stored as JSON.
    #[serde(rename = "localizedNames", skip_serializing_if = "Option::is_none")]
    pub localized_names: Option<serde_json::Value>,
    #[serde(rename = "modifyEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub modify_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<models::UserActionOption>>,
    #[serde(rename = "preventLogin", skip_serializing_if = "Option::is_none")]
    pub prevent_login: Option<bool>,
    #[serde(rename = "sendEndEvent", skip_serializing_if = "Option::is_none")]
    pub send_end_event: Option<bool>,
    #[serde(rename = "startEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub start_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "temporal", skip_serializing_if = "Option::is_none")]
    pub temporal: Option<bool>,
    #[serde(rename = "transactionType", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<models::TransactionType>,
    #[serde(rename = "userEmailingEnabled", skip_serializing_if = "Option::is_none")]
    pub user_emailing_enabled: Option<bool>,
    #[serde(rename = "userNotificationsEnabled", skip_serializing_if = "Option::is_none")]
    pub user_notifications_enabled: Option<bool>,
}

impl UserAction {
    /// An action that can be executed on a user (discipline or reward potentially).
    pub fn new() -> UserAction {
        UserAction {
            active: None,
            cancel_email_template_id: None,
            end_email_template_id: None,
            id: None,
            include_email_in_event_json: None,
            insert_instant: None,
            last_update_instant: None,
            localized_names: None,
            modify_email_template_id: None,
            name: None,
            options: None,
            prevent_login: None,
            send_end_event: None,
            start_email_template_id: None,
            temporal: None,
            transaction_type: None,
            user_emailing_enabled: None,
            user_notifications_enabled: None,
        }
    }
}

