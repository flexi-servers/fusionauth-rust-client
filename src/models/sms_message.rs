/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// SmsMessage : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsMessage {
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "textMessage", skip_serializing_if = "Option::is_none")]
    pub text_message: Option<String>,
}

impl SmsMessage {
    /// 
    pub fn new() -> SmsMessage {
        SmsMessage {
            phone_number: None,
            text_message: None,
        }
    }
}

