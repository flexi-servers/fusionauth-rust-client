/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MemberDeleteRequest : Group Member Delete Request



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberDeleteRequest {
    #[serde(rename = "memberIds", skip_serializing_if = "Option::is_none")]
    pub member_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<uuid::Uuid>>,
}

impl MemberDeleteRequest {
    /// Group Member Delete Request
    pub fn new() -> MemberDeleteRequest {
        MemberDeleteRequest {
            member_ids: None,
            members: None,
        }
    }
}


