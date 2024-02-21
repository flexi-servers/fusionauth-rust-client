/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RawLogin : Raw login information for each time a user logs into an application.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawLogin {
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

impl RawLogin {
    /// Raw login information for each time a user logs into an application.
    pub fn new() -> RawLogin {
        RawLogin {
            application_id: None,
            instant: None,
            ip_address: None,
            user_id: None,
        }
    }
}


