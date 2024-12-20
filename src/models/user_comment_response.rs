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

/// UserCommentResponse : User Comment Response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCommentResponse {
    #[serde(rename = "userComment", skip_serializing_if = "Option::is_none")]
    pub user_comment: Option<Box<models::UserComment>>,
    #[serde(rename = "userComments", skip_serializing_if = "Option::is_none")]
    pub user_comments: Option<Vec<models::UserComment>>,
}

impl UserCommentResponse {
    /// User Comment Response
    pub fn new() -> UserCommentResponse {
        UserCommentResponse {
            user_comment: None,
            user_comments: None,
        }
    }
}

