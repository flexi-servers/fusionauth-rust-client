/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.54.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Email : This class is an abstraction of a simple email message.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Email {
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<models::Attachment>>,
    #[serde(rename = "bcc", skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<models::EmailAddress>>,
    #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<models::EmailAddress>>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<Box<models::EmailAddress>>,
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<Box<models::EmailAddress>>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<models::EmailAddress>>,
}

impl Email {
    /// This class is an abstraction of a simple email message.
    pub fn new() -> Email {
        Email {
            attachments: None,
            bcc: None,
            cc: None,
            from: None,
            html: None,
            reply_to: None,
            subject: None,
            text: None,
            to: None,
        }
    }
}

