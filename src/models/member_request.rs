/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MemberRequest : Group Member Request



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberRequest {
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::GroupMember>>,
}

impl MemberRequest {
    /// Group Member Request
    pub fn new() -> MemberRequest {
        MemberRequest {
            members: None,
        }
    }
}

