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

/// JwtRefreshTokenRevokeEvent : Models the Refresh Token Revoke Event. This event might be for a single token, a user  or an entire application.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtRefreshTokenRevokeEvent {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "applicationTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub application_time_to_live_in_seconds: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<Box<models::RefreshToken>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::User>>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "createInstant", skip_serializing_if = "Option::is_none")]
    pub create_instant: Option<i64>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Box<models::EventInfo>>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<uuid::Uuid>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::EventType>,
}

impl JwtRefreshTokenRevokeEvent {
    /// Models the Refresh Token Revoke Event. This event might be for a single token, a user  or an entire application.
    pub fn new() -> JwtRefreshTokenRevokeEvent {
        JwtRefreshTokenRevokeEvent {
            application_id: None,
            application_time_to_live_in_seconds: None,
            refresh_token: None,
            user: None,
            user_id: None,
            create_instant: None,
            id: None,
            info: None,
            tenant_id: None,
            r#type: None,
        }
    }
}

