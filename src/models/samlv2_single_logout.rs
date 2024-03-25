/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Samlv2SingleLogout {
    #[serde(rename = "keyId", skip_serializing_if = "Option::is_none")]
    pub key_id: Option<uuid::Uuid>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "xmlSignatureC14nMethod", skip_serializing_if = "Option::is_none")]
    pub xml_signature_c14n_method: Option<models::CanonicalizationMethod>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Samlv2SingleLogout {
    pub fn new() -> Samlv2SingleLogout {
        Samlv2SingleLogout {
            key_id: None,
            url: None,
            xml_signature_c14n_method: None,
            enabled: None,
        }
    }
}

