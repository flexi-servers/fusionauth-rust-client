/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FamilyMember : Models a single family member.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FamilyMember {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::FamilyRole>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl FamilyMember {
    /// Models a single family member.
    pub fn new() -> FamilyMember {
        FamilyMember {
            data: None: None,
            insert_instant: None,
            last_update_instant: None,
            owner: None,
            role: None,
            user_id: None,
        }
    }
}


