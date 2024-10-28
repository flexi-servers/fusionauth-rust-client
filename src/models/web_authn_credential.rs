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

/// WebAuthnCredential : A User's WebAuthnCredential. Contains all data required to complete WebAuthn authentication ceremonies.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAuthnCredential {
    #[serde(rename = "algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<models::CoseAlgorithmIdentifier>,
    #[serde(rename = "attestationType", skip_serializing_if = "Option::is_none")]
    pub attestation_type: Option<models::AttestationType>,
    #[serde(rename = "authenticatorSupportsUserVerification", skip_serializing_if = "Option::is_none")]
    pub authenticator_supports_user_verification: Option<bool>,
    #[serde(rename = "credentialId", skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "discoverable", skip_serializing_if = "Option::is_none")]
    pub discoverable: Option<bool>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUseInstant", skip_serializing_if = "Option::is_none")]
    pub last_use_instant: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "relyingPartyId", skip_serializing_if = "Option::is_none")]
    pub relying_party_id: Option<String>,
    #[serde(rename = "signCount", skip_serializing_if = "Option::is_none")]
    pub sign_count: Option<i32>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<uuid::Uuid>,
    #[serde(rename = "transports", skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<String>>,
    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl WebAuthnCredential {
    /// A User's WebAuthnCredential. Contains all data required to complete WebAuthn authentication ceremonies.
    pub fn new() -> WebAuthnCredential {
        WebAuthnCredential {
            algorithm: None,
            attestation_type: None,
            authenticator_supports_user_verification: None,
            credential_id: None,
            data: None,
            discoverable: None,
            display_name: None,
            id: None,
            insert_instant: None,
            last_use_instant: None,
            name: None,
            public_key: None,
            relying_party_id: None,
            sign_count: None,
            tenant_id: None,
            transports: None,
            user_agent: None,
            user_id: None,
        }
    }
}

