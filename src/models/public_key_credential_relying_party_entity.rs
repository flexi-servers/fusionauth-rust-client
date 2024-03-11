/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// PublicKeyCredentialRelyingPartyEntity : Supply additional information about the Relying Party when creating a new credential
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKeyCredentialRelyingPartyEntity {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PublicKeyCredentialRelyingPartyEntity {
    /// Supply additional information about the Relying Party when creating a new credential
    pub fn new() -> PublicKeyCredentialRelyingPartyEntity {
        PublicKeyCredentialRelyingPartyEntity {
            id: None,
            name: None,
        }
    }
}

