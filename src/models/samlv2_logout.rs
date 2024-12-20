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
pub struct Samlv2Logout {
    #[serde(rename = "behavior", skip_serializing_if = "Option::is_none")]
    pub behavior: Option<models::SamlLogoutBehavior>,
    #[serde(rename = "defaultVerificationKeyId", skip_serializing_if = "Option::is_none")]
    pub default_verification_key_id: Option<uuid::Uuid>,
    #[serde(rename = "keyId", skip_serializing_if = "Option::is_none")]
    pub key_id: Option<uuid::Uuid>,
    #[serde(rename = "requireSignedRequests", skip_serializing_if = "Option::is_none")]
    pub require_signed_requests: Option<bool>,
    #[serde(rename = "singleLogout", skip_serializing_if = "Option::is_none")]
    pub single_logout: Option<Box<models::Samlv2SingleLogout>>,
    #[serde(rename = "xmlSignatureC14nMethod", skip_serializing_if = "Option::is_none")]
    pub xml_signature_c14n_method: Option<models::CanonicalizationMethod>,
}

impl Samlv2Logout {
    pub fn new() -> Samlv2Logout {
        Samlv2Logout {
            behavior: None,
            default_verification_key_id: None,
            key_id: None,
            require_signed_requests: None,
            single_logout: None,
            xml_signature_c14n_method: None,
        }
    }
}

