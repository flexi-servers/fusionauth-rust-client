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

/// Requirable : Something that can be required and thus also optional. This currently extends Enableable because anything that is  requiredoptional is almost always enableable as well.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Requirable {
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Requirable {
    /// Something that can be required and thus also optional. This currently extends Enableable because anything that is  requiredoptional is almost always enableable as well.
    pub fn new() -> Requirable {
        Requirable {
            required: None,
            enabled: None,
        }
    }
}

