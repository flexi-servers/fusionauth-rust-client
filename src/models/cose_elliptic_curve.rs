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

/// CoseEllipticCurve : COSE Elliptic Curve identifier to determine which elliptic curve to use with a given key
/// COSE Elliptic Curve identifier to determine which elliptic curve to use with a given key
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CoseEllipticCurve {
    #[serde(rename = "Reserved")]
    Reserved,
    #[serde(rename = "P256")]
    P256,
    #[serde(rename = "P384")]
    P384,
    #[serde(rename = "P521")]
    P521,
    #[serde(rename = "X25519")]
    X25519,
    #[serde(rename = "X448")]
    X448,
    #[serde(rename = "Ed25519")]
    Ed25519,
    #[serde(rename = "Ed448")]
    Ed448,
    #[serde(rename = "Secp256k1")]
    Secp256k1,

}

impl ToString for CoseEllipticCurve {
    fn to_string(&self) -> String {
        match self {
            Self::Reserved => String::from("Reserved"),
            Self::P256 => String::from("P256"),
            Self::P384 => String::from("P384"),
            Self::P521 => String::from("P521"),
            Self::X25519 => String::from("X25519"),
            Self::X448 => String::from("X448"),
            Self::Ed25519 => String::from("Ed25519"),
            Self::Ed448 => String::from("Ed448"),
            Self::Secp256k1 => String::from("Secp256k1"),
        }
    }
}

impl Default for CoseEllipticCurve {
    fn default() -> CoseEllipticCurve {
        Self::Reserved
    }
}

