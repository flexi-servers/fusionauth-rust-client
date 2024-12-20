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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Samlv2AssertionEncryptionConfiguration {
    #[serde(rename = "digestAlgorithm", skip_serializing_if = "Option::is_none")]
    pub digest_algorithm: Option<String>,
    #[serde(rename = "encryptionAlgorithm", skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    #[serde(rename = "keyLocation", skip_serializing_if = "Option::is_none")]
    pub key_location: Option<String>,
    #[serde(rename = "keyTransportAlgorithm", skip_serializing_if = "Option::is_none")]
    pub key_transport_algorithm: Option<String>,
    #[serde(rename = "keyTransportEncryptionKeyId", skip_serializing_if = "Option::is_none")]
    pub key_transport_encryption_key_id: Option<uuid::Uuid>,
    #[serde(rename = "maskGenerationFunction", skip_serializing_if = "Option::is_none")]
    pub mask_generation_function: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Samlv2AssertionEncryptionConfiguration {
    pub fn new() -> Samlv2AssertionEncryptionConfiguration {
        Samlv2AssertionEncryptionConfiguration {
            digest_algorithm: None,
            encryption_algorithm: None,
            key_location: None,
            key_transport_algorithm: None,
            key_transport_encryption_key_id: None,
            mask_generation_function: None,
            enabled: None,
        }
    }
}

