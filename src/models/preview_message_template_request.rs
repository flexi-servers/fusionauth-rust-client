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

/// PreviewMessageTemplateRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreviewMessageTemplateRequest {
    /// A Locale object represents a specific geographical, political, or cultural region.
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "messageTemplate", skip_serializing_if = "Option::is_none")]
    pub message_template: Option<Box<models::MessageTemplate>>,
}

impl PreviewMessageTemplateRequest {
    /// 
    pub fn new() -> PreviewMessageTemplateRequest {
        PreviewMessageTemplateRequest {
            locale: None,
            message_template: None,
        }
    }
}

