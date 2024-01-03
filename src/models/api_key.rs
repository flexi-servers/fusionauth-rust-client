/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ApiKey : domain POJO to represent AuthenticationKey



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKey {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    #[serde(rename = "ipAccessControlListId", skip_serializing_if = "Option::is_none")]
    pub ip_access_control_list_id: Option<uuid::Uuid>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "keyManager", skip_serializing_if = "Option::is_none")]
    pub key_manager: Option<bool>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<Box<crate::models::ApiKeyMetaData>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<crate::models::ApiKeyPermissions>>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<uuid::Uuid>,
}

impl ApiKey {
    /// domain POJO to represent AuthenticationKey
    pub fn new() -> ApiKey {
        ApiKey {
            id: None,
            insert_instant: None,
            ip_access_control_list_id: None,
            key: None,
            key_manager: None,
            last_update_instant: None,
            meta_data: None,
            permissions: None,
            tenant_id: None,
        }
    }
}


