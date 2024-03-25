/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OAuthErrorReason {
    #[serde(rename = "auth_code_not_found")]
    AuthCodeNotFound,
    #[serde(rename = "access_token_malformed")]
    AccessTokenMalformed,
    #[serde(rename = "access_token_expired")]
    AccessTokenExpired,
    #[serde(rename = "access_token_unavailable_for_processing")]
    AccessTokenUnavailableForProcessing,
    #[serde(rename = "access_token_failed_processing")]
    AccessTokenFailedProcessing,
    #[serde(rename = "access_token_invalid")]
    AccessTokenInvalid,
    #[serde(rename = "refresh_token_not_found")]
    RefreshTokenNotFound,
    #[serde(rename = "refresh_token_type_not_supported")]
    RefreshTokenTypeNotSupported,
    #[serde(rename = "invalid_client_id")]
    InvalidClientId,
    #[serde(rename = "invalid_user_credentials")]
    InvalidUserCredentials,
    #[serde(rename = "invalid_grant_type")]
    InvalidGrantType,
    #[serde(rename = "invalid_origin")]
    InvalidOrigin,
    #[serde(rename = "invalid_origin_opaque")]
    InvalidOriginOpaque,
    #[serde(rename = "invalid_pkce_code_verifier")]
    InvalidPkceCodeVerifier,
    #[serde(rename = "invalid_pkce_code_challenge")]
    InvalidPkceCodeChallenge,
    #[serde(rename = "invalid_pkce_code_challenge_method")]
    InvalidPkceCodeChallengeMethod,
    #[serde(rename = "invalid_redirect_uri")]
    InvalidRedirectUri,
    #[serde(rename = "invalid_response_mode")]
    InvalidResponseMode,
    #[serde(rename = "invalid_response_type")]
    InvalidResponseType,
    #[serde(rename = "invalid_id_token_hint")]
    InvalidIdTokenHint,
    #[serde(rename = "invalid_post_logout_redirect_uri")]
    InvalidPostLogoutRedirectUri,
    #[serde(rename = "invalid_device_code")]
    InvalidDeviceCode,
    #[serde(rename = "invalid_user_code")]
    InvalidUserCode,
    #[serde(rename = "invalid_additional_client_id")]
    InvalidAdditionalClientId,
    #[serde(rename = "invalid_target_entity_scope")]
    InvalidTargetEntityScope,
    #[serde(rename = "invalid_entity_permission_scope")]
    InvalidEntityPermissionScope,
    #[serde(rename = "invalid_user_id")]
    InvalidUserId,
    #[serde(rename = "grant_type_disabled")]
    GrantTypeDisabled,
    #[serde(rename = "missing_client_id")]
    MissingClientId,
    #[serde(rename = "missing_client_secret")]
    MissingClientSecret,
    #[serde(rename = "missing_code")]
    MissingCode,
    #[serde(rename = "missing_code_challenge")]
    MissingCodeChallenge,
    #[serde(rename = "missing_code_verifier")]
    MissingCodeVerifier,
    #[serde(rename = "missing_device_code")]
    MissingDeviceCode,
    #[serde(rename = "missing_grant_type")]
    MissingGrantType,
    #[serde(rename = "missing_redirect_uri")]
    MissingRedirectUri,
    #[serde(rename = "missing_refresh_token")]
    MissingRefreshToken,
    #[serde(rename = "missing_response_type")]
    MissingResponseType,
    #[serde(rename = "missing_token")]
    MissingToken,
    #[serde(rename = "missing_user_code")]
    MissingUserCode,
    #[serde(rename = "missing_user_id")]
    MissingUserId,
    #[serde(rename = "missing_verification_uri")]
    MissingVerificationUri,
    #[serde(rename = "login_prevented")]
    LoginPrevented,
    #[serde(rename = "not_licensed")]
    NotLicensed,
    #[serde(rename = "user_code_expired")]
    UserCodeExpired,
    #[serde(rename = "user_expired")]
    UserExpired,
    #[serde(rename = "user_locked")]
    UserLocked,
    #[serde(rename = "user_not_found")]
    UserNotFound,
    #[serde(rename = "client_authentication_missing")]
    ClientAuthenticationMissing,
    #[serde(rename = "invalid_client_authentication_scheme")]
    InvalidClientAuthenticationScheme,
    #[serde(rename = "invalid_client_authentication")]
    InvalidClientAuthentication,
    #[serde(rename = "client_id_mismatch")]
    ClientIdMismatch,
    #[serde(rename = "change_password_administrative")]
    ChangePasswordAdministrative,
    #[serde(rename = "change_password_breached")]
    ChangePasswordBreached,
    #[serde(rename = "change_password_expired")]
    ChangePasswordExpired,
    #[serde(rename = "change_password_validation")]
    ChangePasswordValidation,
    #[serde(rename = "unknown")]
    Unknown,

}

impl ToString for OAuthErrorReason {
    fn to_string(&self) -> String {
        match self {
            Self::AuthCodeNotFound => String::from("auth_code_not_found"),
            Self::AccessTokenMalformed => String::from("access_token_malformed"),
            Self::AccessTokenExpired => String::from("access_token_expired"),
            Self::AccessTokenUnavailableForProcessing => String::from("access_token_unavailable_for_processing"),
            Self::AccessTokenFailedProcessing => String::from("access_token_failed_processing"),
            Self::AccessTokenInvalid => String::from("access_token_invalid"),
            Self::RefreshTokenNotFound => String::from("refresh_token_not_found"),
            Self::RefreshTokenTypeNotSupported => String::from("refresh_token_type_not_supported"),
            Self::InvalidClientId => String::from("invalid_client_id"),
            Self::InvalidUserCredentials => String::from("invalid_user_credentials"),
            Self::InvalidGrantType => String::from("invalid_grant_type"),
            Self::InvalidOrigin => String::from("invalid_origin"),
            Self::InvalidOriginOpaque => String::from("invalid_origin_opaque"),
            Self::InvalidPkceCodeVerifier => String::from("invalid_pkce_code_verifier"),
            Self::InvalidPkceCodeChallenge => String::from("invalid_pkce_code_challenge"),
            Self::InvalidPkceCodeChallengeMethod => String::from("invalid_pkce_code_challenge_method"),
            Self::InvalidRedirectUri => String::from("invalid_redirect_uri"),
            Self::InvalidResponseMode => String::from("invalid_response_mode"),
            Self::InvalidResponseType => String::from("invalid_response_type"),
            Self::InvalidIdTokenHint => String::from("invalid_id_token_hint"),
            Self::InvalidPostLogoutRedirectUri => String::from("invalid_post_logout_redirect_uri"),
            Self::InvalidDeviceCode => String::from("invalid_device_code"),
            Self::InvalidUserCode => String::from("invalid_user_code"),
            Self::InvalidAdditionalClientId => String::from("invalid_additional_client_id"),
            Self::InvalidTargetEntityScope => String::from("invalid_target_entity_scope"),
            Self::InvalidEntityPermissionScope => String::from("invalid_entity_permission_scope"),
            Self::InvalidUserId => String::from("invalid_user_id"),
            Self::GrantTypeDisabled => String::from("grant_type_disabled"),
            Self::MissingClientId => String::from("missing_client_id"),
            Self::MissingClientSecret => String::from("missing_client_secret"),
            Self::MissingCode => String::from("missing_code"),
            Self::MissingCodeChallenge => String::from("missing_code_challenge"),
            Self::MissingCodeVerifier => String::from("missing_code_verifier"),
            Self::MissingDeviceCode => String::from("missing_device_code"),
            Self::MissingGrantType => String::from("missing_grant_type"),
            Self::MissingRedirectUri => String::from("missing_redirect_uri"),
            Self::MissingRefreshToken => String::from("missing_refresh_token"),
            Self::MissingResponseType => String::from("missing_response_type"),
            Self::MissingToken => String::from("missing_token"),
            Self::MissingUserCode => String::from("missing_user_code"),
            Self::MissingUserId => String::from("missing_user_id"),
            Self::MissingVerificationUri => String::from("missing_verification_uri"),
            Self::LoginPrevented => String::from("login_prevented"),
            Self::NotLicensed => String::from("not_licensed"),
            Self::UserCodeExpired => String::from("user_code_expired"),
            Self::UserExpired => String::from("user_expired"),
            Self::UserLocked => String::from("user_locked"),
            Self::UserNotFound => String::from("user_not_found"),
            Self::ClientAuthenticationMissing => String::from("client_authentication_missing"),
            Self::InvalidClientAuthenticationScheme => String::from("invalid_client_authentication_scheme"),
            Self::InvalidClientAuthentication => String::from("invalid_client_authentication"),
            Self::ClientIdMismatch => String::from("client_id_mismatch"),
            Self::ChangePasswordAdministrative => String::from("change_password_administrative"),
            Self::ChangePasswordBreached => String::from("change_password_breached"),
            Self::ChangePasswordExpired => String::from("change_password_expired"),
            Self::ChangePasswordValidation => String::from("change_password_validation"),
            Self::Unknown => String::from("unknown"),
        }
    }
}

impl Default for OAuthErrorReason {
    fn default() -> OAuthErrorReason {
        Self::AuthCodeNotFound
    }
}

