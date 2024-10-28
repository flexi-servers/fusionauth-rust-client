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

/// LogHistory : A historical state of a user log event. Since events can be modified, this stores the historical state.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogHistory {
    #[serde(rename = "historyItems", skip_serializing_if = "Option::is_none")]
    pub history_items: Option<Vec<models::HistoryItem>>,
}

impl LogHistory {
    /// A historical state of a user log event. Since events can be modified, this stores the historical state.
    pub fn new() -> LogHistory {
        LogHistory {
            history_items: None,
        }
    }
}

