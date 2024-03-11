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

/// OAuth2Configuration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth2Configuration {
    #[serde(rename = "authorizedOriginURLs", skip_serializing_if = "Option::is_none")]
    pub authorized_origin_urls: Option<Vec<String>>,
    #[serde(rename = "authorizedRedirectURLs", skip_serializing_if = "Option::is_none")]
    pub authorized_redirect_urls: Option<Vec<String>>,
    #[serde(rename = "authorizedURLValidationPolicy", skip_serializing_if = "Option::is_none")]
    pub authorized_url_validation_policy: Option<models::Oauth2AuthorizedUrlValidationPolicy>,
    #[serde(rename = "clientAuthenticationPolicy", skip_serializing_if = "Option::is_none")]
    pub client_authentication_policy: Option<models::ClientAuthenticationPolicy>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "clientSecret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(rename = "deviceVerificationURL", skip_serializing_if = "Option::is_none")]
    pub device_verification_url: Option<String>,
    #[serde(rename = "enabledGrants", skip_serializing_if = "Option::is_none")]
    pub enabled_grants: Option<Vec<serde_json::Value>>,
    #[serde(rename = "generateRefreshTokens", skip_serializing_if = "Option::is_none")]
    pub generate_refresh_tokens: Option<bool>,
    #[serde(rename = "logoutBehavior", skip_serializing_if = "Option::is_none")]
    pub logout_behavior: Option<models::LogoutBehavior>,
    #[serde(rename = "logoutURL", skip_serializing_if = "Option::is_none")]
    pub logout_url: Option<String>,
    #[serde(rename = "proofKeyForCodeExchangePolicy", skip_serializing_if = "Option::is_none")]
    pub proof_key_for_code_exchange_policy: Option<models::ProofKeyForCodeExchangePolicy>,
    #[serde(rename = "requireClientAuthentication", skip_serializing_if = "Option::is_none")]
    pub require_client_authentication: Option<bool>,
    #[serde(rename = "requireRegistration", skip_serializing_if = "Option::is_none")]
    pub require_registration: Option<bool>,
}

impl OAuth2Configuration {
    /// 
    pub fn new() -> OAuth2Configuration {
        OAuth2Configuration {
            authorized_origin_urls: None,
            authorized_redirect_urls: None,
            authorized_url_validation_policy: None,
            client_authentication_policy: None,
            client_id: None,
            client_secret: None,
            debug: None,
            device_verification_url: None,
            enabled_grants: None,
            generate_refresh_tokens: None,
            logout_behavior: None,
            logout_url: None,
            proof_key_for_code_exchange_policy: None,
            require_client_authentication: None,
            require_registration: None,
        }
    }
}

