/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WebAuthnLoginRequest : Request to complete the WebAuthn registration ceremony
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnLoginRequest {
    #[serde(rename = "credential", skip_serializing_if = "Option::is_none")]
    pub credential: Option<Box<models::WebAuthnPublicKeyAuthenticationRequest>>,
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "rpId", skip_serializing_if = "Option::is_none")]
    pub rp_id: Option<String>,
    #[serde(rename = "twoFactorTrustId", skip_serializing_if = "Option::is_none")]
    pub two_factor_trust_id: Option<String>,
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

impl WebAuthnLoginRequest {
    /// Request to complete the WebAuthn registration ceremony
    pub fn new() -> WebAuthnLoginRequest {
        WebAuthnLoginRequest {
            credential: None,
            origin: None,
            rp_id: None,
            two_factor_trust_id: None,
            application_id: None,
            ip_address: None,
            meta_data: None,
            new_device: None,
            no_jwt: None,
        }
    }
}

