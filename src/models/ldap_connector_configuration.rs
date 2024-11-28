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

/// LdapConnectorConfiguration : Models an LDAP connector.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LdapConnectorConfiguration {
    #[serde(rename = "authenticationURL", skip_serializing_if = "Option::is_none")]
    pub authentication_url: Option<String>,
    #[serde(rename = "baseStructure", skip_serializing_if = "Option::is_none")]
    pub base_structure: Option<String>,
    #[serde(rename = "connectTimeout", skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<i32>,
    #[serde(rename = "identifyingAttribute", skip_serializing_if = "Option::is_none")]
    pub identifying_attribute: Option<String>,
    #[serde(rename = "lambdaConfiguration", skip_serializing_if = "Option::is_none")]
    pub lambda_configuration: Option<Box<models::ConnectorLambdaConfiguration>>,
    #[serde(rename = "loginIdAttribute", skip_serializing_if = "Option::is_none")]
    pub login_id_attribute: Option<String>,
    #[serde(rename = "readTimeout", skip_serializing_if = "Option::is_none")]
    pub read_timeout: Option<i32>,
    #[serde(rename = "requestedAttributes", skip_serializing_if = "Option::is_none")]
    pub requested_attributes: Option<Vec<String>>,
    #[serde(rename = "securityMethod", skip_serializing_if = "Option::is_none")]
    pub security_method: Option<models::LdapSecurityMethod>,
    #[serde(rename = "systemAccountDN", skip_serializing_if = "Option::is_none")]
    pub system_account_dn: Option<String>,
    #[serde(rename = "systemAccountPassword", skip_serializing_if = "Option::is_none")]
    pub system_account_password: Option<String>,
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

impl LdapConnectorConfiguration {
    /// Models an LDAP connector.
    pub fn new() -> LdapConnectorConfiguration {
        LdapConnectorConfiguration {
            authentication_url: None,
            base_structure: None,
            connect_timeout: None,
            identifying_attribute: None,
            lambda_configuration: None,
            login_id_attribute: None,
            read_timeout: None,
            requested_attributes: None,
            security_method: None,
            system_account_dn: None,
            system_account_password: None,
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

