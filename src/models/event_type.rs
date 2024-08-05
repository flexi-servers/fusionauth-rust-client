/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.52.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EventType : Models the event types that FusionAuth produces.
/// Models the event types that FusionAuth produces.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "JWTPublicKeyUpdate")]
    JwtPublicKeyUpdate,
    #[serde(rename = "JWTRefreshTokenRevoke")]
    JwtRefreshTokenRevoke,
    #[serde(rename = "JWTRefresh")]
    JwtRefresh,
    #[serde(rename = "AuditLogCreate")]
    AuditLogCreate,
    #[serde(rename = "EventLogCreate")]
    EventLogCreate,
    #[serde(rename = "KickstartSuccess")]
    KickstartSuccess,
    #[serde(rename = "GroupCreate")]
    GroupCreate,
    #[serde(rename = "GroupCreateComplete")]
    GroupCreateComplete,
    #[serde(rename = "GroupDelete")]
    GroupDelete,
    #[serde(rename = "GroupDeleteComplete")]
    GroupDeleteComplete,
    #[serde(rename = "GroupMemberAdd")]
    GroupMemberAdd,
    #[serde(rename = "GroupMemberAddComplete")]
    GroupMemberAddComplete,
    #[serde(rename = "GroupMemberRemove")]
    GroupMemberRemove,
    #[serde(rename = "GroupMemberRemoveComplete")]
    GroupMemberRemoveComplete,
    #[serde(rename = "GroupMemberUpdate")]
    GroupMemberUpdate,
    #[serde(rename = "GroupMemberUpdateComplete")]
    GroupMemberUpdateComplete,
    #[serde(rename = "GroupUpdate")]
    GroupUpdate,
    #[serde(rename = "GroupUpdateComplete")]
    GroupUpdateComplete,
    #[serde(rename = "UserAction")]
    UserAction,
    #[serde(rename = "UserBulkCreate")]
    UserBulkCreate,
    #[serde(rename = "UserCreate")]
    UserCreate,
    #[serde(rename = "UserCreateComplete")]
    UserCreateComplete,
    #[serde(rename = "UserDeactivate")]
    UserDeactivate,
    #[serde(rename = "UserDelete")]
    UserDelete,
    #[serde(rename = "UserDeleteComplete")]
    UserDeleteComplete,
    #[serde(rename = "UserEmailUpdate")]
    UserEmailUpdate,
    #[serde(rename = "UserEmailVerified")]
    UserEmailVerified,
    #[serde(rename = "UserIdentityProviderLink")]
    UserIdentityProviderLink,
    #[serde(rename = "UserIdentityProviderUnlink")]
    UserIdentityProviderUnlink,
    #[serde(rename = "UserLoginIdDuplicateOnCreate")]
    UserLoginIdDuplicateOnCreate,
    #[serde(rename = "UserLoginIdDuplicateOnUpdate")]
    UserLoginIdDuplicateOnUpdate,
    #[serde(rename = "UserLoginFailed")]
    UserLoginFailed,
    #[serde(rename = "UserLoginNewDevice")]
    UserLoginNewDevice,
    #[serde(rename = "UserLoginSuccess")]
    UserLoginSuccess,
    #[serde(rename = "UserLoginSuspicious")]
    UserLoginSuspicious,
    #[serde(rename = "UserPasswordBreach")]
    UserPasswordBreach,
    #[serde(rename = "UserPasswordResetSend")]
    UserPasswordResetSend,
    #[serde(rename = "UserPasswordResetStart")]
    UserPasswordResetStart,
    #[serde(rename = "UserPasswordResetSuccess")]
    UserPasswordResetSuccess,
    #[serde(rename = "UserPasswordUpdate")]
    UserPasswordUpdate,
    #[serde(rename = "UserReactivate")]
    UserReactivate,
    #[serde(rename = "UserRegistrationCreate")]
    UserRegistrationCreate,
    #[serde(rename = "UserRegistrationCreateComplete")]
    UserRegistrationCreateComplete,
    #[serde(rename = "UserRegistrationDelete")]
    UserRegistrationDelete,
    #[serde(rename = "UserRegistrationDeleteComplete")]
    UserRegistrationDeleteComplete,
    #[serde(rename = "UserRegistrationUpdate")]
    UserRegistrationUpdate,
    #[serde(rename = "UserRegistrationUpdateComplete")]
    UserRegistrationUpdateComplete,
    #[serde(rename = "UserRegistrationVerified")]
    UserRegistrationVerified,
    #[serde(rename = "UserTwoFactorMethodAdd")]
    UserTwoFactorMethodAdd,
    #[serde(rename = "UserTwoFactorMethodRemove")]
    UserTwoFactorMethodRemove,
    #[serde(rename = "UserUpdate")]
    UserUpdate,
    #[serde(rename = "UserUpdateComplete")]
    UserUpdateComplete,
    #[serde(rename = "Test")]
    Test,

}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JwtPublicKeyUpdate => write!(f, "JWTPublicKeyUpdate"),
            Self::JwtRefreshTokenRevoke => write!(f, "JWTRefreshTokenRevoke"),
            Self::JwtRefresh => write!(f, "JWTRefresh"),
            Self::AuditLogCreate => write!(f, "AuditLogCreate"),
            Self::EventLogCreate => write!(f, "EventLogCreate"),
            Self::KickstartSuccess => write!(f, "KickstartSuccess"),
            Self::GroupCreate => write!(f, "GroupCreate"),
            Self::GroupCreateComplete => write!(f, "GroupCreateComplete"),
            Self::GroupDelete => write!(f, "GroupDelete"),
            Self::GroupDeleteComplete => write!(f, "GroupDeleteComplete"),
            Self::GroupMemberAdd => write!(f, "GroupMemberAdd"),
            Self::GroupMemberAddComplete => write!(f, "GroupMemberAddComplete"),
            Self::GroupMemberRemove => write!(f, "GroupMemberRemove"),
            Self::GroupMemberRemoveComplete => write!(f, "GroupMemberRemoveComplete"),
            Self::GroupMemberUpdate => write!(f, "GroupMemberUpdate"),
            Self::GroupMemberUpdateComplete => write!(f, "GroupMemberUpdateComplete"),
            Self::GroupUpdate => write!(f, "GroupUpdate"),
            Self::GroupUpdateComplete => write!(f, "GroupUpdateComplete"),
            Self::UserAction => write!(f, "UserAction"),
            Self::UserBulkCreate => write!(f, "UserBulkCreate"),
            Self::UserCreate => write!(f, "UserCreate"),
            Self::UserCreateComplete => write!(f, "UserCreateComplete"),
            Self::UserDeactivate => write!(f, "UserDeactivate"),
            Self::UserDelete => write!(f, "UserDelete"),
            Self::UserDeleteComplete => write!(f, "UserDeleteComplete"),
            Self::UserEmailUpdate => write!(f, "UserEmailUpdate"),
            Self::UserEmailVerified => write!(f, "UserEmailVerified"),
            Self::UserIdentityProviderLink => write!(f, "UserIdentityProviderLink"),
            Self::UserIdentityProviderUnlink => write!(f, "UserIdentityProviderUnlink"),
            Self::UserLoginIdDuplicateOnCreate => write!(f, "UserLoginIdDuplicateOnCreate"),
            Self::UserLoginIdDuplicateOnUpdate => write!(f, "UserLoginIdDuplicateOnUpdate"),
            Self::UserLoginFailed => write!(f, "UserLoginFailed"),
            Self::UserLoginNewDevice => write!(f, "UserLoginNewDevice"),
            Self::UserLoginSuccess => write!(f, "UserLoginSuccess"),
            Self::UserLoginSuspicious => write!(f, "UserLoginSuspicious"),
            Self::UserPasswordBreach => write!(f, "UserPasswordBreach"),
            Self::UserPasswordResetSend => write!(f, "UserPasswordResetSend"),
            Self::UserPasswordResetStart => write!(f, "UserPasswordResetStart"),
            Self::UserPasswordResetSuccess => write!(f, "UserPasswordResetSuccess"),
            Self::UserPasswordUpdate => write!(f, "UserPasswordUpdate"),
            Self::UserReactivate => write!(f, "UserReactivate"),
            Self::UserRegistrationCreate => write!(f, "UserRegistrationCreate"),
            Self::UserRegistrationCreateComplete => write!(f, "UserRegistrationCreateComplete"),
            Self::UserRegistrationDelete => write!(f, "UserRegistrationDelete"),
            Self::UserRegistrationDeleteComplete => write!(f, "UserRegistrationDeleteComplete"),
            Self::UserRegistrationUpdate => write!(f, "UserRegistrationUpdate"),
            Self::UserRegistrationUpdateComplete => write!(f, "UserRegistrationUpdateComplete"),
            Self::UserRegistrationVerified => write!(f, "UserRegistrationVerified"),
            Self::UserTwoFactorMethodAdd => write!(f, "UserTwoFactorMethodAdd"),
            Self::UserTwoFactorMethodRemove => write!(f, "UserTwoFactorMethodRemove"),
            Self::UserUpdate => write!(f, "UserUpdate"),
            Self::UserUpdateComplete => write!(f, "UserUpdateComplete"),
            Self::Test => write!(f, "Test"),
        }
    }
}

impl Default for EventType {
    fn default() -> EventType {
        Self::JwtPublicKeyUpdate
    }
}

