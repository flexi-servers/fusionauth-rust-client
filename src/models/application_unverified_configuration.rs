/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// ApplicationUnverifiedConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationUnverifiedConfiguration {
    #[serde(rename = "registration", skip_serializing_if = "Option::is_none")]
    pub registration: Option<models::UnverifiedBehavior>,
    #[serde(rename = "verificationStrategy", skip_serializing_if = "Option::is_none")]
    pub verification_strategy: Option<models::VerificationStrategy>,
    #[serde(rename = "whenGated", skip_serializing_if = "Option::is_none")]
    pub when_gated: Option<Box<models::RegistrationUnverifiedOptions>>,
}

impl ApplicationUnverifiedConfiguration {
    /// 
    pub fn new() -> ApplicationUnverifiedConfiguration {
        ApplicationUnverifiedConfiguration {
            registration: None,
            verification_strategy: None,
            when_gated: None,
        }
    }
}

