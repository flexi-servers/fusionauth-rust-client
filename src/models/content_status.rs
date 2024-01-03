/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContentStatus : Status for content like usernames, profile attributes, etc.

/// Status for content like usernames, profile attributes, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REJECTED")]
    Rejected,

}

impl ToString for ContentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("ACTIVE"),
            Self::Pending => String::from("PENDING"),
            Self::Rejected => String::from("REJECTED"),
        }
    }
}

impl Default for ContentStatus {
    fn default() -> ContentStatus {
        Self::Active
    }
}




