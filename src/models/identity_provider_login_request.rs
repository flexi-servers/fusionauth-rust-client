/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// IdentityProviderLoginRequest : Login API request object used for login to third-party systems (i.e. Login with Facebook).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderLoginRequest {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "identityProviderId", skip_serializing_if = "Option::is_none")]
    pub identity_provider_id: Option<uuid::Uuid>,
    #[serde(rename = "noLink", skip_serializing_if = "Option::is_none")]
    pub no_link: Option<bool>,
    #[serde(rename = "encodedJWT", skip_serializing_if = "Option::is_none")]
    pub encoded_jwt: Option<String>,
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

impl IdentityProviderLoginRequest {
    /// Login API request object used for login to third-party systems (i.e. Login with Facebook).
    pub fn new() -> IdentityProviderLoginRequest {
        IdentityProviderLoginRequest {
            data: None,
            identity_provider_id: None,
            no_link: None,
            encoded_jwt: None,
            application_id: None,
            ip_address: None,
            meta_data: None,
            new_device: None,
            no_jwt: None,
        }
    }
}

