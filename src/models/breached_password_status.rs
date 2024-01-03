/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BreachedPasswordStatus : 

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BreachedPasswordStatus {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "ExactMatch")]
    ExactMatch,
    #[serde(rename = "SubAddressMatch")]
    SubAddressMatch,
    #[serde(rename = "PasswordOnly")]
    PasswordOnly,
    #[serde(rename = "CommonPassword")]
    CommonPassword,

}

impl ToString for BreachedPasswordStatus {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("None"),
            Self::ExactMatch => String::from("ExactMatch"),
            Self::SubAddressMatch => String::from("SubAddressMatch"),
            Self::PasswordOnly => String::from("PasswordOnly"),
            Self::CommonPassword => String::from("CommonPassword"),
        }
    }
}

impl Default for BreachedPasswordStatus {
    fn default() -> BreachedPasswordStatus {
        Self::None
    }
}



