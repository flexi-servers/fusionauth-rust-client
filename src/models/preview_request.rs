/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PreviewRequest : 



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreviewRequest {
    #[serde(rename = "emailTemplate", skip_serializing_if = "Option::is_none")]
    pub email_template: Option<Box<crate::models::EmailTemplate>>,
    /// A Locale object represents a specific geographical, political, or cultural region.
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

impl PreviewRequest {
    /// 
    pub fn new() -> PreviewRequest {
        PreviewRequest {
            email_template: None,
            locale: None,
        }
    }
}


