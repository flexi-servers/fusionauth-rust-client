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
pub struct MultiFactorAuthenticatorMethod {
    #[serde(rename = "algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<crate::models::TotpAlgorithm>,
    #[serde(rename = "codeLength", skip_serializing_if = "Option::is_none")]
    pub code_length: Option<i32>,
    #[serde(rename = "timeStep", skip_serializing_if = "Option::is_none")]
    pub time_step: Option<i32>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl MultiFactorAuthenticatorMethod {
    pub fn new() -> MultiFactorAuthenticatorMethod {
        MultiFactorAuthenticatorMethod {
            algorithm: None,
            code_length: None,
            time_step: None,
            enabled: None,
        }
    }
}

