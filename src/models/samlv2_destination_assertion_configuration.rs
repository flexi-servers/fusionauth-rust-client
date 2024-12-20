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

/// Samlv2DestinationAssertionConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Samlv2DestinationAssertionConfiguration {
    #[serde(rename = "alternates", skip_serializing_if = "Option::is_none")]
    pub alternates: Option<Vec<String>>,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<models::Samlv2DestinationAssertionPolicy>,
}

impl Samlv2DestinationAssertionConfiguration {
    /// 
    pub fn new() -> Samlv2DestinationAssertionConfiguration {
        Samlv2DestinationAssertionConfiguration {
            alternates: None,
            policy: None,
        }
    }
}

