/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UniqueUsernameStrategy {
    #[serde(rename = "Always")]
    Always,
    #[serde(rename = "OnCollision")]
    OnCollision,

}

impl ToString for UniqueUsernameStrategy {
    fn to_string(&self) -> String {
        match self {
            Self::Always => String::from("Always"),
            Self::OnCollision => String::from("OnCollision"),
        }
    }
}

impl Default for UniqueUsernameStrategy {
    fn default() -> UniqueUsernameStrategy {
        Self::Always
    }
}




