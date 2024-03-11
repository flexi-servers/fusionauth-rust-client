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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LdapSecurityMethod {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "LDAPS")]
    Ldaps,
    #[serde(rename = "StartTLS")]
    StartTls,

}

impl ToString for LdapSecurityMethod {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("None"),
            Self::Ldaps => String::from("LDAPS"),
            Self::StartTls => String::from("StartTLS"),
        }
    }
}

impl Default for LdapSecurityMethod {
    fn default() -> LdapSecurityMethod {
        Self::None
    }
}

