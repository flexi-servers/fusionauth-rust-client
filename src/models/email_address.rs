/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EmailAddress : An email address.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailAddress {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

impl EmailAddress {
    /// An email address.
    pub fn new() -> EmailAddress {
        EmailAddress {
            address: None,
            display: None,
        }
    }
}


