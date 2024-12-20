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

/// FormField : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormField {
    #[serde(rename = "confirm", skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    #[serde(rename = "consentId", skip_serializing_if = "Option::is_none")]
    pub consent_id: Option<uuid::Uuid>,
    #[serde(rename = "control", skip_serializing_if = "Option::is_none")]
    pub control: Option<models::FormControl>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::FormDataType>,
    #[serde(rename = "validator", skip_serializing_if = "Option::is_none")]
    pub validator: Option<Box<models::FormFieldValidator>>,
}

impl FormField {
    /// 
    pub fn new() -> FormField {
        FormField {
            confirm: None,
            consent_id: None,
            control: None,
            data: None,
            description: None,
            id: None,
            insert_instant: None,
            key: None,
            last_update_instant: None,
            name: None,
            options: None,
            required: None,
            r#type: None,
            validator: None,
        }
    }
}

