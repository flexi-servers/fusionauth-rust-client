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

/// UserActionOption : Models content user action options.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserActionOption {
    /// Models a set of localized Strings that can be stored as JSON.
    #[serde(rename = "localizedNames", skip_serializing_if = "Option::is_none")]
    pub localized_names: Option<serde_json::Value>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UserActionOption {
    /// Models content user action options.
    pub fn new() -> UserActionOption {
        UserActionOption {
            localized_names: None,
            name: None,
        }
    }
}

