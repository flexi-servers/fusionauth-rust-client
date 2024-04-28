/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.50.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateInformation {
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "md5Fingerprint", skip_serializing_if = "Option::is_none")]
    pub md5_fingerprint: Option<String>,
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "sha1Fingerprint", skip_serializing_if = "Option::is_none")]
    pub sha1_fingerprint: Option<String>,
    #[serde(rename = "sha1Thumbprint", skip_serializing_if = "Option::is_none")]
    pub sha1_thumbprint: Option<String>,
    #[serde(rename = "sha256Fingerprint", skip_serializing_if = "Option::is_none")]
    pub sha256_fingerprint: Option<String>,
    #[serde(rename = "sha256Thumbprint", skip_serializing_if = "Option::is_none")]
    pub sha256_thumbprint: Option<String>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "validTo", skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<i64>,
}

impl CertificateInformation {
    pub fn new() -> CertificateInformation {
        CertificateInformation {
            issuer: None,
            md5_fingerprint: None,
            serial_number: None,
            sha1_fingerprint: None,
            sha1_thumbprint: None,
            sha256_fingerprint: None,
            sha256_thumbprint: None,
            subject: None,
            valid_from: None,
            valid_to: None,
        }
    }
}

