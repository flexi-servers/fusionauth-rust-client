/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.51.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ConsentRequest : API request for User consent types.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsentRequest {
    #[serde(rename = "consent", skip_serializing_if = "Option::is_none")]
    pub consent: Option<Box<models::Consent>>,
}

impl ConsentRequest {
    /// API request for User consent types.
    pub fn new() -> ConsentRequest {
        ConsentRequest {
            consent: None,
        }
    }
}

