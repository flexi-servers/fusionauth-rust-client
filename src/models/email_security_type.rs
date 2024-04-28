/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmailSecurityType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "SSL")]
    Ssl,
    #[serde(rename = "TLS")]
    Tls,

}

impl ToString for EmailSecurityType {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("NONE"),
            Self::Ssl => String::from("SSL"),
            Self::Tls => String::from("TLS"),
        }
    }
}

impl Default for EmailSecurityType {
    fn default() -> EmailSecurityType {
        Self::None
    }
}

