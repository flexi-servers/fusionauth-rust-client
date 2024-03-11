/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// Family : Models a family grouping of users.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Family {
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<models::FamilyMember>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
}

impl Family {
    /// Models a family grouping of users.
    pub fn new() -> Family {
        Family {
            members: None,
            id: None,
            insert_instant: None,
            last_update_instant: None,
        }
    }
}

