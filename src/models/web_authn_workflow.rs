/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebAuthnWorkflow : Identifies the WebAuthn workflow. This will affect the parameters used for credential creation  and request based on the Tenant configuration.

/// Identifies the WebAuthn workflow. This will affect the parameters used for credential creation  and request based on the Tenant configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebAuthnWorkflow {
    #[serde(rename = "bootstrap")]
    Bootstrap,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "reauthentication")]
    Reauthentication,

}

impl ToString for WebAuthnWorkflow {
    fn to_string(&self) -> String {
        match self {
            Self::Bootstrap => String::from("bootstrap"),
            Self::General => String::from("general"),
            Self::Reauthentication => String::from("reauthentication"),
        }
    }
}

impl Default for WebAuthnWorkflow {
    fn default() -> WebAuthnWorkflow {
        Self::Bootstrap
    }
}




