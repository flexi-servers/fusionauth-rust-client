/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.54.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantOAuth2Configuration {
    #[serde(rename = "clientCredentialsAccessTokenPopulateLambdaId", skip_serializing_if = "Option::is_none")]
    pub client_credentials_access_token_populate_lambda_id: Option<uuid::Uuid>,
}

impl TenantOAuth2Configuration {
    pub fn new() -> TenantOAuth2Configuration {
        TenantOAuth2Configuration {
            client_credentials_access_token_populate_lambda_id: None,
        }
    }
}

