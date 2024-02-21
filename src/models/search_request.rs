/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SearchRequest : Search API request.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchRequest {
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<Box<crate::models::UserSearchCriteria>>,
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

impl SearchRequest {
    /// Search API request.
    pub fn new() -> SearchRequest {
        SearchRequest {
            search: None,
            expand: None,
        }
    }
}


