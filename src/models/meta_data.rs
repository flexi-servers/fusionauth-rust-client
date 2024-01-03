/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetaData {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::DeviceInfo>>,
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<serde_json::Value>>,
}

impl MetaData {
    pub fn new() -> MetaData {
        MetaData {
            data: None,
            device: None,
            scopes: None,
        }
    }
}


