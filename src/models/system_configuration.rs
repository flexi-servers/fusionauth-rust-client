/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// SystemConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemConfiguration {
    #[serde(rename = "auditLogConfiguration", skip_serializing_if = "Option::is_none")]
    pub audit_log_configuration: Option<Box<models::AuditLogConfiguration>>,
    #[serde(rename = "corsConfiguration", skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Box<models::CorsConfiguration>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "eventLogConfiguration", skip_serializing_if = "Option::is_none")]
    pub event_log_configuration: Option<Box<models::EventLogConfiguration>>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "loginRecordConfiguration", skip_serializing_if = "Option::is_none")]
    pub login_record_configuration: Option<Box<models::LoginRecordConfiguration>>,
    /// Timezone Identifier
    #[serde(rename = "reportTimezone", skip_serializing_if = "Option::is_none")]
    pub report_timezone: Option<String>,
    #[serde(rename = "trustedProxyConfiguration", skip_serializing_if = "Option::is_none")]
    pub trusted_proxy_configuration: Option<Box<models::SystemTrustedProxyConfiguration>>,
    #[serde(rename = "uiConfiguration", skip_serializing_if = "Option::is_none")]
    pub ui_configuration: Option<Box<models::UiConfiguration>>,
}

impl SystemConfiguration {
    /// 
    pub fn new() -> SystemConfiguration {
        SystemConfiguration {
            audit_log_configuration: None,
            cors_configuration: None,
            data: None,
            event_log_configuration: None,
            insert_instant: None,
            last_update_instant: None,
            login_record_configuration: None,
            report_timezone: None,
            trusted_proxy_configuration: None,
            ui_configuration: None,
        }
    }
}

