/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConnectorPolicy : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectorPolicy {
    #[serde(rename = "connectorId", skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<uuid::Uuid>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<serde_json::Value>>,
    #[serde(rename = "migrate", skip_serializing_if = "Option::is_none")]
    pub migrate: Option<bool>,
}

impl ConnectorPolicy {
    /// 
    pub fn new() -> ConnectorPolicy {
        ConnectorPolicy {
            connector_id: None,
            data: None,
            domains: None,
            migrate: None,
        }
    }
}


