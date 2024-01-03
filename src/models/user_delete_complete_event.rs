/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserDeleteCompleteEvent : Models the User Event (and can be converted to JSON) that is used for all user modifications (create, update,  delete).  <p>  This is different than user.delete because it is sent after the tx is committed, this cannot be transactional.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDeleteCompleteEvent {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "createInstant", skip_serializing_if = "Option::is_none")]
    pub create_instant: Option<i64>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Box<crate::models::EventInfo>>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<uuid::Uuid>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::EventType>,
}

impl UserDeleteCompleteEvent {
    /// Models the User Event (and can be converted to JSON) that is used for all user modifications (create, update,  delete).  <p>  This is different than user.delete because it is sent after the tx is committed, this cannot be transactional.
    pub fn new() -> UserDeleteCompleteEvent {
        UserDeleteCompleteEvent {
            user: None,
            create_instant: None,
            id: None,
            info: None,
            tenant_id: None,
            r#type: None,
        }
    }
}


