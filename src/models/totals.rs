/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Totals {
    #[serde(rename = "logins", skip_serializing_if = "Option::is_none")]
    pub logins: Option<i64>,
    #[serde(rename = "registrations", skip_serializing_if = "Option::is_none")]
    pub registrations: Option<i64>,
    #[serde(rename = "totalRegistrations", skip_serializing_if = "Option::is_none")]
    pub total_registrations: Option<i64>,
}

impl Totals {
    pub fn new() -> Totals {
        Totals {
            logins: None,
            registrations: None,
            total_registrations: None,
        }
    }
}


