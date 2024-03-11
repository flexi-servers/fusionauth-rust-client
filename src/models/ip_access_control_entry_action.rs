/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// IpAccessControlEntryAction : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IpAccessControlEntryAction {
    #[serde(rename = "Allow")]
    Allow,
    #[serde(rename = "Block")]
    Block,

}

impl ToString for IpAccessControlEntryAction {
    fn to_string(&self) -> String {
        match self {
            Self::Allow => String::from("Allow"),
            Self::Block => String::from("Block"),
        }
    }
}

impl Default for IpAccessControlEntryAction {
    fn default() -> IpAccessControlEntryAction {
        Self::Allow
    }
}

