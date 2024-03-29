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

/// SystemTrustedProxyConfigurationPolicy : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SystemTrustedProxyConfigurationPolicy {
    #[serde(rename = "All")]
    All,
    #[serde(rename = "OnlyConfigured")]
    OnlyConfigured,

}

impl ToString for SystemTrustedProxyConfigurationPolicy {
    fn to_string(&self) -> String {
        match self {
            Self::All => String::from("All"),
            Self::OnlyConfigured => String::from("OnlyConfigured"),
        }
    }
}

impl Default for SystemTrustedProxyConfigurationPolicy {
    fn default() -> SystemTrustedProxyConfigurationPolicy {
        Self::All
    }
}

