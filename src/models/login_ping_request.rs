/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LoginPingRequest : Login Ping API request object.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginPingRequest {
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<Box<crate::models::MetaData>>,
    #[serde(rename = "newDevice", skip_serializing_if = "Option::is_none")]
    pub new_device: Option<bool>,
    #[serde(rename = "noJWT", skip_serializing_if = "Option::is_none")]
    pub no_jwt: Option<bool>,
}

impl LoginPingRequest {
    /// Login Ping API request object.
    pub fn new() -> LoginPingRequest {
        LoginPingRequest {
            user_id: None,
            application_id: None,
            ip_address: None,
            meta_data: None,
            new_device: None,
            no_jwt: None,
        }
    }
}

