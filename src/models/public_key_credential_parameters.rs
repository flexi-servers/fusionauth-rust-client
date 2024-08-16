/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PublicKeyCredentialParameters : Supply information on credential type and algorithm to the <i>authenticator<i>.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKeyCredentialParameters {
    #[serde(rename = "alg", skip_serializing_if = "Option::is_none")]
    pub alg: Option<models::CoseAlgorithmIdentifier>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::PublicKeyCredentialType>,
}

impl PublicKeyCredentialParameters {
    /// Supply information on credential type and algorithm to the <i>authenticator<i>.
    pub fn new() -> PublicKeyCredentialParameters {
        PublicKeyCredentialParameters {
            alg: None,
            r#type: None,
        }
    }
}

