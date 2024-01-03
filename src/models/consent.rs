/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Consent : Models a consent.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Consent {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "consentEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub consent_email_template_id: Option<uuid::Uuid>,
    /// Models a set of localized Integers that can be stored as JSON.
    #[serde(rename = "countryMinimumAgeForSelfConsent", skip_serializing_if = "Option::is_none")]
    pub country_minimum_age_for_self_consent: Option<serde_json::Value>,
    #[serde(rename = "defaultMinimumAgeForSelfConsent", skip_serializing_if = "Option::is_none")]
    pub default_minimum_age_for_self_consent: Option<i32>,
    #[serde(rename = "emailPlus", skip_serializing_if = "Option::is_none")]
    pub email_plus: Option<Box<crate::models::EmailPlus>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "insertInstant", skip_serializing_if = "Option::is_none")]
    pub insert_instant: Option<i64>,
    /// The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC.
    #[serde(rename = "lastUpdateInstant", skip_serializing_if = "Option::is_none")]
    pub last_update_instant: Option<i64>,
    #[serde(rename = "multipleValuesAllowed", skip_serializing_if = "Option::is_none")]
    pub multiple_values_allowed: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl Consent {
    /// Models a consent.
    pub fn new() -> Consent {
        Consent {
            data: None,
            consent_email_template_id: None,
            country_minimum_age_for_self_consent: None,
            default_minimum_age_for_self_consent: None,
            email_plus: None,
            id: None,
            insert_instant: None,
            last_update_instant: None,
            multiple_values_allowed: None,
            name: None,
            values: None,
        }
    }
}

