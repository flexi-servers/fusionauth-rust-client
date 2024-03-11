/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// User : The global view of a User. This object contains all global information about the user including birthdate, registration information  preferred languages, global attributes, etc.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "preferredLanguages", skip_serializing_if = "Option::is_none")]
    pub preferred_languages: Option<Vec<String>>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A date without a time-zone in the ISO-8601 calendar system, such as 2007-12-03.
    #[serde(rename = "birthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(rename = "cleanSpeakId", skip_serializing_if = "Option::is_none")]
    pub clean_speak_id: Option<uuid::Uuid>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "mobilePhone", skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    #[serde(rename = "parentEmail", skip_serializing_if = "Option::is_none")]
    pub parent_email: Option<String>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<uuid::Uuid>,
    /// Timezone Identifier
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "twoFactor", skip_serializing_if = "Option::is_none")]
    pub two_factor: Option<Box<models::UserTwoFactorConfiguration>>,
    #[serde(rename = "memberships", skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<models::GroupMember>>,
    #[serde(rename = "registrations", skip_serializing_if = "Option::is_none")]
    pub registrations: Option<Vec<models::UserRegistration>>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "breachedPasswordLastCheckedInstant", skip_serializing_if = "Option::is_none")]
    pub breached_password_last_checked_instant: Option<i64>,
    #[serde(rename = "breachedPasswordStatus", skip_serializing_if = "Option::is_none")]
    pub breached_password_status: Option<models::BreachedPasswordStatus>,
    #[serde(rename = "connectorId", skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<uuid::Uuid>,
    #[serde(rename = "encryptionScheme", skip_serializing_if = "Option::is_none")]
    pub encryption_scheme: Option<String>,
    #[serde(rename = "factor", skip_serializing_if = "Option::is_none")]
    pub factor: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastLoginInstant", skip_serializing_if = "Option::is_none")]
    pub last_login_instant: Option<i64>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "passwordChangeReason", skip_serializing_if = "Option::is_none")]
    pub password_change_reason: Option<models::ChangePasswordReason>,
    #[serde(rename = "passwordChangeRequired", skip_serializing_if = "Option::is_none")]
    pub password_change_required: Option<bool>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "passwordLastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub password_last_update_instant: Option<i64>,
    #[serde(rename = "salt", skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
    #[serde(rename = "uniqueUsername", skip_serializing_if = "Option::is_none")]
    pub unique_username: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "usernameStatus", skip_serializing_if = "Option::is_none")]
    pub username_status: Option<models::ContentStatus>,
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "verifiedInstant", skip_serializing_if = "Option::is_none")]
    pub verified_instant: Option<i64>,
}

impl User {
    /// The global view of a User. This object contains all global information about the user including birthdate, registration information  preferred languages, global attributes, etc.
    pub fn new() -> User {
        User {
            preferred_languages: None,
            active: None,
            birth_date: None,
            clean_speak_id: None,
            data: None,
            email: None,
            expiry: None,
            first_name: None,
            full_name: None,
            image_url: None,
            insert_instant: None,
            last_name: None,
            last_update_instant: None,
            middle_name: None,
            mobile_phone: None,
            parent_email: None,
            tenant_id: None,
            timezone: None,
            two_factor: None,
            memberships: None,
            registrations: None,
            breached_password_last_checked_instant: None,
            breached_password_status: None,
            connector_id: None,
            encryption_scheme: None,
            factor: None,
            id: None,
            last_login_instant: None,
            password: None,
            password_change_reason: None,
            password_change_required: None,
            password_last_update_instant: None,
            salt: None,
            unique_username: None,
            username: None,
            username_status: None,
            verified: None,
            verified_instant: None,
        }
    }
}

