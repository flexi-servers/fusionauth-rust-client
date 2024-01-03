/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Samlv2AssertionConfiguration : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Samlv2AssertionConfiguration {
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<Box<crate::models::Samlv2DestinationAssertionConfiguration>>,
}

impl Samlv2AssertionConfiguration {
    /// 
    pub fn new() -> Samlv2AssertionConfiguration {
        Samlv2AssertionConfiguration {
            destination: None,
        }
    }
}


