/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConnectorType : The types of connectors. This enum is stored as an ordinal on the <code>identities<code> table, order must be maintained.

/// The types of connectors. This enum is stored as an ordinal on the <code>identities<code> table, order must be maintained.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConnectorType {
    #[serde(rename = "FusionAuth")]
    FusionAuth,
    #[serde(rename = "Generic")]
    Generic,
    #[serde(rename = "LDAP")]
    Ldap,

}

impl ToString for ConnectorType {
    fn to_string(&self) -> String {
        match self {
            Self::FusionAuth => String::from("FusionAuth"),
            Self::Generic => String::from("Generic"),
            Self::Ldap => String::from("LDAP"),
        }
    }
}

impl Default for ConnectorType {
    fn default() -> ConnectorType {
        Self::FusionAuth
    }
}



