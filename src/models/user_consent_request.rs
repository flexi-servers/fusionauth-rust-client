/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserConsentRequest : API response for User consent.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserConsentRequest {
    #[serde(rename = "userConsent", skip_serializing_if = "Option::is_none")]
    pub user_consent: Option<Box<crate::models::UserConsent>>,
}

impl UserConsentRequest {
    /// API response for User consent.
    pub fn new() -> UserConsentRequest {
        UserConsentRequest {
            user_consent: None,
        }
    }
}

