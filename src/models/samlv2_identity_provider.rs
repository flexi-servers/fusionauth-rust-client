/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Samlv2IdentityProvider : SAML v2 identity provider configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Samlv2IdentityProvider {
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<serde_json::Value>>,
    #[serde(rename = "assertionConfiguration", skip_serializing_if = "Option::is_none")]
    pub assertion_configuration: Option<Box<models::Samlv2AssertionConfiguration>>,
    #[serde(rename = "buttonImageURL", skip_serializing_if = "Option::is_none")]
    pub button_image_url: Option<String>,
    #[serde(rename = "buttonText", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(rename = "idpEndpoint", skip_serializing_if = "Option::is_none")]
    pub idp_endpoint: Option<String>,
    #[serde(rename = "idpInitiatedConfiguration", skip_serializing_if = "Option::is_none")]
    pub idp_initiated_configuration: Option<Box<models::Samlv2IdpInitiatedConfiguration>>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "loginHintConfiguration", skip_serializing_if = "Option::is_none")]
    pub login_hint_configuration: Option<Box<models::LoginHintConfiguration>>,
    #[serde(rename = "nameIdFormat", skip_serializing_if = "Option::is_none")]
    pub name_id_format: Option<String>,
    #[serde(rename = "postRequest", skip_serializing_if = "Option::is_none")]
    pub post_request: Option<bool>,
    #[serde(rename = "requestSigningKeyId", skip_serializing_if = "Option::is_none")]
    pub request_signing_key_id: Option<uuid::Uuid>,
    #[serde(rename = "signRequest", skip_serializing_if = "Option::is_none")]
    pub sign_request: Option<bool>,
    #[serde(rename = "xmlSignatureC14nMethod", skip_serializing_if = "Option::is_none")]
    pub xml_signature_c14n_method: Option<models::CanonicalizationMethod>,
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

impl Samlv2IdentityProvider {
    /// SAML v2 identity provider configuration.
    pub fn new() -> Samlv2IdentityProvider {
        Samlv2IdentityProvider {
            domains: None,
            assertion_configuration: None,
            button_image_url: None,
            button_text: None,
            idp_endpoint: None,
            idp_initiated_configuration: None,
            issuer: None,
            login_hint_configuration: None,
            name_id_format: None,
            post_request: None,
            request_signing_key_id: None,
            sign_request: None,
            xml_signature_c14n_method: None,
            email_claim: None,
            key_id: None,
            unique_id_claim: None,
            use_name_id_for_email: None,
            username_claim: None,
        }
    }
}

