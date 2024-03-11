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

/// ApplicationAccessControlConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationAccessControlConfiguration {
    #[serde(rename = "uiIPAccessControlListId", skip_serializing_if = "Option::is_none")]
    pub ui_ip_access_control_list_id: Option<uuid::Uuid>,
}

impl ApplicationAccessControlConfiguration {
    /// 
    pub fn new() -> ApplicationAccessControlConfiguration {
        ApplicationAccessControlConfiguration {
            ui_ip_access_control_list_id: None,
        }
    }
}

