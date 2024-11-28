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

/// UserLoginFailedReason : The reason for the login failure.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLoginFailedReason {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "lambdaId", skip_serializing_if = "Option::is_none")]
    pub lambda_id: Option<uuid::Uuid>,
    #[serde(rename = "lambdaResult", skip_serializing_if = "Option::is_none")]
    pub lambda_result: Option<Box<models::Errors>>,
}

impl UserLoginFailedReason {
    /// The reason for the login failure.
    pub fn new() -> UserLoginFailedReason {
        UserLoginFailedReason {
            code: None,
            lambda_id: None,
            lambda_result: None,
        }
    }
}

