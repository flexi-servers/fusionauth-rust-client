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

/// PublicKeyCredentialEntity : Describes a user account or WebAuthn Relying Party associated with a public key credential
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKeyCredentialEntity {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PublicKeyCredentialEntity {
    /// Describes a user account or WebAuthn Relying Party associated with a public key credential
    pub fn new() -> PublicKeyCredentialEntity {
        PublicKeyCredentialEntity {
            name: None,
        }
    }
}

