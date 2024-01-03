/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AttestationConveyancePreference : Used to communicate whether and how authenticator attestation should be delivered to the Relying Party

/// Used to communicate whether and how authenticator attestation should be delivered to the Relying Party
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttestationConveyancePreference {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "indirect")]
    Indirect,
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "enterprise")]
    Enterprise,

}

impl ToString for AttestationConveyancePreference {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::Indirect => String::from("indirect"),
            Self::Direct => String::from("direct"),
            Self::Enterprise => String::from("enterprise"),
        }
    }
}

impl Default for AttestationConveyancePreference {
    fn default() -> AttestationConveyancePreference {
        Self::None
    }
}




