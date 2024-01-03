/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ApplicationSearchCriteria : Search criteria for Applications



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationSearchCriteria {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::ObjectState>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<uuid::Uuid>,
    #[serde(rename = "numberOfResults", skip_serializing_if = "Option::is_none")]
    pub number_of_results: Option<i32>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "startRow", skip_serializing_if = "Option::is_none")]
    pub start_row: Option<i32>,
}

impl ApplicationSearchCriteria {
    /// Search criteria for Applications
    pub fn new() -> ApplicationSearchCriteria {
        ApplicationSearchCriteria {
            name: None,
            state: None,
            tenant_id: None,
            number_of_results: None,
            order_by: None,
            start_row: None,
        }
    }
}

