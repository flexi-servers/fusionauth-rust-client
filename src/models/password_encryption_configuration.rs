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

/// PasswordEncryptionConfiguration : Password Encryption Scheme Configuration
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordEncryptionConfiguration {
    #[serde(rename = "encryptionScheme", skip_serializing_if = "Option::is_none")]
    pub encryption_scheme: Option<String>,
    #[serde(rename = "encryptionSchemeFactor", skip_serializing_if = "Option::is_none")]
    pub encryption_scheme_factor: Option<i32>,
    #[serde(rename = "modifyEncryptionSchemeOnLogin", skip_serializing_if = "Option::is_none")]
    pub modify_encryption_scheme_on_login: Option<bool>,
}

impl PasswordEncryptionConfiguration {
    /// Password Encryption Scheme Configuration
    pub fn new() -> PasswordEncryptionConfiguration {
        PasswordEncryptionConfiguration {
            encryption_scheme: None,
            encryption_scheme_factor: None,
            modify_encryption_scheme_on_login: None,
        }
    }
}

