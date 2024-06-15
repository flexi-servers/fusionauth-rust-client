/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionData {
    #[serde(rename = "actioneeUserId", skip_serializing_if = "Option::is_none")]
    pub actionee_user_id: Option<uuid::Uuid>,
    #[serde(rename = "actionerUserId", skip_serializing_if = "Option::is_none")]
    pub actioner_user_id: Option<uuid::Uuid>,
    #[serde(rename = "applicationIds", skip_serializing_if = "Option::is_none")]
    pub application_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "emailUser", skip_serializing_if = "Option::is_none")]
    pub email_user: Option<bool>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
    #[serde(rename = "notifyUser", skip_serializing_if = "Option::is_none")]
    pub notify_user: Option<bool>,
    #[serde(rename = "option", skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
    #[serde(rename = "reasonId", skip_serializing_if = "Option::is_none")]
    pub reason_id: Option<uuid::Uuid>,
    #[serde(rename = "userActionId", skip_serializing_if = "Option::is_none")]
    pub user_action_id: Option<uuid::Uuid>,
}

impl ActionData {
    pub fn new() -> ActionData {
        ActionData {
            actionee_user_id: None,
            actioner_user_id: None,
            application_ids: None,
            comment: None,
            email_user: None,
            expiry: None,
            notify_user: None,
            option: None,
            reason_id: None,
            user_action_id: None,
        }
    }
}

