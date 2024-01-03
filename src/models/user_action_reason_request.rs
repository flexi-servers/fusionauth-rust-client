/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserActionReasonRequest : User Action Reason API request object.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserActionReasonRequest {
    #[serde(rename = "userActionReason", skip_serializing_if = "Option::is_none")]
    pub user_action_reason: Option<Box<crate::models::UserActionReason>>,
}

impl UserActionReasonRequest {
    /// User Action Reason API request object.
    pub fn new() -> UserActionReasonRequest {
        UserActionReasonRequest {
            user_action_reason: None,
        }
    }
}


