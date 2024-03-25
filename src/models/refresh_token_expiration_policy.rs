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

/// RefreshTokenExpirationPolicy : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RefreshTokenExpirationPolicy {
    #[serde(rename = "Fixed")]
    Fixed,
    #[serde(rename = "SlidingWindow")]
    SlidingWindow,
    #[serde(rename = "SlidingWindowWithMaximumLifetime")]
    SlidingWindowWithMaximumLifetime,

}

impl ToString for RefreshTokenExpirationPolicy {
    fn to_string(&self) -> String {
        match self {
            Self::Fixed => String::from("Fixed"),
            Self::SlidingWindow => String::from("SlidingWindow"),
            Self::SlidingWindowWithMaximumLifetime => String::from("SlidingWindowWithMaximumLifetime"),
        }
    }
}

impl Default for RefreshTokenExpirationPolicy {
    fn default() -> RefreshTokenExpirationPolicy {
        Self::Fixed
    }
}

