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

/// XboxIdentityProvider : Xbox gaming login provider.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct XboxIdentityProvider {
    #[serde(rename = "buttonText", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "applicationConfiguration", skip_serializing_if = "Option::is_none")]
    pub application_configuration: Option<std::collections::HashMap<String, models::XboxApplicationConfiguration>>,
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    #[serde(rename = "lambdaConfiguration", skip_serializing_if = "Option::is_none")]
    pub lambda_configuration: Option<Box<models::ProviderLambdaConfiguration>>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "linkingStrategy", skip_serializing_if = "Option::is_none")]
    pub linking_strategy: Option<models::IdentityProviderLinkingStrategy>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "tenantConfiguration", skip_serializing_if = "Option::is_none")]
    pub tenant_configuration: Option<std::collections::HashMap<String, models::IdentityProviderTenantConfiguration>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::IdentityProviderType>,
}

impl XboxIdentityProvider {
    /// Xbox gaming login provider.
    pub fn new() -> XboxIdentityProvider {
        XboxIdentityProvider {
            button_text: None,
            client_id: None,
            client_secret: None,
            scope: None,
            data: None,
            application_configuration: None,
            debug: None,
            id: None,
            insert_instant: None,
            lambda_configuration: None,
            last_update_instant: None,
            linking_strategy: None,
            name: None,
            tenant_configuration: None,
            r#type: None,
        }
    }
}

