/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// TenantDeleteRequest : Request for the Tenant API to delete a tenant rather than using the URL parameters.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantDeleteRequest {
    #[serde(rename = "async", skip_serializing_if = "Option::is_none")]
    pub r#async: Option<bool>,
    #[serde(rename = "eventInfo", skip_serializing_if = "Option::is_none")]
    pub event_info: Option<Box<models::EventInfo>>,
}

impl TenantDeleteRequest {
    /// Request for the Tenant API to delete a tenant rather than using the URL parameters.
    pub fn new() -> TenantDeleteRequest {
        TenantDeleteRequest {
            r#async: None,
            event_info: None,
        }
    }
}

