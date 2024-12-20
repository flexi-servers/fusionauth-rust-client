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

/// Jwt : JSON Web Token (JWT) as defined by RFC 7519.  <pre>  From RFC 7519 Section 1. Introduction:     The suggested pronunciation of JWT is the same as the English word \"jot\".  <pre>  The JWT is not Thread-Safe and should not be re-used.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Jwt {
    #[serde(rename = "aud", skip_serializing_if = "Option::is_none")]
    pub aud: Option<serde_json::Value>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "iat", skip_serializing_if = "Option::is_none")]
    pub iat: Option<i64>,
    #[serde(rename = "iss", skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "nbf", skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,
    #[serde(rename = "otherClaims", skip_serializing_if = "Option::is_none")]
    pub other_claims: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "sub", skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    #[serde(rename = "jti", skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
}

impl Jwt {
    /// JSON Web Token (JWT) as defined by RFC 7519.  <pre>  From RFC 7519 Section 1. Introduction:     The suggested pronunciation of JWT is the same as the English word \"jot\".  <pre>  The JWT is not Thread-Safe and should not be re-used.
    pub fn new() -> Jwt {
        Jwt {
            aud: None,
            exp: None,
            iat: None,
            iss: None,
            nbf: None,
            other_claims: None,
            sub: None,
            jti: None,
        }
    }
}

