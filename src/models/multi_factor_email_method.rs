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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiFactorEmailMethod {
    #[serde(rename = "templateId", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<uuid::Uuid>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl MultiFactorEmailMethod {
    pub fn new() -> MultiFactorEmailMethod {
        MultiFactorEmailMethod {
            template_id: None,
            enabled: None,
        }
    }
}

