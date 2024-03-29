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

/// RateLimitedRequestType : 
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RateLimitedRequestType {
    #[serde(rename = "FailedLogin")]
    FailedLogin,
    #[serde(rename = "ForgotPassword")]
    ForgotPassword,
    #[serde(rename = "SendEmailVerification")]
    SendEmailVerification,
    #[serde(rename = "SendPasswordless")]
    SendPasswordless,
    #[serde(rename = "SendRegistrationVerification")]
    SendRegistrationVerification,
    #[serde(rename = "SendTwoFactor")]
    SendTwoFactor,

}

impl ToString for RateLimitedRequestType {
    fn to_string(&self) -> String {
        match self {
            Self::FailedLogin => String::from("FailedLogin"),
            Self::ForgotPassword => String::from("ForgotPassword"),
            Self::SendEmailVerification => String::from("SendEmailVerification"),
            Self::SendPasswordless => String::from("SendPasswordless"),
            Self::SendRegistrationVerification => String::from("SendRegistrationVerification"),
            Self::SendTwoFactor => String::from("SendTwoFactor"),
        }
    }
}

impl Default for RateLimitedRequestType {
    fn default() -> RateLimitedRequestType {
        Self::FailedLogin
    }
}

