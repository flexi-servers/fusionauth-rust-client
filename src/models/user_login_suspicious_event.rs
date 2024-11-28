/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.54.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserLoginSuspiciousEvent : Models the User Login event that is suspicious.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLoginSuspiciousEvent {
    #[serde(rename = "threatsDetected", skip_serializing_if = "Option::is_none")]
    pub threats_detected: Option<Vec<serde_json::Value>>,
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
}

impl UserLoginSuspiciousEvent {
    /// Models the User Login event that is suspicious.
    pub fn new() -> UserLoginSuspiciousEvent {
        UserLoginSuspiciousEvent {
            threats_detected: None,
            application_id: None,
            authentication_type: None,
            connector_id: None,
            identity_provider_id: None,
            identity_provider_name: None,
            ip_address: None,
        }
    }
}

