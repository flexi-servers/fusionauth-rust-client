/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ExpiryUnit : 

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExpiryUnit {
    #[serde(rename = "MINUTES")]
    Minutes,
    #[serde(rename = "HOURS")]
    Hours,
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "WEEKS")]
    Weeks,
    #[serde(rename = "MONTHS")]
    Months,
    #[serde(rename = "YEARS")]
    Years,

}

impl ToString for ExpiryUnit {
    fn to_string(&self) -> String {
        match self {
            Self::Minutes => String::from("MINUTES"),
            Self::Hours => String::from("HOURS"),
            Self::Days => String::from("DAYS"),
            Self::Weeks => String::from("WEEKS"),
            Self::Months => String::from("MONTHS"),
            Self::Years => String::from("YEARS"),
        }
    }
}

impl Default for ExpiryUnit {
    fn default() -> ExpiryUnit {
        Self::Minutes
    }
}



