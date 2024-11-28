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

/// PublicKeyCredentialCreationOptions : Allows the Relying Party to specify desired attributes of a new credential.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKeyCredentialCreationOptions {
    #[serde(rename = "attestation", skip_serializing_if = "Option::is_none")]
    pub attestation: Option<models::AttestationConveyancePreference>,
    #[serde(rename = "authenticatorSelection", skip_serializing_if = "Option::is_none")]
    pub authenticator_selection: Option<Box<models::AuthenticatorSelectionCriteria>>,
    #[serde(rename = "challenge", skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    #[serde(rename = "excludeCredentials", skip_serializing_if = "Option::is_none")]
    pub exclude_credentials: Option<Vec<models::PublicKeyCredentialDescriptor>>,
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Box<models::WebAuthnRegistrationExtensionOptions>>,
    #[serde(rename = "pubKeyCredParams", skip_serializing_if = "Option::is_none")]
    pub pub_key_cred_params: Option<Vec<models::PublicKeyCredentialParameters>>,
    #[serde(rename = "rp", skip_serializing_if = "Option::is_none")]
    pub rp: Option<Box<models::PublicKeyCredentialRelyingPartyEntity>>,
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::PublicKeyCredentialUserEntity>>,
}

impl PublicKeyCredentialCreationOptions {
    /// Allows the Relying Party to specify desired attributes of a new credential.
    pub fn new() -> PublicKeyCredentialCreationOptions {
        PublicKeyCredentialCreationOptions {
            attestation: None,
            authenticator_selection: None,
            challenge: None,
            exclude_credentials: None,
            extensions: None,
            pub_key_cred_params: None,
            rp: None,
            timeout: None,
            user: None,
        }
    }
}

