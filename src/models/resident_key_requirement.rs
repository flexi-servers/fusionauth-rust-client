/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResidentKeyRequirement : Describes the Relying Party's requirements for <a href=\"https:www.w3.orgTRwebauthn-2#client-side-discoverable-credential\">client-side  discoverable credentials<a> (formerly known as \"resident keys\")

/// Describes the Relying Party's requirements for <a href=\"https:www.w3.orgTRwebauthn-2#client-side-discoverable-credential\">client-side  discoverable credentials<a> (formerly known as \"resident keys\")
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResidentKeyRequirement {
    #[serde(rename = "discouraged")]
    Discouraged,
    #[serde(rename = "preferred")]
    Preferred,
    #[serde(rename = "required")]
    Required,

}

impl ToString for ResidentKeyRequirement {
    fn to_string(&self) -> String {
        match self {
            Self::Discouraged => String::from("discouraged"),
            Self::Preferred => String::from("preferred"),
            Self::Required => String::from("required"),
        }
    }
}

impl Default for ResidentKeyRequirement {
    fn default() -> ResidentKeyRequirement {
        Self::Discouraged
    }
}



