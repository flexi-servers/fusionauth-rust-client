/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// BaseLoginRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseLoginRequest {
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<uuid::Uuid>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<Box<models::MetaData>>,
    #[serde(rename = "newDevice", skip_serializing_if = "Option::is_none")]
    pub new_device: Option<bool>,
    #[serde(rename = "noJWT", skip_serializing_if = "Option::is_none")]
    pub no_jwt: Option<bool>,
    #[serde(rename = "eventInfo", skip_serializing_if = "Option::is_none")]
    pub event_info: Option<Box<models::EventInfo>>,
}

impl BaseLoginRequest {
    /// 
    pub fn new() -> BaseLoginRequest {
        BaseLoginRequest {
            application_id: None,
            ip_address: None,
            meta_data: None,
            new_device: None,
            no_jwt: None,
            event_info: None,
        }
    }
}

