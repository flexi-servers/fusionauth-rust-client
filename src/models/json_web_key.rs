/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// JsonWebKey : A JSON Web Key as defined by <a href=\"https:tools.ietf.orghtmlrfc7517#section-4\">RFC 7517 JSON Web Key (JWK)  Section 4<a> and <a href=\"https:tools.ietf.orghtmlrfc7518\">RFC 7518 JSON Web Algorithms (JWA)<a>.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonWebKey {
    #[serde(rename = "alg", skip_serializing_if = "Option::is_none")]
    pub alg: Option<models::Algorithm>,
    #[serde(rename = "crv", skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    #[serde(rename = "dp", skip_serializing_if = "Option::is_none")]
    pub dp: Option<String>,
    #[serde(rename = "dq", skip_serializing_if = "Option::is_none")]
    pub dq: Option<String>,
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(rename = "kty", skip_serializing_if = "Option::is_none")]
    pub kty: Option<models::KeyType>,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(rename = "other", skip_serializing_if = "Option::is_none")]
    pub other: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename = "qi", skip_serializing_if = "Option::is_none")]
    pub qi: Option<String>,
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub r#use: Option<String>,
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(rename = "x5c", skip_serializing_if = "Option::is_none")]
    pub x5c: Option<Vec<String>>,
    #[serde(rename = "x5t", skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,
    #[serde(rename = "x5t#S256", skip_serializing_if = "Option::is_none")]
    pub x5t_hash_s256: Option<String>,
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}

impl JsonWebKey {
    /// A JSON Web Key as defined by <a href=\"https:tools.ietf.orghtmlrfc7517#section-4\">RFC 7517 JSON Web Key (JWK)  Section 4<a> and <a href=\"https:tools.ietf.orghtmlrfc7518\">RFC 7518 JSON Web Algorithms (JWA)<a>.
    pub fn new() -> JsonWebKey {
        JsonWebKey {
            alg: None,
            crv: None,
            d: None,
            dp: None,
            dq: None,
            e: None,
            kid: None,
            kty: None,
            n: None,
            other: None,
            p: None,
            q: None,
            qi: None,
            r#use: None,
            x: None,
            x5c: None,
            x5t: None,
            x5t_hash_s256: None,
            y: None,
        }
    }
}

