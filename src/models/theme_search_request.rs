/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ThemeSearchRequest : Search request for Themes.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeSearchRequest {
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<Box<crate::models::ThemeSearchCriteria>>,
}

impl ThemeSearchRequest {
    /// Search request for Themes.
    pub fn new() -> ThemeSearchRequest {
        ThemeSearchRequest {
            search: None,
        }
    }
}

