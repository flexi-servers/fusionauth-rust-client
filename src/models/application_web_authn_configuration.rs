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

/// ApplicationWebAuthnConfiguration : Application-level configuration for WebAuthn
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationWebAuthnConfiguration {
    #[serde(rename = "bootstrapWorkflow", skip_serializing_if = "Option::is_none")]
    pub bootstrap_workflow: Option<Box<models::ApplicationWebAuthnWorkflowConfiguration>>,
    #[serde(rename = "reauthenticationWorkflow", skip_serializing_if = "Option::is_none")]
    pub reauthentication_workflow: Option<Box<models::ApplicationWebAuthnWorkflowConfiguration>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl ApplicationWebAuthnConfiguration {
    /// Application-level configuration for WebAuthn
    pub fn new() -> ApplicationWebAuthnConfiguration {
        ApplicationWebAuthnConfiguration {
            bootstrap_workflow: None,
            reauthentication_workflow: None,
            enabled: None,
        }
    }
}

