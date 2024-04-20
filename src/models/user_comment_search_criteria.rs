/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// UserCommentSearchCriteria : Search criteria for user comments.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCommentSearchCriteria {
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "commenterId", skip_serializing_if = "Option::is_none")]
    pub commenter_id: Option<uuid::Uuid>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<uuid::Uuid>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(rename = "numberOfResults", skip_serializing_if = "Option::is_none")]
    pub number_of_results: Option<i32>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "startRow", skip_serializing_if = "Option::is_none")]
    pub start_row: Option<i32>,
}

impl UserCommentSearchCriteria {
    /// Search criteria for user comments.
    pub fn new() -> UserCommentSearchCriteria {
        UserCommentSearchCriteria {
            comment: None,
            commenter_id: None,
            tenant_id: None,
            user_id: None,
            number_of_results: None,
            order_by: None,
            start_row: None,
        }
    }
}

