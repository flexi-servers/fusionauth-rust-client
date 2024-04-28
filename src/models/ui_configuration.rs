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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiConfiguration {
    #[serde(rename = "headerColor", skip_serializing_if = "Option::is_none")]
    pub header_color: Option<String>,
    #[serde(rename = "logoURL", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "menuFontColor", skip_serializing_if = "Option::is_none")]
    pub menu_font_color: Option<String>,
}

impl UiConfiguration {
    pub fn new() -> UiConfiguration {
        UiConfiguration {
            header_color: None,
            logo_url: None,
            menu_font_color: None,
        }
    }
}

