/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SecureIdentity : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecureIdentity {
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "breachedPasswordLastCheckedInstant", skip_serializing_if = "Option::is_none")]
    pub breached_password_last_checked_instant: Option<i64>,
    #[serde(rename = "breachedPasswordStatus", skip_serializing_if = "Option::is_none")]
    pub breached_password_status: Option<models::BreachedPasswordStatus>,
    #[serde(rename = "connectorId", skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<uuid::Uuid>,
    #[serde(rename = "encryptionScheme", skip_serializing_if = "Option::is_none")]
    pub encryption_scheme: Option<String>,
    #[serde(rename = "factor", skip_serializing_if = "Option::is_none")]
    pub factor: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastLoginInstant", skip_serializing_if = "Option::is_none")]
    pub last_login_instant: Option<i64>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "passwordChangeReason", skip_serializing_if = "Option::is_none")]
    pub password_change_reason: Option<models::ChangePasswordReason>,
    #[serde(rename = "passwordChangeRequired", skip_serializing_if = "Option::is_none")]
    pub password_change_required: Option<bool>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "passwordLastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub password_last_update_instant: Option<i64>,
    #[serde(rename = "salt", skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
    #[serde(rename = "uniqueUsername", skip_serializing_if = "Option::is_none")]
    pub unique_username: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "usernameStatus", skip_serializing_if = "Option::is_none")]
    pub username_status: Option<models::ContentStatus>,
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "verifiedInstant", skip_serializing_if = "Option::is_none")]
    pub verified_instant: Option<i64>,
}

impl SecureIdentity {
    /// 
    pub fn new() -> SecureIdentity {
        SecureIdentity {
            breached_password_last_checked_instant: None,
            breached_password_status: None,
            connector_id: None,
            encryption_scheme: None,
            factor: None,
            id: None,
            last_login_instant: None,
            password: None,
            password_change_reason: None,
            password_change_required: None,
            password_last_update_instant: None,
            salt: None,
            unique_username: None,
            username: None,
            username_status: None,
            verified: None,
            verified_instant: None,
        }
    }
}

