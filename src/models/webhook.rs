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

/// Webhook : A server where events are sent. This includes user action events and any other events sent by FusionAuth.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    #[serde(rename = "connectTimeout", skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<i32>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "eventsEnabled", skip_serializing_if = "Option::is_none")]
    pub events_enabled: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "global", skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
    /// Type for webhook headers.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(rename = "httpAuthenticationPassword", skip_serializing_if = "Option::is_none")]
    pub http_authentication_password: Option<String>,
    #[serde(rename = "httpAuthenticationUsername", skip_serializing_if = "Option::is_none")]
    pub http_authentication_username: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "readTimeout", skip_serializing_if = "Option::is_none")]
    pub read_timeout: Option<i32>,
    #[serde(rename = "signatureConfiguration", skip_serializing_if = "Option::is_none")]
    pub signature_configuration: Option<Box<models::WebhookSignatureConfiguration>>,
    #[serde(rename = "sslCertificate", skip_serializing_if = "Option::is_none")]
    pub ssl_certificate: Option<String>,
    #[serde(rename = "sslCertificateKeyId", skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_key_id: Option<uuid::Uuid>,
    #[serde(rename = "tenantIds", skip_serializing_if = "Option::is_none")]
    pub tenant_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Webhook {
    /// A server where events are sent. This includes user action events and any other events sent by FusionAuth.
    pub fn new() -> Webhook {
        Webhook {
            connect_timeout: None,
            data: None,
            description: None,
            events_enabled: None,
            global: None,
            headers: None,
            http_authentication_password: None,
            http_authentication_username: None,
            id: None,
            insert_instant: None,
            last_update_instant: None,
            read_timeout: None,
            signature_configuration: None,
            ssl_certificate: None,
            ssl_certificate_key_id: None,
            tenant_ids: None,
            url: None,
        }
    }
}

