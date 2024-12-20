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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OAuthErrorType {
    #[serde(rename = "invalid_request")]
    InvalidRequest,
    #[serde(rename = "invalid_client")]
    InvalidClient,
    #[serde(rename = "invalid_grant")]
    InvalidGrant,
    #[serde(rename = "invalid_token")]
    InvalidToken,
    #[serde(rename = "unauthorized_client")]
    UnauthorizedClient,
    #[serde(rename = "invalid_scope")]
    InvalidScope,
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "unsupported_grant_type")]
    UnsupportedGrantType,
    #[serde(rename = "unsupported_response_type")]
    UnsupportedResponseType,
    #[serde(rename = "access_denied")]
    AccessDenied,
    #[serde(rename = "change_password_required")]
    ChangePasswordRequired,
    #[serde(rename = "not_licensed")]
    NotLicensed,
    #[serde(rename = "two_factor_required")]
    TwoFactorRequired,
    #[serde(rename = "authorization_pending")]
    AuthorizationPending,
    #[serde(rename = "expired_token")]
    ExpiredToken,
    #[serde(rename = "unsupported_token_type")]
    UnsupportedTokenType,

}

impl std::fmt::Display for OAuthErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidRequest => write!(f, "invalid_request"),
            Self::InvalidClient => write!(f, "invalid_client"),
            Self::InvalidGrant => write!(f, "invalid_grant"),
            Self::InvalidToken => write!(f, "invalid_token"),
            Self::UnauthorizedClient => write!(f, "unauthorized_client"),
            Self::InvalidScope => write!(f, "invalid_scope"),
            Self::ServerError => write!(f, "server_error"),
            Self::UnsupportedGrantType => write!(f, "unsupported_grant_type"),
            Self::UnsupportedResponseType => write!(f, "unsupported_response_type"),
            Self::AccessDenied => write!(f, "access_denied"),
            Self::ChangePasswordRequired => write!(f, "change_password_required"),
            Self::NotLicensed => write!(f, "not_licensed"),
            Self::TwoFactorRequired => write!(f, "two_factor_required"),
            Self::AuthorizationPending => write!(f, "authorization_pending"),
            Self::ExpiredToken => write!(f, "expired_token"),
            Self::UnsupportedTokenType => write!(f, "unsupported_token_type"),
        }
    }
}

impl Default for OAuthErrorType {
    fn default() -> OAuthErrorType {
        Self::InvalidRequest
    }
}

