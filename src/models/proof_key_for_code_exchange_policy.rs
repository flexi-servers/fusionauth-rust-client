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

/// ProofKeyForCodeExchangePolicy : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProofKeyForCodeExchangePolicy {
    #[serde(rename = "Required")]
    Required,
    #[serde(rename = "NotRequired")]
    NotRequired,
    #[serde(rename = "NotRequiredWhenUsingClientAuthentication")]
    NotRequiredWhenUsingClientAuthentication,

}

impl ToString for ProofKeyForCodeExchangePolicy {
    fn to_string(&self) -> String {
        match self {
            Self::Required => String::from("Required"),
            Self::NotRequired => String::from("NotRequired"),
            Self::NotRequiredWhenUsingClientAuthentication => String::from("NotRequiredWhenUsingClientAuthentication"),
        }
    }
}

impl Default for ProofKeyForCodeExchangePolicy {
    fn default() -> ProofKeyForCodeExchangePolicy {
        Self::Required
    }
}

