/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// MessageTemplateRequest : A Message Template Request to the API
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageTemplateRequest {
    #[serde(rename = "messageTemplate", skip_serializing_if = "Option::is_none")]
    pub message_template: Option<Box<models::MessageTemplate>>,
}

impl MessageTemplateRequest {
    /// A Message Template Request to the API
    pub fn new() -> MessageTemplateRequest {
        MessageTemplateRequest {
            message_template: None,
        }
    }
}

