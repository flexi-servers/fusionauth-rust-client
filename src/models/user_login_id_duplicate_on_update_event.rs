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

/// UserLoginIdDuplicateOnUpdateEvent : Models an event where a user is being updated and tries to use an \"in-use\" login Id (email or username).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLoginIdDuplicateOnUpdateEvent {
    #[serde(rename = "duplicateEmail", skip_serializing_if = "Option::is_none")]
    pub duplicate_email: Option<String>,
    #[serde(rename = "duplicateUsername", skip_serializing_if = "Option::is_none")]
    pub duplicate_username: Option<String>,
    #[serde(rename = "existing", skip_serializing_if = "Option::is_none")]
    pub existing: Option<Box<models::User>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::User>>,
}

impl UserLoginIdDuplicateOnUpdateEvent {
    /// Models an event where a user is being updated and tries to use an \"in-use\" login Id (email or username).
    pub fn new() -> UserLoginIdDuplicateOnUpdateEvent {
        UserLoginIdDuplicateOnUpdateEvent {
            duplicate_email: None,
            duplicate_username: None,
            existing: None,
            user: None,
        }
    }
}

