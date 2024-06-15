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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Samlv2Configuration {
    #[serde(rename = "assertionEncryptionConfiguration", skip_serializing_if = "Option::is_none")]
    pub assertion_encryption_configuration: Option<Box<models::Samlv2AssertionEncryptionConfiguration>>,
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(rename = "authorizedRedirectURLs", skip_serializing_if = "Option::is_none")]
    pub authorized_redirect_urls: Option<Vec<String>>,
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(rename = "defaultVerificationKeyId", skip_serializing_if = "Option::is_none")]
    pub default_verification_key_id: Option<uuid::Uuid>,
    #[serde(rename = "initiatedLogin", skip_serializing_if = "Option::is_none")]
    pub initiated_login: Option<Box<models::Samlv2IdPInitiatedLoginConfiguration>>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "keyId", skip_serializing_if = "Option::is_none")]
    pub key_id: Option<uuid::Uuid>,
    #[serde(rename = "loginHintConfiguration", skip_serializing_if = "Option::is_none")]
    pub login_hint_configuration: Option<Box<models::LoginHintConfiguration>>,
    #[serde(rename = "logout", skip_serializing_if = "Option::is_none")]
    pub logout: Option<Box<models::Samlv2Logout>>,
    #[serde(rename = "logoutURL", skip_serializing_if = "Option::is_none")]
    pub logout_url: Option<String>,
    #[serde(rename = "requireSignedRequests", skip_serializing_if = "Option::is_none")]
    pub require_signed_requests: Option<bool>,
    #[serde(rename = "xmlSignatureC14nMethod", skip_serializing_if = "Option::is_none")]
    pub xml_signature_c14n_method: Option<models::CanonicalizationMethod>,
    #[serde(rename = "xmlSignatureLocation", skip_serializing_if = "Option::is_none")]
    pub xml_signature_location: Option<models::XmlSignatureLocation>,
    #[serde(rename = "callbackURL", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Samlv2Configuration {
    pub fn new() -> Samlv2Configuration {
        Samlv2Configuration {
            assertion_encryption_configuration: None,
            audience: None,
            authorized_redirect_urls: None,
            debug: None,
            default_verification_key_id: None,
            initiated_login: None,
            issuer: None,
            key_id: None,
            login_hint_configuration: None,
            logout: None,
            logout_url: None,
            require_signed_requests: None,
            xml_signature_c14n_method: None,
            xml_signature_location: None,
            callback_url: None,
            enabled: None,
        }
    }
}

