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

/// EmailTemplateResponse : Email template response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailTemplateResponse {
    #[serde(rename = "emailTemplate", skip_serializing_if = "Option::is_none")]
    pub email_template: Option<Box<models::EmailTemplate>>,
    #[serde(rename = "emailTemplates", skip_serializing_if = "Option::is_none")]
    pub email_templates: Option<Vec<models::EmailTemplate>>,
}

impl EmailTemplateResponse {
    /// Email template response.
    pub fn new() -> EmailTemplateResponse {
        EmailTemplateResponse {
            email_template: None,
            email_templates: None,
        }
    }
}

