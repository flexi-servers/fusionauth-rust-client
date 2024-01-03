/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FamilyConfiguration : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FamilyConfiguration {
    #[serde(rename = "allowChildRegistrations", skip_serializing_if = "Option::is_none")]
    pub allow_child_registrations: Option<bool>,
    #[serde(rename = "confirmChildEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub confirm_child_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "deleteOrphanedAccounts", skip_serializing_if = "Option::is_none")]
    pub delete_orphaned_accounts: Option<bool>,
    #[serde(rename = "deleteOrphanedAccountsDays", skip_serializing_if = "Option::is_none")]
    pub delete_orphaned_accounts_days: Option<i32>,
    #[serde(rename = "familyRequestEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub family_request_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "maximumChildAge", skip_serializing_if = "Option::is_none")]
    pub maximum_child_age: Option<i32>,
    #[serde(rename = "minimumOwnerAge", skip_serializing_if = "Option::is_none")]
    pub minimum_owner_age: Option<i32>,
    #[serde(rename = "parentEmailRequired", skip_serializing_if = "Option::is_none")]
    pub parent_email_required: Option<bool>,
    #[serde(rename = "parentRegistrationEmailTemplateId", skip_serializing_if = "Option::is_none")]
    pub parent_registration_email_template_id: Option<uuid::Uuid>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl FamilyConfiguration {
    /// 
    pub fn new() -> FamilyConfiguration {
        FamilyConfiguration {
            allow_child_registrations: None,
            confirm_child_email_template_id: None,
            delete_orphaned_accounts: None,
            delete_orphaned_accounts_days: None,
            family_request_email_template_id: None,
            maximum_child_age: None,
            minimum_owner_age: None,
            parent_email_required: None,
            parent_registration_email_template_id: None,
            enabled: None,
        }
    }
}

