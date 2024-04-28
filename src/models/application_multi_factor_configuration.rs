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

/// ApplicationMultiFactorConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationMultiFactorConfiguration {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<models::MultiFactorEmailTemplate>>,
    #[serde(rename = "loginPolicy", skip_serializing_if = "Option::is_none")]
    pub login_policy: Option<models::MultiFactorLoginPolicy>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<Box<models::MultiFactorSmsTemplate>>,
    #[serde(rename = "trustPolicy", skip_serializing_if = "Option::is_none")]
    pub trust_policy: Option<models::ApplicationMultiFactorTrustPolicy>,
}

impl ApplicationMultiFactorConfiguration {
    /// 
    pub fn new() -> ApplicationMultiFactorConfiguration {
        ApplicationMultiFactorConfiguration {
            email: None,
            login_policy: None,
            sms: None,
            trust_policy: None,
        }
    }
}

