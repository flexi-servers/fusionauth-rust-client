/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserLoginSuccessEvent : Models the User Login Success Event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLoginSuccessEvent {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "authenticationType", skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "connectorId", skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<uuid::Uuid>,
    #[serde(rename = "identityProviderId", skip_serializing_if = "Option::is_none")]
    pub identity_provider_id: Option<uuid::Uuid>,
    #[serde(rename = "identityProviderName", skip_serializing_if = "Option::is_none")]
    pub identity_provider_name: Option<String>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::User>>,
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

impl UserLoginSuccessEvent {
    /// Models the User Login Success Event.
    pub fn new() -> UserLoginSuccessEvent {
        UserLoginSuccessEvent {
            application_id: None,
            authentication_type: None,
            connector_id: None,
            identity_provider_id: None,
            identity_provider_name: None,
            ip_address: None,
            user: None,
            create_instant: None,
            id: None,
            info: None,
            tenant_id: None,
            r#type: None,
        }
    }
}

