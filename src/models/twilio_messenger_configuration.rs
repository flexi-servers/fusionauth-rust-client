/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TwilioMessengerConfiguration : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwilioMessengerConfiguration {
    #[serde(rename = "accountSID", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "authToken", skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "fromPhoneNumber", skip_serializing_if = "Option::is_none")]
    pub from_phone_number: Option<String>,
    #[serde(rename = "messagingServiceSid", skip_serializing_if = "Option::is_none")]
    pub messaging_service_sid: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "transport", skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::MessengerType>,
}

impl TwilioMessengerConfiguration {
    /// 
    pub fn new() -> TwilioMessengerConfiguration {
        TwilioMessengerConfiguration {
            account_sid: None,
            auth_token: None,
            from_phone_number: None,
            messaging_service_sid: None,
            url: None,
            data: None,
            debug: None,
            id: None,
            insert_instant: None,
            last_update_instant: None,
            name: None,
            transport: None,
            r#type: None,
        }
    }
}


