/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ExternalIdentifierConfiguration : 



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalIdentifierConfiguration {
    #[serde(rename = "authorizationGrantIdTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub authorization_grant_id_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "changePasswordIdGenerator", skip_serializing_if = "Option::is_none")]
    pub change_password_id_generator: Option<Box<crate::models::SecureGeneratorConfiguration>>,
    #[serde(rename = "changePasswordIdTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub change_password_id_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "deviceCodeTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub device_code_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "deviceUserCodeIdGenerator", skip_serializing_if = "Option::is_none")]
    pub device_user_code_id_generator: Option<Box<crate::models::SecureGeneratorConfiguration>>,
    #[serde(rename = "emailVerificationIdGenerator", skip_serializing_if = "Option::is_none")]
    pub email_verification_id_generator: Option<Box<crate::models::SecureGeneratorConfiguration>>,
    #[serde(rename = "emailVerificationIdTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub email_verification_id_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "emailVerificationOneTimeCodeGenerator", skip_serializing_if = "Option::is_none")]
    pub email_verification_one_time_code_generator: Option<Box<crate::models::SecureGeneratorConfiguration>>,
    #[serde(rename = "externalAuthenticationIdTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub external_authentication_id_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "oneTimePasswordTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub one_time_password_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "passwordlessLoginGenerator", skip_serializing_if = "Option::is_none")]
    pub passwordless_login_generator: Option<Box<crate::models::SecureGeneratorConfiguration>>,
    #[serde(rename = "passwordlessLoginTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub passwordless_login_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "pendingAccountLinkTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub pending_account_link_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "registrationVerificationIdGenerator", skip_serializing_if = "Option::is_none")]
    pub registration_verification_id_generator: Option<Box<crate::models::SecureGeneratorConfiguration>>,
    #[serde(rename = "registrationVerificationIdTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub registration_verification_id_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "registrationVerificationOneTimeCodeGenerator", skip_serializing_if = "Option::is_none")]
    pub registration_verification_one_time_code_generator: Option<Box<crate::models::SecureGeneratorConfiguration>>,
    #[serde(rename = "samlv2AuthNRequestIdTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub samlv2_auth_n_request_id_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "setupPasswordIdGenerator", skip_serializing_if = "Option::is_none")]
    pub setup_password_id_generator: Option<Box<crate::models::SecureGeneratorConfiguration>>,
    #[serde(rename = "setupPasswordIdTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub setup_password_id_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "trustTokenTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub trust_token_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "twoFactorIdTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub two_factor_id_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "twoFactorOneTimeCodeIdGenerator", skip_serializing_if = "Option::is_none")]
    pub two_factor_one_time_code_id_generator: Option<Box<crate::models::SecureGeneratorConfiguration>>,
    #[serde(rename = "twoFactorOneTimeCodeIdTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub two_factor_one_time_code_id_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "twoFactorTrustIdTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub two_factor_trust_id_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "webAuthnAuthenticationChallengeTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub web_authn_authentication_challenge_time_to_live_in_seconds: Option<i32>,
    #[serde(rename = "webAuthnRegistrationChallengeTimeToLiveInSeconds", skip_serializing_if = "Option::is_none")]
    pub web_authn_registration_challenge_time_to_live_in_seconds: Option<i32>,
}

impl ExternalIdentifierConfiguration {
    /// 
    pub fn new() -> ExternalIdentifierConfiguration {
        ExternalIdentifierConfiguration {
            authorization_grant_id_time_to_live_in_seconds: None,
            change_password_id_generator: None,
            change_password_id_time_to_live_in_seconds: None,
            device_code_time_to_live_in_seconds: None,
            device_user_code_id_generator: None,
            email_verification_id_generator: None,
            email_verification_id_time_to_live_in_seconds: None,
            email_verification_one_time_code_generator: None,
            external_authentication_id_time_to_live_in_seconds: None,
            one_time_password_time_to_live_in_seconds: None,
            passwordless_login_generator: None,
            passwordless_login_time_to_live_in_seconds: None,
            pending_account_link_time_to_live_in_seconds: None,
            registration_verification_id_generator: None,
            registration_verification_id_time_to_live_in_seconds: None,
            registration_verification_one_time_code_generator: None,
            samlv2_auth_n_request_id_time_to_live_in_seconds: None,
            setup_password_id_generator: None,
            setup_password_id_time_to_live_in_seconds: None,
            trust_token_time_to_live_in_seconds: None,
            two_factor_id_time_to_live_in_seconds: None,
            two_factor_one_time_code_id_generator: None,
            two_factor_one_time_code_id_time_to_live_in_seconds: None,
            two_factor_trust_id_time_to_live_in_seconds: None,
            web_authn_authentication_challenge_time_to_live_in_seconds: None,
            web_authn_registration_challenge_time_to_live_in_seconds: None,
        }
    }
}


