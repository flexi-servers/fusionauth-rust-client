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

impl ToString for EventType {
    fn to_string(&self) -> String {
        match self {
            Self::JwtPublicKeyUpdate => String::from("JWTPublicKeyUpdate"),
            Self::JwtRefreshTokenRevoke => String::from("JWTRefreshTokenRevoke"),
            Self::JwtRefresh => String::from("JWTRefresh"),
            Self::AuditLogCreate => String::from("AuditLogCreate"),
            Self::EventLogCreate => String::from("EventLogCreate"),
            Self::KickstartSuccess => String::from("KickstartSuccess"),
            Self::GroupCreate => String::from("GroupCreate"),
            Self::GroupCreateComplete => String::from("GroupCreateComplete"),
            Self::GroupDelete => String::from("GroupDelete"),
            Self::GroupDeleteComplete => String::from("GroupDeleteComplete"),
            Self::GroupMemberAdd => String::from("GroupMemberAdd"),
            Self::GroupMemberAddComplete => String::from("GroupMemberAddComplete"),
            Self::GroupMemberRemove => String::from("GroupMemberRemove"),
            Self::GroupMemberRemoveComplete => String::from("GroupMemberRemoveComplete"),
            Self::GroupMemberUpdate => String::from("GroupMemberUpdate"),
            Self::GroupMemberUpdateComplete => String::from("GroupMemberUpdateComplete"),
            Self::GroupUpdate => String::from("GroupUpdate"),
            Self::GroupUpdateComplete => String::from("GroupUpdateComplete"),
            Self::UserAction => String::from("UserAction"),
            Self::UserBulkCreate => String::from("UserBulkCreate"),
            Self::UserCreate => String::from("UserCreate"),
            Self::UserCreateComplete => String::from("UserCreateComplete"),
            Self::UserDeactivate => String::from("UserDeactivate"),
            Self::UserDelete => String::from("UserDelete"),
            Self::UserDeleteComplete => String::from("UserDeleteComplete"),
            Self::UserEmailUpdate => String::from("UserEmailUpdate"),
            Self::UserEmailVerified => String::from("UserEmailVerified"),
            Self::UserIdentityProviderLink => String::from("UserIdentityProviderLink"),
            Self::UserIdentityProviderUnlink => String::from("UserIdentityProviderUnlink"),
            Self::UserLoginIdDuplicateOnCreate => String::from("UserLoginIdDuplicateOnCreate"),
            Self::UserLoginIdDuplicateOnUpdate => String::from("UserLoginIdDuplicateOnUpdate"),
            Self::UserLoginFailed => String::from("UserLoginFailed"),
            Self::UserLoginNewDevice => String::from("UserLoginNewDevice"),
            Self::UserLoginSuccess => String::from("UserLoginSuccess"),
            Self::UserLoginSuspicious => String::from("UserLoginSuspicious"),
            Self::UserPasswordBreach => String::from("UserPasswordBreach"),
            Self::UserPasswordResetSend => String::from("UserPasswordResetSend"),
            Self::UserPasswordResetStart => String::from("UserPasswordResetStart"),
            Self::UserPasswordResetSuccess => String::from("UserPasswordResetSuccess"),
            Self::UserPasswordUpdate => String::from("UserPasswordUpdate"),
            Self::UserReactivate => String::from("UserReactivate"),
            Self::UserRegistrationCreate => String::from("UserRegistrationCreate"),
            Self::UserRegistrationCreateComplete => String::from("UserRegistrationCreateComplete"),
            Self::UserRegistrationDelete => String::from("UserRegistrationDelete"),
            Self::UserRegistrationDeleteComplete => String::from("UserRegistrationDeleteComplete"),
            Self::UserRegistrationUpdate => String::from("UserRegistrationUpdate"),
            Self::UserRegistrationUpdateComplete => String::from("UserRegistrationUpdateComplete"),
            Self::UserRegistrationVerified => String::from("UserRegistrationVerified"),
            Self::UserTwoFactorMethodAdd => String::from("UserTwoFactorMethodAdd"),
            Self::UserTwoFactorMethodRemove => String::from("UserTwoFactorMethodRemove"),
            Self::UserUpdate => String::from("UserUpdate"),
            Self::UserUpdateComplete => String::from("UserUpdateComplete"),
            Self::Test => String::from("Test"),
        }
    }
}

impl Default for EventType {
    fn default() -> EventType {
        Self::JwtPublicKeyUpdate
    }
}

