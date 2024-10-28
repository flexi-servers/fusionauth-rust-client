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

/// MessengerType : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessengerType {
    #[serde(rename = "Generic")]
    Generic,
    #[serde(rename = "Kafka")]
    Kafka,
    #[serde(rename = "Twilio")]
    Twilio,

}

impl std::fmt::Display for MessengerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Generic => write!(f, "Generic"),
            Self::Kafka => write!(f, "Kafka"),
            Self::Twilio => write!(f, "Twilio"),
        }
    }
}

impl Default for MessengerType {
    fn default() -> MessengerType {
        Self::Generic
    }
}

