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

/// HttpMethod : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "PATCH")]
    Patch,

}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Get => String::from("GET"),
            Self::Post => String::from("POST"),
            Self::Put => String::from("PUT"),
            Self::Delete => String::from("DELETE"),
            Self::Head => String::from("HEAD"),
            Self::Options => String::from("OPTIONS"),
            Self::Patch => String::from("PATCH"),
        }
    }
}

impl Default for HttpMethod {
    fn default() -> HttpMethod {
        Self::Get
    }
}

