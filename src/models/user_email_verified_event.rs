/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// UserEmailVerifiedEvent : Models the User Email Verify Event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserEmailVerifiedEvent {
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

impl UserEmailVerifiedEvent {
    /// Models the User Email Verify Event.
    pub fn new() -> UserEmailVerifiedEvent {
        UserEmailVerifiedEvent {
            user: None,
            create_instant: None,
            id: None,
            info: None,
            tenant_id: None,
            r#type: None,
        }
    }
}

