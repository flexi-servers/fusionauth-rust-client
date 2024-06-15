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

/// GenericConnectorConfiguration : Models a generic connector.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericConnectorConfiguration {
    #[serde(rename = "authenticationURL", skip_serializing_if = "Option::is_none")]
    pub authentication_url: Option<String>,
    #[serde(rename = "connectTimeout", skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<i32>,
    /// Type for webhook headers.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(rename = "httpAuthenticationPassword", skip_serializing_if = "Option::is_none")]
    pub http_authentication_password: Option<String>,
    #[serde(rename = "httpAuthenticationUsername", skip_serializing_if = "Option::is_none")]
    pub http_authentication_username: Option<String>,
    #[serde(rename = "readTimeout", skip_serializing_if = "Option::is_none")]
    pub read_timeout: Option<i32>,
    #[serde(rename = "sslCertificateKeyId", skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_key_id: Option<uuid::Uuid>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::ConnectorType>,
}

impl GenericConnectorConfiguration {
    /// Models a generic connector.
    pub fn new() -> GenericConnectorConfiguration {
        GenericConnectorConfiguration {
            authentication_url: None,
            connect_timeout: None,
            headers: None,
            http_authentication_password: None,
            http_authentication_username: None,
            read_timeout: None,
            ssl_certificate_key_id: None,
            data: None,
            debug: None,
            id: None,
            insert_instant: None,
            last_update_instant: None,
            name: None,
            r#type: None,
        }
    }
}

