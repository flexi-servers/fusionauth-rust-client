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

/// EmailTemplateSearchResponse : Email template search response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailTemplateSearchResponse {
    #[serde(rename = "emailTemplates", skip_serializing_if = "Option::is_none")]
    pub email_templates: Option<Vec<models::EmailTemplate>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl EmailTemplateSearchResponse {
    /// Email template search response
    pub fn new() -> EmailTemplateSearchResponse {
        EmailTemplateSearchResponse {
            email_templates: None,
            total: None,
        }
    }
}

