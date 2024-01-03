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
pub struct EventLogConfiguration {
    #[serde(rename = "numberToRetain", skip_serializing_if = "Option::is_none")]
    pub number_to_retain: Option<i32>,
}

impl EventLogConfiguration {
    pub fn new() -> EventLogConfiguration {
        EventLogConfiguration {
            number_to_retain: None,
        }
    }
}

