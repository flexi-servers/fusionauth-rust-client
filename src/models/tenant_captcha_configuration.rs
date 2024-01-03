/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TenantCaptchaConfiguration : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantCaptchaConfiguration {
    #[serde(rename = "captchaMethod", skip_serializing_if = "Option::is_none")]
    pub captcha_method: Option<crate::models::CaptchaMethod>,
    #[serde(rename = "secretKey", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[serde(rename = "siteKey", skip_serializing_if = "Option::is_none")]
    pub site_key: Option<String>,
    #[serde(rename = "threshold", skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl TenantCaptchaConfiguration {
    /// 
    pub fn new() -> TenantCaptchaConfiguration {
        TenantCaptchaConfiguration {
            captcha_method: None,
            secret_key: None,
            site_key: None,
            threshold: None,
            enabled: None,
        }
    }
}


