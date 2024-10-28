/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Attachment : This class is a simple attachment with a byte array, name and MIME type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "attachment", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<std::path::PathBuf>,
    #[serde(rename = "mime", skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Attachment {
    /// This class is a simple attachment with a byte array, name and MIME type.
    pub fn new() -> Attachment {
        Attachment {
            attachment: None,
            mime: None,
            name: None,
        }
    }
}

