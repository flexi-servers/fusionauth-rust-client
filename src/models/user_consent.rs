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

/// UserConsent : Models a User consent.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserConsent {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "consent", skip_serializing_if = "Option::is_none")]
    pub consent: Option<Box<models::Consent>>,
    #[serde(rename = "consentId", skip_serializing_if = "Option::is_none")]
    pub consent_id: Option<uuid::Uuid>,
    #[serde(rename = "giverUserId", skip_serializing_if = "Option::is_none")]
    pub giver_user_id: Option<uuid::Uuid>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ConsentStatus>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl UserConsent {
    /// Models a User consent.
    pub fn new() -> UserConsent {
        UserConsent {
            data: None,
            consent: None,
            consent_id: None,
            giver_user_id: None,
            id: None,
            insert_instant: None,
            last_update_instant: None,
            status: None,
            user_id: None,
            values: None,
        }
    }
}

