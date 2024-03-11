/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// TenantWebAuthnWorkflowConfiguration : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantWebAuthnWorkflowConfiguration {
    #[serde(rename = "authenticatorAttachmentPreference", skip_serializing_if = "Option::is_none")]
    pub authenticator_attachment_preference: Option<models::AuthenticatorAttachmentPreference>,
    #[serde(rename = "userVerificationRequirement", skip_serializing_if = "Option::is_none")]
    pub user_verification_requirement: Option<models::UserVerificationRequirement>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl TenantWebAuthnWorkflowConfiguration {
    /// 
    pub fn new() -> TenantWebAuthnWorkflowConfiguration {
        TenantWebAuthnWorkflowConfiguration {
            authenticator_attachment_preference: None,
            user_verification_requirement: None,
            enabled: None,
        }
    }
}

