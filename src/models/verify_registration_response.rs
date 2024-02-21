/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VerifyRegistrationResponse : 



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifyRegistrationResponse {
    #[serde(rename = "oneTimeCode", skip_serializing_if = "Option::is_none")]
    pub one_time_code: Option<String>,
    #[serde(rename = "verificationId", skip_serializing_if = "Option::is_none")]
    pub verification_id: Option<String>,
}

impl VerifyRegistrationResponse {
    /// 
    pub fn new() -> VerifyRegistrationResponse {
        VerifyRegistrationResponse {
            one_time_code: None,
            verification_id: None,
        }
    }
}


