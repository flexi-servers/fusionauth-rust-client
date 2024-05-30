/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// OpenIdConfiguration : OpenID Connect Configuration as described by the <a href=\"https:openid.netspecsopenid-connect-discovery-1_0.html#ProviderMetadata\">OpenID  Provider Metadata<a>.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenIdConfiguration {
    #[serde(rename = "authorization_endpoint", skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    #[serde(rename = "backchannel_logout_supported", skip_serializing_if = "Option::is_none")]
    pub backchannel_logout_supported: Option<bool>,
    #[serde(rename = "claims_supported", skip_serializing_if = "Option::is_none")]
    pub claims_supported: Option<Vec<String>>,
    #[serde(rename = "device_authorization_endpoint", skip_serializing_if = "Option::is_none")]
    pub device_authorization_endpoint: Option<String>,
    #[serde(rename = "end_session_endpoint", skip_serializing_if = "Option::is_none")]
    pub end_session_endpoint: Option<String>,
    #[serde(rename = "frontchannel_logout_supported", skip_serializing_if = "Option::is_none")]
    pub frontchannel_logout_supported: Option<bool>,
    #[serde(rename = "grant_types_supported", skip_serializing_if = "Option::is_none")]
    pub grant_types_supported: Option<Vec<String>>,
    #[serde(rename = "id_token_signing_alg_values_supported", skip_serializing_if = "Option::is_none")]
    pub id_token_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "jwks_uri", skip_serializing_if = "Option::is_none")]
    pub jwks_uri: Option<String>,
    #[serde(rename = "response_modes_supported", skip_serializing_if = "Option::is_none")]
    pub response_modes_supported: Option<Vec<String>>,
    #[serde(rename = "response_types_supported", skip_serializing_if = "Option::is_none")]
    pub response_types_supported: Option<Vec<String>>,
    #[serde(rename = "scopes_supported", skip_serializing_if = "Option::is_none")]
    pub scopes_supported: Option<Vec<String>>,
    #[serde(rename = "subject_types_supported", skip_serializing_if = "Option::is_none")]
    pub subject_types_supported: Option<Vec<String>>,
    #[serde(rename = "token_endpoint", skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
    #[serde(rename = "token_endpoint_auth_methods_supported", skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_methods_supported: Option<Vec<String>>,
    #[serde(rename = "userinfo_endpoint", skip_serializing_if = "Option::is_none")]
    pub userinfo_endpoint: Option<String>,
    #[serde(rename = "userinfo_signing_alg_values_supported", skip_serializing_if = "Option::is_none")]
    pub userinfo_signing_alg_values_supported: Option<Vec<String>>,
}

impl OpenIdConfiguration {
    /// OpenID Connect Configuration as described by the <a href=\"https:openid.netspecsopenid-connect-discovery-1_0.html#ProviderMetadata\">OpenID  Provider Metadata<a>.
    pub fn new() -> OpenIdConfiguration {
        OpenIdConfiguration {
            authorization_endpoint: None,
            backchannel_logout_supported: None,
            claims_supported: None,
            device_authorization_endpoint: None,
            end_session_endpoint: None,
            frontchannel_logout_supported: None,
            grant_types_supported: None,
            id_token_signing_alg_values_supported: None,
            issuer: None,
            jwks_uri: None,
            response_modes_supported: None,
            response_types_supported: None,
            scopes_supported: None,
            subject_types_supported: None,
            token_endpoint: None,
            token_endpoint_auth_methods_supported: None,
            userinfo_endpoint: None,
            userinfo_signing_alg_values_supported: None,
        }
    }
}

