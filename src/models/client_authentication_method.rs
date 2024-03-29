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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClientAuthenticationMethod {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "client_secret_basic")]
    ClientSecretBasic,
    #[serde(rename = "client_secret_post")]
    ClientSecretPost,

}

impl ToString for ClientAuthenticationMethod {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::ClientSecretBasic => String::from("client_secret_basic"),
            Self::ClientSecretPost => String::from("client_secret_post"),
        }
    }
}

impl Default for ClientAuthenticationMethod {
    fn default() -> ClientAuthenticationMethod {
        Self::None
    }
}

