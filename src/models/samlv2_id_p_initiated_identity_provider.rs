/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.54.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Samlv2IdPInitiatedIdentityProvider : SAML v2 IdP Initiated identity provider configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Samlv2IdPInitiatedIdentityProvider {
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "emailClaim", skip_serializing_if = "Option::is_none")]
    pub email_claim: Option<String>,
    #[serde(rename = "keyId", skip_serializing_if = "Option::is_none")]
    pub key_id: Option<uuid::Uuid>,
    #[serde(rename = "uniqueIdClaim", skip_serializing_if = "Option::is_none")]
    pub unique_id_claim: Option<String>,
    #[serde(rename = "useNameIdForEmail", skip_serializing_if = "Option::is_none")]
    pub use_name_id_for_email: Option<bool>,
    #[serde(rename = "usernameClaim", skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
}

impl Samlv2IdPInitiatedIdentityProvider {
    /// SAML v2 IdP Initiated identity provider configuration.
    pub fn new() -> Samlv2IdPInitiatedIdentityProvider {
        Samlv2IdPInitiatedIdentityProvider {
            issuer: None,
            email_claim: None,
            key_id: None,
            unique_id_claim: None,
            use_name_id_for_email: None,
            username_claim: None,
        }
    }
}

