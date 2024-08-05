/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.52.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CoseKeyType : COSE key type
/// COSE key type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CoseKeyType {
    #[serde(rename = "Reserved")]
    Reserved,
    #[serde(rename = "OKP")]
    Okp,
    #[serde(rename = "EC2")]
    Ec2,
    #[serde(rename = "RSA")]
    Rsa,
    #[serde(rename = "Symmetric")]
    Symmetric,

}

impl std::fmt::Display for CoseKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Reserved => write!(f, "Reserved"),
            Self::Okp => write!(f, "OKP"),
            Self::Ec2 => write!(f, "EC2"),
            Self::Rsa => write!(f, "RSA"),
            Self::Symmetric => write!(f, "Symmetric"),
        }
    }
}

impl Default for CoseKeyType {
    fn default() -> CoseKeyType {
        Self::Reserved
    }
}

