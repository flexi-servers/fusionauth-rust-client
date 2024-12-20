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

/// TenantWebAuthnConfiguration : Tenant-level configuration for WebAuthn
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantWebAuthnConfiguration {
    #[serde(rename = "bootstrapWorkflow", skip_serializing_if = "Option::is_none")]
    pub bootstrap_workflow: Option<Box<models::TenantWebAuthnWorkflowConfiguration>>,
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(rename = "reauthenticationWorkflow", skip_serializing_if = "Option::is_none")]
    pub reauthentication_workflow: Option<Box<models::TenantWebAuthnWorkflowConfiguration>>,
    #[serde(rename = "relyingPartyId", skip_serializing_if = "Option::is_none")]
    pub relying_party_id: Option<String>,
    #[serde(rename = "relyingPartyName", skip_serializing_if = "Option::is_none")]
    pub relying_party_name: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl TenantWebAuthnConfiguration {
    /// Tenant-level configuration for WebAuthn
    pub fn new() -> TenantWebAuthnConfiguration {
        TenantWebAuthnConfiguration {
            bootstrap_workflow: None,
            debug: None,
            reauthentication_workflow: None,
            relying_party_id: None,
            relying_party_name: None,
            enabled: None,
        }
    }
}

