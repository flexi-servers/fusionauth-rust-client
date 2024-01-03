/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PublicKeyCredentialType : Defines valid credential types. This is an extension point in the WebAuthn spec. The only defined value at this time is \"public-key\"

/// Defines valid credential types. This is an extension point in the WebAuthn spec. The only defined value at this time is \"public-key\"
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PublicKeyCredentialType {
    #[serde(rename = "publicKey")]
    PublicKey,

}

impl ToString for PublicKeyCredentialType {
    fn to_string(&self) -> String {
        match self {
            Self::PublicKey => String::from("publicKey"),
        }
    }
}

impl Default for PublicKeyCredentialType {
    fn default() -> PublicKeyCredentialType {
        Self::PublicKey
    }
}




