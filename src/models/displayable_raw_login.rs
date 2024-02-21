/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DisplayableRawLogin : A displayable raw login that includes application name and user loginId.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayableRawLogin {
    #[serde(rename = "applicationName", skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<crate::models::Location>>,
    #[serde(rename = "loginId", skip_serializing_if = "Option::is_none")]
    pub login_id: Option<String>,
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "instant", skip_serializing_if = "Option::is_none")]
    pub instant: Option<i64>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl DisplayableRawLogin {
    /// A displayable raw login that includes application name and user loginId.
    pub fn new() -> DisplayableRawLogin {
        DisplayableRawLogin {
            application_name: None,
            location: None,
            login_id: None,
            application_id: None,
            instant: None,
            ip_address: None,
            user_id: None,
        }
    }
}


