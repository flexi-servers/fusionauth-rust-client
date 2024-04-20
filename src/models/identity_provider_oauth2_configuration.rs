/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// IdentityProviderOauth2Configuration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderOauth2Configuration {
    #[serde(rename = "authorization_endpoint", skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    #[serde(rename = "clientAuthenticationMethod", skip_serializing_if = "Option::is_none")]
    pub client_authentication_method: Option<models::ClientAuthenticationMethod>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "emailClaim", skip_serializing_if = "Option::is_none")]
    pub email_claim: Option<String>,
    #[serde(rename = "emailVerifiedClaim", skip_serializing_if = "Option::is_none")]
    pub email_verified_claim: Option<String>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "token_endpoint", skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
    #[serde(rename = "uniqueIdClaim", skip_serializing_if = "Option::is_none")]
    pub unique_id_claim: Option<String>,
    #[serde(rename = "userinfo_endpoint", skip_serializing_if = "Option::is_none")]
    pub userinfo_endpoint: Option<String>,
    #[serde(rename = "usernameClaim", skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
}

impl IdentityProviderOauth2Configuration {
    /// 
    pub fn new() -> IdentityProviderOauth2Configuration {
        IdentityProviderOauth2Configuration {
            authorization_endpoint: None,
            client_authentication_method: None,
            client_id: None,
            client_secret: None,
            email_claim: None,
            email_verified_claim: None,
            issuer: None,
            scope: None,
            token_endpoint: None,
            unique_id_claim: None,
            userinfo_endpoint: None,
            username_claim: None,
        }
    }
}

