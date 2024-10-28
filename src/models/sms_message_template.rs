/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SmsMessageTemplate : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsMessageTemplate {
    #[serde(rename = "defaultTemplate", skip_serializing_if = "Option::is_none")]
    pub default_template: Option<String>,
    /// Models a set of localized Strings that can be stored as JSON.
    #[serde(rename = "localizedTemplates", skip_serializing_if = "Option::is_none")]
    pub localized_templates: Option<serde_json::Value>,
}

impl SmsMessageTemplate {
    /// 
    pub fn new() -> SmsMessageTemplate {
        SmsMessageTemplate {
            default_template: None,
            localized_templates: None,
        }
    }
}

