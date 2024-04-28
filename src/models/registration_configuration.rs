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
pub struct RegistrationConfiguration {
    #[serde(rename = "birthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<Box<models::Requirable>>,
    #[serde(rename = "confirmPassword", skip_serializing_if = "Option::is_none")]
    pub confirm_password: Option<bool>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Box<models::Requirable>>,
    #[serde(rename = "formId", skip_serializing_if = "Option::is_none")]
    pub form_id: Option<uuid::Uuid>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<Box<models::Requirable>>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<models::Requirable>>,
    #[serde(rename = "loginIdType", skip_serializing_if = "Option::is_none")]
    pub login_id_type: Option<models::LoginIdType>,
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<Box<models::Requirable>>,
    #[serde(rename = "mobilePhone", skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<Box<models::Requirable>>,
    #[serde(rename = "preferredLanguages", skip_serializing_if = "Option::is_none")]
    pub preferred_languages: Option<Box<models::Requirable>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::RegistrationType>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl RegistrationConfiguration {
    pub fn new() -> RegistrationConfiguration {
        RegistrationConfiguration {
            birth_date: None,
            confirm_password: None,
            first_name: None,
            form_id: None,
            full_name: None,
            last_name: None,
            login_id_type: None,
            middle_name: None,
            mobile_phone: None,
            preferred_languages: None,
            r#type: None,
            enabled: None,
        }
    }
}

