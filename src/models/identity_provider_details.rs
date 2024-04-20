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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderDetails {
    #[serde(rename = "applicationIds", skip_serializing_if = "Option::is_none")]
    pub application_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "idpEndpoint", skip_serializing_if = "Option::is_none")]
    pub idp_endpoint: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "oauth2", skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<Box<models::IdentityProviderOauth2Configuration>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::IdentityProviderType>,
}

impl IdentityProviderDetails {
    pub fn new() -> IdentityProviderDetails {
        IdentityProviderDetails {
            application_ids: None,
            id: None,
            idp_endpoint: None,
            name: None,
            oauth2: None,
            r#type: None,
        }
    }
}

