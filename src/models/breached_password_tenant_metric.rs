/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// BreachedPasswordTenantMetric : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BreachedPasswordTenantMetric {
    #[serde(rename = "actionRequired", skip_serializing_if = "Option::is_none")]
    pub action_required: Option<i32>,
    #[serde(rename = "matchedCommonPasswordCount", skip_serializing_if = "Option::is_none")]
    pub matched_common_password_count: Option<i32>,
    #[serde(rename = "matchedExactCount", skip_serializing_if = "Option::is_none")]
    pub matched_exact_count: Option<i32>,
    #[serde(rename = "matchedPasswordCount", skip_serializing_if = "Option::is_none")]
    pub matched_password_count: Option<i32>,
    #[serde(rename = "matchedSubAddressCount", skip_serializing_if = "Option::is_none")]
    pub matched_sub_address_count: Option<i32>,
    #[serde(rename = "passwordsCheckedCount", skip_serializing_if = "Option::is_none")]
    pub passwords_checked_count: Option<i32>,
}

impl BreachedPasswordTenantMetric {
    /// 
    pub fn new() -> BreachedPasswordTenantMetric {
        BreachedPasswordTenantMetric {
            action_required: None,
            matched_common_password_count: None,
            matched_exact_count: None,
            matched_password_count: None,
            matched_sub_address_count: None,
            passwords_checked_count: None,
        }
    }
}

