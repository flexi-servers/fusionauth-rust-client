/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Samlv2IdpInitiatedConfiguration : Config for regular SAML IDP configurations that support IdP initiated requests



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Samlv2IdpInitiatedConfiguration {
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Samlv2IdpInitiatedConfiguration {
    /// Config for regular SAML IDP configurations that support IdP initiated requests
    pub fn new() -> Samlv2IdpInitiatedConfiguration {
        Samlv2IdpInitiatedConfiguration {
            issuer: None,
            enabled: None,
        }
    }
}


