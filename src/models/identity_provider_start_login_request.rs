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

/// IdentityProviderStartLoginRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderStartLoginRequest {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "identityProviderId", skip_serializing_if = "Option::is_none")]
    pub identity_provider_id: Option<uuid::Uuid>,
    #[serde(rename = "loginId", skip_serializing_if = "Option::is_none")]
    pub login_id: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<Box<models::MetaData>>,
    #[serde(rename = "newDevice", skip_serializing_if = "Option::is_none")]
    pub new_device: Option<bool>,
    #[serde(rename = "noJWT", skip_serializing_if = "Option::is_none")]
    pub no_jwt: Option<bool>,
}

impl IdentityProviderStartLoginRequest {
    /// 
    pub fn new() -> IdentityProviderStartLoginRequest {
        IdentityProviderStartLoginRequest {
            data: None,
            identity_provider_id: None,
            login_id: None,
            state: None,
            application_id: None,
            ip_address: None,
            meta_data: None,
            new_device: None,
            no_jwt: None,
        }
    }
}

