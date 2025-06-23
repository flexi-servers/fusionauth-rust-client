# \DefaultApi

All URIs are relative to *http://localhost:9011*

Method | HTTP request | Description
------------- | ------------- | -------------
[**action_user_with_id**](DefaultApi.md#action_user_with_id) | **POST** /api/user/action | 
[**activate_reactor_with_id**](DefaultApi.md#activate_reactor_with_id) | **POST** /api/reactor | 
[**approve_device_with_id**](DefaultApi.md#approve_device_with_id) | **POST** /oauth2/device/approve | 
[**cancel_action_with_id**](DefaultApi.md#cancel_action_with_id) | **DELETE** /api/user/action/{actionId} | 
[**change_password_by_identity_with_id**](DefaultApi.md#change_password_by_identity_with_id) | **POST** /api/user/change-password | 
[**change_password_with_id**](DefaultApi.md#change_password_with_id) | **POST** /api/user/change-password/{changePasswordId} | 
[**check_change_password_using_id_with_id**](DefaultApi.md#check_change_password_using_id_with_id) | **GET** /api/user/change-password/{changePasswordId} | 
[**comment_on_user_with_id**](DefaultApi.md#comment_on_user_with_id) | **POST** /api/user/comment | 
[**complete_web_authn_assertion_with_id**](DefaultApi.md#complete_web_authn_assertion_with_id) | **POST** /api/webauthn/assert | 
[**complete_web_authn_login_with_id**](DefaultApi.md#complete_web_authn_login_with_id) | **POST** /api/webauthn/login | 
[**complete_web_authn_registration_with_id**](DefaultApi.md#complete_web_authn_registration_with_id) | **POST** /api/webauthn/register/complete | 
[**create_api_key**](DefaultApi.md#create_api_key) | **POST** /api/api-key | 
[**create_api_key_with_id**](DefaultApi.md#create_api_key_with_id) | **POST** /api/api-key/{keyId} | 
[**create_application**](DefaultApi.md#create_application) | **POST** /api/application | 
[**create_application_role**](DefaultApi.md#create_application_role) | **POST** /api/application/{applicationId}/role | 
[**create_application_role_with_id**](DefaultApi.md#create_application_role_with_id) | **POST** /api/application/{applicationId}/role/{roleId} | 
[**create_application_with_id**](DefaultApi.md#create_application_with_id) | **POST** /api/application/{applicationId} | 
[**create_audit_log_with_id**](DefaultApi.md#create_audit_log_with_id) | **POST** /api/system/audit-log | 
[**create_connector**](DefaultApi.md#create_connector) | **POST** /api/connector | 
[**create_connector_with_id**](DefaultApi.md#create_connector_with_id) | **POST** /api/connector/{connectorId} | 
[**create_consent**](DefaultApi.md#create_consent) | **POST** /api/consent | 
[**create_consent_with_id**](DefaultApi.md#create_consent_with_id) | **POST** /api/consent/{consentId} | 
[**create_email_template**](DefaultApi.md#create_email_template) | **POST** /api/email/template | 
[**create_email_template_with_id**](DefaultApi.md#create_email_template_with_id) | **POST** /api/email/template/{emailTemplateId} | 
[**create_entity**](DefaultApi.md#create_entity) | **POST** /api/entity | 
[**create_entity_type**](DefaultApi.md#create_entity_type) | **POST** /api/entity/type | 
[**create_entity_type_permission**](DefaultApi.md#create_entity_type_permission) | **POST** /api/entity/type/{entityTypeId}/permission | 
[**create_entity_type_permission_with_id**](DefaultApi.md#create_entity_type_permission_with_id) | **POST** /api/entity/type/{entityTypeId}/permission/{permissionId} | 
[**create_entity_type_with_id**](DefaultApi.md#create_entity_type_with_id) | **POST** /api/entity/type/{entityTypeId} | 
[**create_entity_with_id**](DefaultApi.md#create_entity_with_id) | **POST** /api/entity/{entityId} | 
[**create_family**](DefaultApi.md#create_family) | **POST** /api/user/family | 
[**create_family_with_id**](DefaultApi.md#create_family_with_id) | **POST** /api/user/family/{familyId} | 
[**create_form**](DefaultApi.md#create_form) | **POST** /api/form | 
[**create_form_field**](DefaultApi.md#create_form_field) | **POST** /api/form/field | 
[**create_form_field_with_id**](DefaultApi.md#create_form_field_with_id) | **POST** /api/form/field/{fieldId} | 
[**create_form_with_id**](DefaultApi.md#create_form_with_id) | **POST** /api/form/{formId} | 
[**create_group**](DefaultApi.md#create_group) | **POST** /api/group | 
[**create_group_members_with_id**](DefaultApi.md#create_group_members_with_id) | **POST** /api/group/member | 
[**create_group_with_id**](DefaultApi.md#create_group_with_id) | **POST** /api/group/{groupId} | 
[**create_identity_provider**](DefaultApi.md#create_identity_provider) | **POST** /api/identity-provider | 
[**create_identity_provider_with_id**](DefaultApi.md#create_identity_provider_with_id) | **POST** /api/identity-provider/{identityProviderId} | 
[**create_introspect**](DefaultApi.md#create_introspect) | **POST** /oauth2/introspect | 
[**create_ip_access_control_list**](DefaultApi.md#create_ip_access_control_list) | **POST** /api/ip-acl | 
[**create_ip_access_control_list_with_id**](DefaultApi.md#create_ip_access_control_list_with_id) | **POST** /api/ip-acl/{accessControlListId} | 
[**create_lambda**](DefaultApi.md#create_lambda) | **POST** /api/lambda | 
[**create_lambda_with_id**](DefaultApi.md#create_lambda_with_id) | **POST** /api/lambda/{lambdaId} | 
[**create_logout**](DefaultApi.md#create_logout) | **POST** /api/logout | 
[**create_message_template**](DefaultApi.md#create_message_template) | **POST** /api/message/template | 
[**create_message_template_with_id**](DefaultApi.md#create_message_template_with_id) | **POST** /api/message/template/{messageTemplateId} | 
[**create_messenger**](DefaultApi.md#create_messenger) | **POST** /api/messenger | 
[**create_messenger_with_id**](DefaultApi.md#create_messenger_with_id) | **POST** /api/messenger/{messengerId} | 
[**create_o_auth_scope**](DefaultApi.md#create_o_auth_scope) | **POST** /api/application/{applicationId}/scope | 
[**create_o_auth_scope_with_id**](DefaultApi.md#create_o_auth_scope_with_id) | **POST** /api/application/{applicationId}/scope/{scopeId} | 
[**create_tenant**](DefaultApi.md#create_tenant) | **POST** /api/tenant | 
[**create_tenant_with_id**](DefaultApi.md#create_tenant_with_id) | **POST** /api/tenant/{tenantId} | 
[**create_theme**](DefaultApi.md#create_theme) | **POST** /api/theme | 
[**create_theme_with_id**](DefaultApi.md#create_theme_with_id) | **POST** /api/theme/{themeId} | 
[**create_token**](DefaultApi.md#create_token) | **POST** /oauth2/token | 
[**create_user**](DefaultApi.md#create_user) | **POST** /api/user | 
[**create_user_action**](DefaultApi.md#create_user_action) | **POST** /api/user-action | 
[**create_user_action_reason**](DefaultApi.md#create_user_action_reason) | **POST** /api/user-action-reason | 
[**create_user_action_reason_with_id**](DefaultApi.md#create_user_action_reason_with_id) | **POST** /api/user-action-reason/{userActionReasonId} | 
[**create_user_action_with_id**](DefaultApi.md#create_user_action_with_id) | **POST** /api/user-action/{userActionId} | 
[**create_user_consent**](DefaultApi.md#create_user_consent) | **POST** /api/user/consent | 
[**create_user_consent_with_id**](DefaultApi.md#create_user_consent_with_id) | **POST** /api/user/consent/{userConsentId} | 
[**create_user_link_with_id**](DefaultApi.md#create_user_link_with_id) | **POST** /api/identity-provider/link | 
[**create_user_verify_email**](DefaultApi.md#create_user_verify_email) | **POST** /api/user/verify-email | 
[**create_user_with_id**](DefaultApi.md#create_user_with_id) | **POST** /api/user/{userId} | 
[**create_webhook**](DefaultApi.md#create_webhook) | **POST** /api/webhook | 
[**create_webhook_with_id**](DefaultApi.md#create_webhook_with_id) | **POST** /api/webhook/{webhookId} | 
[**delete_api_key_with_id**](DefaultApi.md#delete_api_key_with_id) | **DELETE** /api/api-key/{keyId} | 
[**delete_application_role_with_id**](DefaultApi.md#delete_application_role_with_id) | **DELETE** /api/application/{applicationId}/role/{roleId} | 
[**delete_application_with_id**](DefaultApi.md#delete_application_with_id) | **DELETE** /api/application/{applicationId} | 
[**delete_connector_with_id**](DefaultApi.md#delete_connector_with_id) | **DELETE** /api/connector/{connectorId} | 
[**delete_consent_with_id**](DefaultApi.md#delete_consent_with_id) | **DELETE** /api/consent/{consentId} | 
[**delete_email_template_with_id**](DefaultApi.md#delete_email_template_with_id) | **DELETE** /api/email/template/{emailTemplateId} | 
[**delete_entity_grant_with_id**](DefaultApi.md#delete_entity_grant_with_id) | **DELETE** /api/entity/{entityId}/grant | 
[**delete_entity_type_permission_with_id**](DefaultApi.md#delete_entity_type_permission_with_id) | **DELETE** /api/entity/type/{entityTypeId}/permission/{permissionId} | 
[**delete_entity_type_with_id**](DefaultApi.md#delete_entity_type_with_id) | **DELETE** /api/entity/type/{entityTypeId} | 
[**delete_entity_with_id**](DefaultApi.md#delete_entity_with_id) | **DELETE** /api/entity/{entityId} | 
[**delete_form_field_with_id**](DefaultApi.md#delete_form_field_with_id) | **DELETE** /api/form/field/{fieldId} | 
[**delete_form_with_id**](DefaultApi.md#delete_form_with_id) | **DELETE** /api/form/{formId} | 
[**delete_group_members_with_id**](DefaultApi.md#delete_group_members_with_id) | **DELETE** /api/group/member | 
[**delete_group_with_id**](DefaultApi.md#delete_group_with_id) | **DELETE** /api/group/{groupId} | 
[**delete_identity_provider_with_id**](DefaultApi.md#delete_identity_provider_with_id) | **DELETE** /api/identity-provider/{identityProviderId} | 
[**delete_ip_access_control_list_with_id**](DefaultApi.md#delete_ip_access_control_list_with_id) | **DELETE** /api/ip-acl/{ipAccessControlListId} | 
[**delete_jwt_refresh**](DefaultApi.md#delete_jwt_refresh) | **DELETE** /api/jwt/refresh | 
[**delete_key_with_id**](DefaultApi.md#delete_key_with_id) | **DELETE** /api/key/{keyId} | 
[**delete_lambda_with_id**](DefaultApi.md#delete_lambda_with_id) | **DELETE** /api/lambda/{lambdaId} | 
[**delete_message_template_with_id**](DefaultApi.md#delete_message_template_with_id) | **DELETE** /api/message/template/{messageTemplateId} | 
[**delete_messenger_with_id**](DefaultApi.md#delete_messenger_with_id) | **DELETE** /api/messenger/{messengerId} | 
[**delete_o_auth_scope_with_id**](DefaultApi.md#delete_o_auth_scope_with_id) | **DELETE** /api/application/{applicationId}/scope/{scopeId} | 
[**delete_tenant_with_id**](DefaultApi.md#delete_tenant_with_id) | **DELETE** /api/tenant/{tenantId} | 
[**delete_theme_with_id**](DefaultApi.md#delete_theme_with_id) | **DELETE** /api/theme/{themeId} | 
[**delete_user_action_reason_with_id**](DefaultApi.md#delete_user_action_reason_with_id) | **DELETE** /api/user-action-reason/{userActionReasonId} | 
[**delete_user_action_with_id**](DefaultApi.md#delete_user_action_with_id) | **DELETE** /api/user-action/{userActionId} | 
[**delete_user_bulk**](DefaultApi.md#delete_user_bulk) | **DELETE** /api/user/bulk | 
[**delete_user_link_with_id**](DefaultApi.md#delete_user_link_with_id) | **DELETE** /api/identity-provider/link | 
[**delete_user_registration_with_id**](DefaultApi.md#delete_user_registration_with_id) | **DELETE** /api/user/registration/{userId}/{applicationId} | 
[**delete_user_two_factor_with_id**](DefaultApi.md#delete_user_two_factor_with_id) | **DELETE** /api/user/two-factor/{userId} | 
[**delete_user_with_id**](DefaultApi.md#delete_user_with_id) | **DELETE** /api/user/{userId} | 
[**delete_web_authn_credential_with_id**](DefaultApi.md#delete_web_authn_credential_with_id) | **DELETE** /api/webauthn/{id} | 
[**delete_webhook_with_id**](DefaultApi.md#delete_webhook_with_id) | **DELETE** /api/webhook/{webhookId} | 
[**enable_two_factor_with_id**](DefaultApi.md#enable_two_factor_with_id) | **POST** /api/user/two-factor/{userId} | 
[**exchange_refresh_token_for_jwt_with_id**](DefaultApi.md#exchange_refresh_token_for_jwt_with_id) | **POST** /api/jwt/refresh | 
[**forgot_password_with_id**](DefaultApi.md#forgot_password_with_id) | **POST** /api/user/forgot-password | 
[**generate_key**](DefaultApi.md#generate_key) | **POST** /api/key/generate | 
[**generate_key_with_id**](DefaultApi.md#generate_key_with_id) | **POST** /api/key/generate/{keyId} | 
[**generate_two_factor_recovery_codes_with_id**](DefaultApi.md#generate_two_factor_recovery_codes_with_id) | **POST** /api/user/two-factor/recovery-code/{userId} | 
[**generate_two_factor_secret_using_jwt_with_id**](DefaultApi.md#generate_two_factor_secret_using_jwt_with_id) | **GET** /api/two-factor/secret | 
[**identity_provider_login_with_id**](DefaultApi.md#identity_provider_login_with_id) | **POST** /api/identity-provider/login | 
[**import_key**](DefaultApi.md#import_key) | **POST** /api/key/import | 
[**import_key_with_id**](DefaultApi.md#import_key_with_id) | **POST** /api/key/import/{keyId} | 
[**import_refresh_tokens_with_id**](DefaultApi.md#import_refresh_tokens_with_id) | **POST** /api/user/refresh-token/import | 
[**import_users_with_id**](DefaultApi.md#import_users_with_id) | **POST** /api/user/import | 
[**import_web_authn_credential_with_id**](DefaultApi.md#import_web_authn_credential_with_id) | **POST** /api/webauthn/import | 
[**issue_jwt_with_id**](DefaultApi.md#issue_jwt_with_id) | **GET** /api/jwt/issue | 
[**login_ping_with_id**](DefaultApi.md#login_ping_with_id) | **PUT** /api/login/{userId}/{applicationId} | 
[**login_ping_with_request_with_id**](DefaultApi.md#login_ping_with_request_with_id) | **PUT** /api/login | 
[**login_with_id**](DefaultApi.md#login_with_id) | **POST** /api/login | 
[**lookup_identity_provider_with_id**](DefaultApi.md#lookup_identity_provider_with_id) | **GET** /api/identity-provider/lookup | 
[**modify_action_with_id**](DefaultApi.md#modify_action_with_id) | **PUT** /api/user/action/{actionId} | 
[**passwordless_login_with_id**](DefaultApi.md#passwordless_login_with_id) | **POST** /api/passwordless/login | 
[**patch_application_role_with_id**](DefaultApi.md#patch_application_role_with_id) | **PATCH** /api/application/{applicationId}/role/{roleId} | 
[**patch_application_with_id**](DefaultApi.md#patch_application_with_id) | **PATCH** /api/application/{applicationId} | 
[**patch_connector_with_id**](DefaultApi.md#patch_connector_with_id) | **PATCH** /api/connector/{connectorId} | 
[**patch_consent_with_id**](DefaultApi.md#patch_consent_with_id) | **PATCH** /api/consent/{consentId} | 
[**patch_email_template_with_id**](DefaultApi.md#patch_email_template_with_id) | **PATCH** /api/email/template/{emailTemplateId} | 
[**patch_entity_type_permission_with_id**](DefaultApi.md#patch_entity_type_permission_with_id) | **PATCH** /api/entity/type/{entityTypeId}/permission/{permissionId} | 
[**patch_entity_type_with_id**](DefaultApi.md#patch_entity_type_with_id) | **PATCH** /api/entity/type/{entityTypeId} | 
[**patch_entity_with_id**](DefaultApi.md#patch_entity_with_id) | **PATCH** /api/entity/{entityId} | 
[**patch_form_field_with_id**](DefaultApi.md#patch_form_field_with_id) | **PATCH** /api/form/field/{fieldId} | 
[**patch_form_with_id**](DefaultApi.md#patch_form_with_id) | **PATCH** /api/form/{formId} | 
[**patch_group_with_id**](DefaultApi.md#patch_group_with_id) | **PATCH** /api/group/{groupId} | 
[**patch_identity_provider_with_id**](DefaultApi.md#patch_identity_provider_with_id) | **PATCH** /api/identity-provider/{identityProviderId} | 
[**patch_integrations_with_id**](DefaultApi.md#patch_integrations_with_id) | **PATCH** /api/integration | 
[**patch_ip_access_control_list_with_id**](DefaultApi.md#patch_ip_access_control_list_with_id) | **PATCH** /api/ip-acl/{accessControlListId} | 
[**patch_lambda_with_id**](DefaultApi.md#patch_lambda_with_id) | **PATCH** /api/lambda/{lambdaId} | 
[**patch_message_template_with_id**](DefaultApi.md#patch_message_template_with_id) | **PATCH** /api/message/template/{messageTemplateId} | 
[**patch_messenger_with_id**](DefaultApi.md#patch_messenger_with_id) | **PATCH** /api/messenger/{messengerId} | 
[**patch_o_auth_scope_with_id**](DefaultApi.md#patch_o_auth_scope_with_id) | **PATCH** /api/application/{applicationId}/scope/{scopeId} | 
[**patch_registration_with_id**](DefaultApi.md#patch_registration_with_id) | **PATCH** /api/user/registration/{userId} | 
[**patch_system_configuration_with_id**](DefaultApi.md#patch_system_configuration_with_id) | **PATCH** /api/system-configuration | 
[**patch_tenant_with_id**](DefaultApi.md#patch_tenant_with_id) | **PATCH** /api/tenant/{tenantId} | 
[**patch_theme_with_id**](DefaultApi.md#patch_theme_with_id) | **PATCH** /api/theme/{themeId} | 
[**patch_user_action_reason_with_id**](DefaultApi.md#patch_user_action_reason_with_id) | **PATCH** /api/user-action-reason/{userActionReasonId} | 
[**patch_user_action_with_id**](DefaultApi.md#patch_user_action_with_id) | **PATCH** /api/user-action/{userActionId} | 
[**patch_user_consent_with_id**](DefaultApi.md#patch_user_consent_with_id) | **PATCH** /api/user/consent/{userConsentId} | 
[**patch_user_with_id**](DefaultApi.md#patch_user_with_id) | **PATCH** /api/user/{userId} | 
[**patch_webhook_with_id**](DefaultApi.md#patch_webhook_with_id) | **PATCH** /api/webhook/{webhookId} | 
[**reconcile_jwt_with_id**](DefaultApi.md#reconcile_jwt_with_id) | **POST** /api/jwt/reconcile | 
[**register**](DefaultApi.md#register) | **POST** /api/user/registration | 
[**register_with_id**](DefaultApi.md#register_with_id) | **POST** /api/user/registration/{userId} | 
[**reindex_with_id**](DefaultApi.md#reindex_with_id) | **POST** /api/system/reindex | 
[**remove_user_from_family_with_id**](DefaultApi.md#remove_user_from_family_with_id) | **DELETE** /api/user/family/{familyId}/{userId} | 
[**retrieve_action_with_id**](DefaultApi.md#retrieve_action_with_id) | **GET** /api/user/action/{actionId} | 
[**retrieve_api_key_with_id**](DefaultApi.md#retrieve_api_key_with_id) | **GET** /api/api-key/{keyId} | 
[**retrieve_application**](DefaultApi.md#retrieve_application) | **GET** /api/application | 
[**retrieve_application_with_id**](DefaultApi.md#retrieve_application_with_id) | **GET** /api/application/{applicationId} | 
[**retrieve_audit_log_with_id**](DefaultApi.md#retrieve_audit_log_with_id) | **GET** /api/system/audit-log/{auditLogId} | 
[**retrieve_connector_with_id**](DefaultApi.md#retrieve_connector_with_id) | **GET** /api/connector/{connectorId} | 
[**retrieve_consent_with_id**](DefaultApi.md#retrieve_consent_with_id) | **GET** /api/consent/{consentId} | 
[**retrieve_daily_active_report_with_id**](DefaultApi.md#retrieve_daily_active_report_with_id) | **GET** /api/report/daily-active-user | 
[**retrieve_device_user_code**](DefaultApi.md#retrieve_device_user_code) | **GET** /oauth2/device/user-code | 
[**retrieve_email_template**](DefaultApi.md#retrieve_email_template) | **GET** /api/email/template | 
[**retrieve_email_template_preview_with_id**](DefaultApi.md#retrieve_email_template_preview_with_id) | **POST** /api/email/template/preview | 
[**retrieve_email_template_with_id**](DefaultApi.md#retrieve_email_template_with_id) | **GET** /api/email/template/{emailTemplateId} | 
[**retrieve_entity_grant_with_id**](DefaultApi.md#retrieve_entity_grant_with_id) | **GET** /api/entity/{entityId}/grant | 
[**retrieve_entity_type_with_id**](DefaultApi.md#retrieve_entity_type_with_id) | **GET** /api/entity/type/{entityTypeId} | 
[**retrieve_entity_with_id**](DefaultApi.md#retrieve_entity_with_id) | **GET** /api/entity/{entityId} | 
[**retrieve_event_log_with_id**](DefaultApi.md#retrieve_event_log_with_id) | **GET** /api/system/event-log/{eventLogId} | 
[**retrieve_families_with_id**](DefaultApi.md#retrieve_families_with_id) | **GET** /api/user/family | 
[**retrieve_family_members_by_family_id_with_id**](DefaultApi.md#retrieve_family_members_by_family_id_with_id) | **GET** /api/user/family/{familyId} | 
[**retrieve_form_field_with_id**](DefaultApi.md#retrieve_form_field_with_id) | **GET** /api/form/field/{fieldId} | 
[**retrieve_form_with_id**](DefaultApi.md#retrieve_form_with_id) | **GET** /api/form/{formId} | 
[**retrieve_group_with_id**](DefaultApi.md#retrieve_group_with_id) | **GET** /api/group/{groupId} | 
[**retrieve_identity_provider_by_type_with_id**](DefaultApi.md#retrieve_identity_provider_by_type_with_id) | **GET** /api/identity-provider | 
[**retrieve_identity_provider_link**](DefaultApi.md#retrieve_identity_provider_link) | **GET** /api/identity-provider/link | 
[**retrieve_identity_provider_with_id**](DefaultApi.md#retrieve_identity_provider_with_id) | **GET** /api/identity-provider/{identityProviderId} | 
[**retrieve_ip_access_control_list_with_id**](DefaultApi.md#retrieve_ip_access_control_list_with_id) | **GET** /api/ip-acl/{ipAccessControlListId} | 
[**retrieve_json_web_key_set_with_id**](DefaultApi.md#retrieve_json_web_key_set_with_id) | **GET** /.well-known/jwks.json | 
[**retrieve_jwt_public_key**](DefaultApi.md#retrieve_jwt_public_key) | **GET** /api/jwt/public-key | 
[**retrieve_key_with_id**](DefaultApi.md#retrieve_key_with_id) | **GET** /api/key/{keyId} | 
[**retrieve_keys_with_id**](DefaultApi.md#retrieve_keys_with_id) | **GET** /api/key | 
[**retrieve_lambda_with_id**](DefaultApi.md#retrieve_lambda_with_id) | **GET** /api/lambda/{lambdaId} | 
[**retrieve_lambdas_by_type_with_id**](DefaultApi.md#retrieve_lambdas_by_type_with_id) | **GET** /api/lambda | 
[**retrieve_message_template**](DefaultApi.md#retrieve_message_template) | **GET** /api/message/template | 
[**retrieve_message_template_preview_with_id**](DefaultApi.md#retrieve_message_template_preview_with_id) | **POST** /api/message/template/preview | 
[**retrieve_message_template_with_id**](DefaultApi.md#retrieve_message_template_with_id) | **GET** /api/message/template/{messageTemplateId} | 
[**retrieve_messenger_with_id**](DefaultApi.md#retrieve_messenger_with_id) | **GET** /api/messenger/{messengerId} | 
[**retrieve_monthly_active_report_with_id**](DefaultApi.md#retrieve_monthly_active_report_with_id) | **GET** /api/report/monthly-active-user | 
[**retrieve_o_auth_scope_with_id**](DefaultApi.md#retrieve_o_auth_scope_with_id) | **GET** /api/application/{applicationId}/scope/{scopeId} | 
[**retrieve_oauth_configuration_with_id**](DefaultApi.md#retrieve_oauth_configuration_with_id) | **GET** /api/application/{applicationId}/oauth-configuration | 
[**retrieve_open_id_configuration_with_id**](DefaultApi.md#retrieve_open_id_configuration_with_id) | **GET** /.well-known/openid-configuration | 
[**retrieve_password_validation_rules_with_id**](DefaultApi.md#retrieve_password_validation_rules_with_id) | **GET** /api/tenant/password-validation-rules | 
[**retrieve_password_validation_rules_with_tenant_id_with_id**](DefaultApi.md#retrieve_password_validation_rules_with_tenant_id_with_id) | **GET** /api/tenant/password-validation-rules/{tenantId} | 
[**retrieve_pending_children_with_id**](DefaultApi.md#retrieve_pending_children_with_id) | **GET** /api/user/family/pending | 
[**retrieve_pending_link_with_id**](DefaultApi.md#retrieve_pending_link_with_id) | **GET** /api/identity-provider/link/pending/{pendingLinkId} | 
[**retrieve_reactor_metrics_with_id**](DefaultApi.md#retrieve_reactor_metrics_with_id) | **GET** /api/reactor/metrics | 
[**retrieve_refresh_token_by_id_with_id**](DefaultApi.md#retrieve_refresh_token_by_id_with_id) | **GET** /api/jwt/refresh/{tokenId} | 
[**retrieve_refresh_tokens_with_id**](DefaultApi.md#retrieve_refresh_tokens_with_id) | **GET** /api/jwt/refresh | 
[**retrieve_registration_report_with_id**](DefaultApi.md#retrieve_registration_report_with_id) | **GET** /api/report/registration | 
[**retrieve_registration_with_id**](DefaultApi.md#retrieve_registration_with_id) | **GET** /api/user/registration/{userId}/{applicationId} | 
[**retrieve_report_login**](DefaultApi.md#retrieve_report_login) | **GET** /api/report/login | 
[**retrieve_status**](DefaultApi.md#retrieve_status) | **GET** /api/status | 
[**retrieve_system_health_with_id**](DefaultApi.md#retrieve_system_health_with_id) | **GET** /api/health | 
[**retrieve_tenant_with_id**](DefaultApi.md#retrieve_tenant_with_id) | **GET** /api/tenant/{tenantId} | 
[**retrieve_theme_with_id**](DefaultApi.md#retrieve_theme_with_id) | **GET** /api/theme/{themeId} | 
[**retrieve_total_report_with_id**](DefaultApi.md#retrieve_total_report_with_id) | **GET** /api/report/totals | 
[**retrieve_two_factor_recovery_codes_with_id**](DefaultApi.md#retrieve_two_factor_recovery_codes_with_id) | **GET** /api/user/two-factor/recovery-code/{userId} | 
[**retrieve_two_factor_status_with_id**](DefaultApi.md#retrieve_two_factor_status_with_id) | **GET** /api/two-factor/status/{twoFactorTrustId} | 
[**retrieve_user**](DefaultApi.md#retrieve_user) | **GET** /api/user | 
[**retrieve_user_action**](DefaultApi.md#retrieve_user_action) | **GET** /api/user-action | 
[**retrieve_user_action_reason**](DefaultApi.md#retrieve_user_action_reason) | **GET** /api/user-action-reason | 
[**retrieve_user_action_reason_with_id**](DefaultApi.md#retrieve_user_action_reason_with_id) | **GET** /api/user-action-reason/{userActionReasonId} | 
[**retrieve_user_action_with_id**](DefaultApi.md#retrieve_user_action_with_id) | **GET** /api/user-action/{userActionId} | 
[**retrieve_user_actioning**](DefaultApi.md#retrieve_user_actioning) | **GET** /api/user/action | 
[**retrieve_user_change_password**](DefaultApi.md#retrieve_user_change_password) | **GET** /api/user/change-password | 
[**retrieve_user_comments_with_id**](DefaultApi.md#retrieve_user_comments_with_id) | **GET** /api/user/comment/{userId} | 
[**retrieve_user_consent_with_id**](DefaultApi.md#retrieve_user_consent_with_id) | **GET** /api/user/consent/{userConsentId} | 
[**retrieve_user_consents_with_id**](DefaultApi.md#retrieve_user_consents_with_id) | **GET** /api/user/consent | 
[**retrieve_user_info_from_access_token_with_id**](DefaultApi.md#retrieve_user_info_from_access_token_with_id) | **GET** /oauth2/userinfo | 
[**retrieve_user_recent_login**](DefaultApi.md#retrieve_user_recent_login) | **GET** /api/user/recent-login | 
[**retrieve_user_with_id**](DefaultApi.md#retrieve_user_with_id) | **GET** /api/user/{userId} | 
[**retrieve_version_with_id**](DefaultApi.md#retrieve_version_with_id) | **GET** /api/system/version | 
[**retrieve_web_authn_credential_with_id**](DefaultApi.md#retrieve_web_authn_credential_with_id) | **GET** /api/webauthn/{id} | 
[**retrieve_web_authn_credentials_for_user_with_id**](DefaultApi.md#retrieve_web_authn_credentials_for_user_with_id) | **GET** /api/webauthn | 
[**retrieve_webhook**](DefaultApi.md#retrieve_webhook) | **GET** /api/webhook | 
[**retrieve_webhook_attempt_log_with_id**](DefaultApi.md#retrieve_webhook_attempt_log_with_id) | **GET** /api/system/webhook-attempt-log/{webhookAttemptLogId} | 
[**retrieve_webhook_event_log_with_id**](DefaultApi.md#retrieve_webhook_event_log_with_id) | **GET** /api/system/webhook-event-log/{webhookEventLogId} | 
[**retrieve_webhook_with_id**](DefaultApi.md#retrieve_webhook_with_id) | **GET** /api/webhook/{webhookId} | 
[**revoke_refresh_token_by_id_with_id**](DefaultApi.md#revoke_refresh_token_by_id_with_id) | **DELETE** /api/jwt/refresh/{tokenId} | 
[**revoke_user_consent_with_id**](DefaultApi.md#revoke_user_consent_with_id) | **DELETE** /api/user/consent/{userConsentId} | 
[**search_applications_with_id**](DefaultApi.md#search_applications_with_id) | **POST** /api/application/search | 
[**search_audit_logs_with_id**](DefaultApi.md#search_audit_logs_with_id) | **POST** /api/system/audit-log/search | 
[**search_consents_with_id**](DefaultApi.md#search_consents_with_id) | **POST** /api/consent/search | 
[**search_email_templates_with_id**](DefaultApi.md#search_email_templates_with_id) | **POST** /api/email/template/search | 
[**search_entities_by_ids_with_id**](DefaultApi.md#search_entities_by_ids_with_id) | **GET** /api/entity/search | 
[**search_entities_with_id**](DefaultApi.md#search_entities_with_id) | **POST** /api/entity/search | 
[**search_entity_grants_with_id**](DefaultApi.md#search_entity_grants_with_id) | **POST** /api/entity/grant/search | 
[**search_entity_types_with_id**](DefaultApi.md#search_entity_types_with_id) | **POST** /api/entity/type/search | 
[**search_event_logs_with_id**](DefaultApi.md#search_event_logs_with_id) | **POST** /api/system/event-log/search | 
[**search_group_members_with_id**](DefaultApi.md#search_group_members_with_id) | **POST** /api/group/member/search | 
[**search_groups_with_id**](DefaultApi.md#search_groups_with_id) | **POST** /api/group/search | 
[**search_identity_providers_with_id**](DefaultApi.md#search_identity_providers_with_id) | **POST** /api/identity-provider/search | 
[**search_ip_access_control_lists_with_id**](DefaultApi.md#search_ip_access_control_lists_with_id) | **POST** /api/ip-acl/search | 
[**search_keys_with_id**](DefaultApi.md#search_keys_with_id) | **POST** /api/key/search | 
[**search_lambdas_with_id**](DefaultApi.md#search_lambdas_with_id) | **POST** /api/lambda/search | 
[**search_login_records_with_id**](DefaultApi.md#search_login_records_with_id) | **POST** /api/system/login-record/search | 
[**search_tenants_with_id**](DefaultApi.md#search_tenants_with_id) | **POST** /api/tenant/search | 
[**search_themes_with_id**](DefaultApi.md#search_themes_with_id) | **POST** /api/theme/search | 
[**search_user_comments_with_id**](DefaultApi.md#search_user_comments_with_id) | **POST** /api/user/comment/search | 
[**search_users_by_ids_with_id**](DefaultApi.md#search_users_by_ids_with_id) | **GET** /api/user/search | 
[**search_users_by_query_with_id**](DefaultApi.md#search_users_by_query_with_id) | **POST** /api/user/search | 
[**search_webhook_event_logs_with_id**](DefaultApi.md#search_webhook_event_logs_with_id) | **POST** /api/system/webhook-event-log/search | 
[**search_webhooks_with_id**](DefaultApi.md#search_webhooks_with_id) | **POST** /api/webhook/search | 
[**send_email_with_id**](DefaultApi.md#send_email_with_id) | **POST** /api/email/send/{emailTemplateId} | 
[**send_family_request_email_with_id**](DefaultApi.md#send_family_request_email_with_id) | **POST** /api/user/family/request | 
[**send_passwordless_code_with_id**](DefaultApi.md#send_passwordless_code_with_id) | **POST** /api/passwordless/send | 
[**send_two_factor_code_for_enable_disable_with_id**](DefaultApi.md#send_two_factor_code_for_enable_disable_with_id) | **POST** /api/two-factor/send | 
[**send_two_factor_code_for_login_using_method_with_id**](DefaultApi.md#send_two_factor_code_for_login_using_method_with_id) | **POST** /api/two-factor/send/{twoFactorId} | 
[**start_identity_provider_login_with_id**](DefaultApi.md#start_identity_provider_login_with_id) | **POST** /api/identity-provider/start | 
[**start_passwordless_login_with_id**](DefaultApi.md#start_passwordless_login_with_id) | **POST** /api/passwordless/start | 
[**start_two_factor_login_with_id**](DefaultApi.md#start_two_factor_login_with_id) | **POST** /api/two-factor/start | 
[**start_web_authn_login_with_id**](DefaultApi.md#start_web_authn_login_with_id) | **POST** /api/webauthn/start | 
[**start_web_authn_registration_with_id**](DefaultApi.md#start_web_authn_registration_with_id) | **POST** /api/webauthn/register/start | 
[**two_factor_login_with_id**](DefaultApi.md#two_factor_login_with_id) | **POST** /api/two-factor/login | 
[**update_api_key_with_id**](DefaultApi.md#update_api_key_with_id) | **PUT** /api/api-key/{apiKeyId} | 
[**update_application_role_with_id**](DefaultApi.md#update_application_role_with_id) | **PUT** /api/application/{applicationId}/role/{roleId} | 
[**update_application_with_id**](DefaultApi.md#update_application_with_id) | **PUT** /api/application/{applicationId} | 
[**update_connector_with_id**](DefaultApi.md#update_connector_with_id) | **PUT** /api/connector/{connectorId} | 
[**update_consent_with_id**](DefaultApi.md#update_consent_with_id) | **PUT** /api/consent/{consentId} | 
[**update_email_template_with_id**](DefaultApi.md#update_email_template_with_id) | **PUT** /api/email/template/{emailTemplateId} | 
[**update_entity_type_permission_with_id**](DefaultApi.md#update_entity_type_permission_with_id) | **PUT** /api/entity/type/{entityTypeId}/permission/{permissionId} | 
[**update_entity_type_with_id**](DefaultApi.md#update_entity_type_with_id) | **PUT** /api/entity/type/{entityTypeId} | 
[**update_entity_with_id**](DefaultApi.md#update_entity_with_id) | **PUT** /api/entity/{entityId} | 
[**update_form_field_with_id**](DefaultApi.md#update_form_field_with_id) | **PUT** /api/form/field/{fieldId} | 
[**update_form_with_id**](DefaultApi.md#update_form_with_id) | **PUT** /api/form/{formId} | 
[**update_group_members_with_id**](DefaultApi.md#update_group_members_with_id) | **PUT** /api/group/member | 
[**update_group_with_id**](DefaultApi.md#update_group_with_id) | **PUT** /api/group/{groupId} | 
[**update_identity_provider_with_id**](DefaultApi.md#update_identity_provider_with_id) | **PUT** /api/identity-provider/{identityProviderId} | 
[**update_integrations_with_id**](DefaultApi.md#update_integrations_with_id) | **PUT** /api/integration | 
[**update_ip_access_control_list_with_id**](DefaultApi.md#update_ip_access_control_list_with_id) | **PUT** /api/ip-acl/{accessControlListId} | 
[**update_key_with_id**](DefaultApi.md#update_key_with_id) | **PUT** /api/key/{keyId} | 
[**update_lambda_with_id**](DefaultApi.md#update_lambda_with_id) | **PUT** /api/lambda/{lambdaId} | 
[**update_message_template_with_id**](DefaultApi.md#update_message_template_with_id) | **PUT** /api/message/template/{messageTemplateId} | 
[**update_messenger_with_id**](DefaultApi.md#update_messenger_with_id) | **PUT** /api/messenger/{messengerId} | 
[**update_o_auth_scope_with_id**](DefaultApi.md#update_o_auth_scope_with_id) | **PUT** /api/application/{applicationId}/scope/{scopeId} | 
[**update_registration_with_id**](DefaultApi.md#update_registration_with_id) | **PUT** /api/user/registration/{userId} | 
[**update_system_configuration_with_id**](DefaultApi.md#update_system_configuration_with_id) | **PUT** /api/system-configuration | 
[**update_tenant_with_id**](DefaultApi.md#update_tenant_with_id) | **PUT** /api/tenant/{tenantId} | 
[**update_theme_with_id**](DefaultApi.md#update_theme_with_id) | **PUT** /api/theme/{themeId} | 
[**update_user_action_reason_with_id**](DefaultApi.md#update_user_action_reason_with_id) | **PUT** /api/user-action-reason/{userActionReasonId} | 
[**update_user_action_with_id**](DefaultApi.md#update_user_action_with_id) | **PUT** /api/user-action/{userActionId} | 
[**update_user_consent_with_id**](DefaultApi.md#update_user_consent_with_id) | **PUT** /api/user/consent/{userConsentId} | 
[**update_user_family_with_id**](DefaultApi.md#update_user_family_with_id) | **PUT** /api/user/family/{familyId} | 
[**update_user_verify_email**](DefaultApi.md#update_user_verify_email) | **PUT** /api/user/verify-email | 
[**update_user_verify_registration**](DefaultApi.md#update_user_verify_registration) | **PUT** /api/user/verify-registration | 
[**update_user_with_id**](DefaultApi.md#update_user_with_id) | **PUT** /api/user/{userId} | 
[**update_webhook_with_id**](DefaultApi.md#update_webhook_with_id) | **PUT** /api/webhook/{webhookId} | 
[**upsert_entity_grant_with_id**](DefaultApi.md#upsert_entity_grant_with_id) | **POST** /api/entity/{entityId}/grant | 
[**validate_device_with_id**](DefaultApi.md#validate_device_with_id) | **GET** /oauth2/device/validate | 
[**validate_jwt_with_id**](DefaultApi.md#validate_jwt_with_id) | **GET** /api/jwt/validate | 
[**vend_jwt_with_id**](DefaultApi.md#vend_jwt_with_id) | **POST** /api/jwt/vend | 
[**verify_user_registration_with_id**](DefaultApi.md#verify_user_registration_with_id) | **POST** /api/user/verify-registration | 



## action_user_with_id

> models::ActionResponse action_user_with_id(action_request)


Takes an action on a user. The user being actioned is called the \"actionee\" and the user taking the action is called the \"actioner\". Both user ids are required in the request object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_request** | Option<[**ActionRequest**](ActionRequest.md)> |  |  |

### Return type

[**models::ActionResponse**](ActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activate_reactor_with_id

> activate_reactor_with_id(reactor_request)


Activates the FusionAuth Reactor using a license Id and optionally a license text (for air-gapped deployments)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reactor_request** | Option<[**ReactorRequest**](ReactorRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## approve_device_with_id

> models::DeviceApprovalResponse approve_device_with_id()


Approve a device grant.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DeviceApprovalResponse**](DeviceApprovalResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_action_with_id

> models::ActionResponse cancel_action_with_id(action_id, action_request)


Cancels the user action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | The action Id of the action to cancel. | [required] |
**action_request** | Option<[**ActionRequest**](ActionRequest.md)> |  |  |

### Return type

[**models::ActionResponse**](ActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_password_by_identity_with_id

> change_password_by_identity_with_id(change_password_request)


Changes a user's password using their identity (loginId and password). Using a loginId instead of the changePasswordId bypasses the email verification and allows a password to be changed directly without first calling the #forgotPassword method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_password_request** | Option<[**ChangePasswordRequest**](ChangePasswordRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_password_with_id

> models::ChangePasswordResponse change_password_with_id(change_password_id, change_password_request)


Changes a user's password using the change password Id. This usually occurs after an email has been sent to the user and they clicked on a link to reset their password.  As of version 1.32.2, prefer sending the changePasswordId in the request body. To do this, omit the first parameter, and set the value in the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_password_id** | **String** | The change password Id used to find the user. This value is generated by FusionAuth once the change password workflow has been initiated. | [required] |
**change_password_request** | Option<[**ChangePasswordRequest**](ChangePasswordRequest.md)> |  |  |

### Return type

[**models::ChangePasswordResponse**](ChangePasswordResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_change_password_using_id_with_id

> check_change_password_using_id_with_id(change_password_id)


Check to see if the user must obtain a Trust Token Id in order to complete a change password request. When a user has enabled Two-Factor authentication, before you are allowed to use the Change Password API to change your password, you must obtain a Trust Token by completing a Two-Factor Step-Up authentication.  An HTTP status code of 400 with a general error code of [TrustTokenRequired] indicates that a Trust Token is required to make a POST request to this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_password_id** | **String** | The change password Id used to find the user. This value is generated by FusionAuth once the change password workflow has been initiated. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comment_on_user_with_id

> models::UserCommentResponse comment_on_user_with_id(x_fusion_auth_tenant_id, user_comment_request)


Adds a comment to the user's account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**user_comment_request** | Option<[**UserCommentRequest**](UserCommentRequest.md)> |  |  |

### Return type

[**models::UserCommentResponse**](UserCommentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_web_authn_assertion_with_id

> models::WebAuthnAssertResponse complete_web_authn_assertion_with_id(web_authn_login_request)


Complete a WebAuthn authentication ceremony by validating the signature against the previously generated challenge without logging the user in

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_authn_login_request** | Option<[**WebAuthnLoginRequest**](WebAuthnLoginRequest.md)> |  |  |

### Return type

[**models::WebAuthnAssertResponse**](WebAuthnAssertResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_web_authn_login_with_id

> models::LoginResponse complete_web_authn_login_with_id(web_authn_login_request)


Complete a WebAuthn authentication ceremony by validating the signature against the previously generated challenge and then login the user in

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_authn_login_request** | Option<[**WebAuthnLoginRequest**](WebAuthnLoginRequest.md)> |  |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_web_authn_registration_with_id

> models::WebAuthnRegisterCompleteResponse complete_web_authn_registration_with_id(web_authn_register_complete_request)


Complete a WebAuthn registration ceremony by validating the client request and saving the new credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_authn_register_complete_request** | Option<[**WebAuthnRegisterCompleteRequest**](WebAuthnRegisterCompleteRequest.md)> |  |  |

### Return type

[**models::WebAuthnRegisterCompleteResponse**](WebAuthnRegisterCompleteResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_key

> models::ApiKeyResponse create_api_key(api_key_request)


Creates an API key. You can optionally specify a unique Id for the key, if not provided one will be generated. an API key can only be created with equal or lesser authority. An API key cannot create another API key unless it is granted  to that API key.  If an API key is locked to a tenant, it can only create API Keys for that same tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_request** | Option<[**ApiKeyRequest**](ApiKeyRequest.md)> |  |  |

### Return type

[**models::ApiKeyResponse**](APIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_key_with_id

> models::ApiKeyResponse create_api_key_with_id(key_id, api_key_request)


Creates an API key. You can optionally specify a unique Id for the key, if not provided one will be generated. an API key can only be created with equal or lesser authority. An API key cannot create another API key unless it is granted  to that API key.  If an API key is locked to a tenant, it can only create API Keys for that same tenant. OR Updates an authentication API key by given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The unique Id of the API key. If not provided a secure random Id will be generated. | [required] |
**api_key_request** | Option<[**ApiKeyRequest**](ApiKeyRequest.md)> |  |  |

### Return type

[**models::ApiKeyResponse**](APIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application

> models::ApplicationResponse create_application(x_fusion_auth_tenant_id, application_request)


Creates an application. You can optionally specify an Id for the application, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**application_request** | Option<[**ApplicationRequest**](ApplicationRequest.md)> |  |  |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application_role

> models::ApplicationResponse create_application_role(application_id, x_fusion_auth_tenant_id, application_request)


Creates a new role for an application. You must specify the Id of the application you are creating the role for. You can optionally specify an Id for the role inside the ApplicationRole object itself, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application to create the role on. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**application_request** | Option<[**ApplicationRequest**](ApplicationRequest.md)> |  |  |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application_role_with_id

> models::ApplicationResponse create_application_role_with_id(application_id, role_id, x_fusion_auth_tenant_id, application_request)


Creates a new role for an application. You must specify the Id of the application you are creating the role for. You can optionally specify an Id for the role inside the ApplicationRole object itself, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application to create the role on. | [required] |
**role_id** | **String** | The Id of the role. If not provided a secure random UUID will be generated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**application_request** | Option<[**ApplicationRequest**](ApplicationRequest.md)> |  |  |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application_with_id

> models::ApplicationResponse create_application_with_id(application_id, x_fusion_auth_tenant_id, application_request)


Creates an application. You can optionally specify an Id for the application, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id to use for the application. If not provided a secure random UUID will be generated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**application_request** | Option<[**ApplicationRequest**](ApplicationRequest.md)> |  |  |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_audit_log_with_id

> models::AuditLogResponse create_audit_log_with_id(audit_log_request)


Creates an audit log with the message and user name (usually an email). Audit logs should be written anytime you make changes to the FusionAuth database. When using the FusionAuth App web interface, any changes are automatically written to the audit log. However, if you are accessing the API, you must write the audit logs yourself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audit_log_request** | Option<[**AuditLogRequest**](AuditLogRequest.md)> |  |  |

### Return type

[**models::AuditLogResponse**](AuditLogResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_connector

> models::ConnectorResponse create_connector(connector_request)


Creates a connector.  You can optionally specify an Id for the connector, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_request** | Option<[**ConnectorRequest**](ConnectorRequest.md)> |  |  |

### Return type

[**models::ConnectorResponse**](ConnectorResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_connector_with_id

> models::ConnectorResponse create_connector_with_id(connector_id, connector_request)


Creates a connector.  You can optionally specify an Id for the connector, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** | The Id for the connector. If not provided a secure random UUID will be generated. | [required] |
**connector_request** | Option<[**ConnectorRequest**](ConnectorRequest.md)> |  |  |

### Return type

[**models::ConnectorResponse**](ConnectorResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_consent

> models::ConsentResponse create_consent(x_fusion_auth_tenant_id, consent_request)


Creates a user consent type. You can optionally specify an Id for the consent type, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**consent_request** | Option<[**ConsentRequest**](ConsentRequest.md)> |  |  |

### Return type

[**models::ConsentResponse**](ConsentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_consent_with_id

> models::ConsentResponse create_consent_with_id(consent_id, x_fusion_auth_tenant_id, consent_request)


Creates a user consent type. You can optionally specify an Id for the consent type, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | The Id for the consent. If not provided a secure random UUID will be generated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**consent_request** | Option<[**ConsentRequest**](ConsentRequest.md)> |  |  |

### Return type

[**models::ConsentResponse**](ConsentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_email_template

> models::EmailTemplateResponse create_email_template(x_fusion_auth_tenant_id, email_template_request)


Creates an email template. You can optionally specify an Id for the template, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**email_template_request** | Option<[**EmailTemplateRequest**](EmailTemplateRequest.md)> |  |  |

### Return type

[**models::EmailTemplateResponse**](EmailTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_email_template_with_id

> models::EmailTemplateResponse create_email_template_with_id(email_template_id, x_fusion_auth_tenant_id, email_template_request)


Creates an email template. You can optionally specify an Id for the template, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_id** | **String** | The Id for the template. If not provided a secure random UUID will be generated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**email_template_request** | Option<[**EmailTemplateRequest**](EmailTemplateRequest.md)> |  |  |

### Return type

[**models::EmailTemplateResponse**](EmailTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_entity

> models::EntityResponse create_entity(x_fusion_auth_tenant_id, entity_request)


Creates an Entity. You can optionally specify an Id for the Entity. If not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**entity_request** | Option<[**EntityRequest**](EntityRequest.md)> |  |  |

### Return type

[**models::EntityResponse**](EntityResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_entity_type

> models::EntityTypeResponse create_entity_type(entity_type_request)


Creates a Entity Type. You can optionally specify an Id for the Entity Type, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_request** | Option<[**EntityTypeRequest**](EntityTypeRequest.md)> |  |  |

### Return type

[**models::EntityTypeResponse**](EntityTypeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_entity_type_permission

> models::EntityTypeResponse create_entity_type_permission(entity_type_id, entity_type_request)


Creates a new permission for an entity type. You must specify the Id of the entity type you are creating the permission for. You can optionally specify an Id for the permission inside the EntityTypePermission object itself, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_id** | **String** | The Id of the entity type to create the permission on. | [required] |
**entity_type_request** | Option<[**EntityTypeRequest**](EntityTypeRequest.md)> |  |  |

### Return type

[**models::EntityTypeResponse**](EntityTypeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_entity_type_permission_with_id

> models::EntityTypeResponse create_entity_type_permission_with_id(entity_type_id, permission_id, entity_type_request)


Creates a new permission for an entity type. You must specify the Id of the entity type you are creating the permission for. You can optionally specify an Id for the permission inside the EntityTypePermission object itself, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_id** | **String** | The Id of the entity type to create the permission on. | [required] |
**permission_id** | **String** | The Id of the permission. If not provided a secure random UUID will be generated. | [required] |
**entity_type_request** | Option<[**EntityTypeRequest**](EntityTypeRequest.md)> |  |  |

### Return type

[**models::EntityTypeResponse**](EntityTypeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_entity_type_with_id

> models::EntityTypeResponse create_entity_type_with_id(entity_type_id, entity_type_request)


Creates a Entity Type. You can optionally specify an Id for the Entity Type, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_id** | **String** | The Id for the Entity Type. If not provided a secure random UUID will be generated. | [required] |
**entity_type_request** | Option<[**EntityTypeRequest**](EntityTypeRequest.md)> |  |  |

### Return type

[**models::EntityTypeResponse**](EntityTypeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_entity_with_id

> models::EntityResponse create_entity_with_id(entity_id, x_fusion_auth_tenant_id, entity_request)


Creates an Entity. You can optionally specify an Id for the Entity. If not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | The Id for the Entity. If not provided a secure random UUID will be generated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**entity_request** | Option<[**EntityRequest**](EntityRequest.md)> |  |  |

### Return type

[**models::EntityResponse**](EntityResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_family

> models::FamilyResponse create_family(x_fusion_auth_tenant_id, family_request)


Creates a family with the user Id in the request as the owner and sole member of the family. You can optionally specify an Id for the family, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**family_request** | Option<[**FamilyRequest**](FamilyRequest.md)> |  |  |

### Return type

[**models::FamilyResponse**](FamilyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_family_with_id

> models::FamilyResponse create_family_with_id(family_id, x_fusion_auth_tenant_id, family_request)


Creates a family with the user Id in the request as the owner and sole member of the family. You can optionally specify an Id for the family, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**family_id** | **String** | The Id for the family. If not provided a secure random UUID will be generated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**family_request** | Option<[**FamilyRequest**](FamilyRequest.md)> |  |  |

### Return type

[**models::FamilyResponse**](FamilyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_form

> models::FormResponse create_form(form_request)


Creates a form.  You can optionally specify an Id for the form, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_request** | Option<[**FormRequest**](FormRequest.md)> |  |  |

### Return type

[**models::FormResponse**](FormResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_form_field

> models::FormFieldResponse create_form_field(form_field_request)


Creates a form field.  You can optionally specify an Id for the form, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_field_request** | Option<[**FormFieldRequest**](FormFieldRequest.md)> |  |  |

### Return type

[**models::FormFieldResponse**](FormFieldResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_form_field_with_id

> models::FormFieldResponse create_form_field_with_id(field_id, form_field_request)


Creates a form field.  You can optionally specify an Id for the form, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The Id for the form field. If not provided a secure random UUID will be generated. | [required] |
**form_field_request** | Option<[**FormFieldRequest**](FormFieldRequest.md)> |  |  |

### Return type

[**models::FormFieldResponse**](FormFieldResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_form_with_id

> models::FormResponse create_form_with_id(form_id, form_request)


Creates a form.  You can optionally specify an Id for the form, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | The Id for the form. If not provided a secure random UUID will be generated. | [required] |
**form_request** | Option<[**FormRequest**](FormRequest.md)> |  |  |

### Return type

[**models::FormResponse**](FormResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group

> models::GroupResponse create_group(x_fusion_auth_tenant_id, group_request)


Creates a group. You can optionally specify an Id for the group, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**group_request** | Option<[**GroupRequest**](GroupRequest.md)> |  |  |

### Return type

[**models::GroupResponse**](GroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_members_with_id

> models::MemberResponse create_group_members_with_id(member_request)


Creates a member in a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_request** | Option<[**MemberRequest**](MemberRequest.md)> |  |  |

### Return type

[**models::MemberResponse**](MemberResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_with_id

> models::GroupResponse create_group_with_id(group_id, x_fusion_auth_tenant_id, group_request)


Creates a group. You can optionally specify an Id for the group, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The Id for the group. If not provided a secure random UUID will be generated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**group_request** | Option<[**GroupRequest**](GroupRequest.md)> |  |  |

### Return type

[**models::GroupResponse**](GroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_identity_provider

> models::IdentityProviderResponse create_identity_provider(identity_provider_request)


Creates an identity provider. You can optionally specify an Id for the identity provider, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_request** | Option<[**IdentityProviderRequest**](IdentityProviderRequest.md)> |  |  |

### Return type

[**models::IdentityProviderResponse**](IdentityProviderResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_identity_provider_with_id

> models::IdentityProviderResponse create_identity_provider_with_id(identity_provider_id, identity_provider_request)


Creates an identity provider. You can optionally specify an Id for the identity provider, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_id** | **String** | The Id of the identity provider. If not provided a secure random UUID will be generated. | [required] |
**identity_provider_request** | Option<[**IdentityProviderRequest**](IdentityProviderRequest.md)> |  |  |

### Return type

[**models::IdentityProviderResponse**](IdentityProviderResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_introspect

> serde_json::Value create_introspect()


Inspect an access token issued as the result of the User based grant such as the Authorization Code Grant, Implicit Grant, the User Credentials Grant or the Refresh Grant. OR Inspect an access token issued as the result of the Client Credentials Grant.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ip_access_control_list

> models::IpAccessControlListResponse create_ip_access_control_list(ip_access_control_list_request)


Creates an IP Access Control List. You can optionally specify an Id on this create request, if one is not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_access_control_list_request** | Option<[**IpAccessControlListRequest**](IpAccessControlListRequest.md)> |  |  |

### Return type

[**models::IpAccessControlListResponse**](IPAccessControlListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ip_access_control_list_with_id

> models::IpAccessControlListResponse create_ip_access_control_list_with_id(access_control_list_id, ip_access_control_list_request)


Creates an IP Access Control List. You can optionally specify an Id on this create request, if one is not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_control_list_id** | **String** | The Id for the IP Access Control List. If not provided a secure random UUID will be generated. | [required] |
**ip_access_control_list_request** | Option<[**IpAccessControlListRequest**](IpAccessControlListRequest.md)> |  |  |

### Return type

[**models::IpAccessControlListResponse**](IPAccessControlListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_lambda

> models::LambdaResponse create_lambda(lambda_request)


Creates a Lambda. You can optionally specify an Id for the lambda, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lambda_request** | Option<[**LambdaRequest**](LambdaRequest.md)> |  |  |

### Return type

[**models::LambdaResponse**](LambdaResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_lambda_with_id

> models::LambdaResponse create_lambda_with_id(lambda_id, lambda_request)


Creates a Lambda. You can optionally specify an Id for the lambda, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lambda_id** | **String** | The Id for the lambda. If not provided a secure random UUID will be generated. | [required] |
**lambda_request** | Option<[**LambdaRequest**](LambdaRequest.md)> |  |  |

### Return type

[**models::LambdaResponse**](LambdaResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_logout

> create_logout(global, refresh_token, logout_request)


The Logout API is intended to be used to remove the refresh token and access token cookies if they exist on the client and revoke the refresh token stored. This API does nothing if the request does not contain an access token or refresh token cookies. OR The Logout API is intended to be used to remove the refresh token and access token cookies if they exist on the client and revoke the refresh token stored. This API takes the refresh token in the JSON body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**global** | Option<**String**> | When this value is set to true all the refresh tokens issued to the owner of the provided token will be revoked. |  |
**refresh_token** | Option<**String**> | The refresh_token as a request parameter instead of coming in via a cookie. If provided this takes precedence over the cookie. |  |
**logout_request** | Option<[**LogoutRequest**](LogoutRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_message_template

> models::MessageTemplateResponse create_message_template(message_template_request)


Creates an message template. You can optionally specify an Id for the template, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_template_request** | Option<[**MessageTemplateRequest**](MessageTemplateRequest.md)> |  |  |

### Return type

[**models::MessageTemplateResponse**](MessageTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_message_template_with_id

> models::MessageTemplateResponse create_message_template_with_id(message_template_id, message_template_request)


Creates an message template. You can optionally specify an Id for the template, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_template_id** | **String** | The Id for the template. If not provided a secure random UUID will be generated. | [required] |
**message_template_request** | Option<[**MessageTemplateRequest**](MessageTemplateRequest.md)> |  |  |

### Return type

[**models::MessageTemplateResponse**](MessageTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_messenger

> models::MessengerResponse create_messenger(messenger_request)


Creates a messenger.  You can optionally specify an Id for the messenger, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messenger_request** | Option<[**MessengerRequest**](MessengerRequest.md)> |  |  |

### Return type

[**models::MessengerResponse**](MessengerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_messenger_with_id

> models::MessengerResponse create_messenger_with_id(messenger_id, messenger_request)


Creates a messenger.  You can optionally specify an Id for the messenger, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messenger_id** | **String** | The Id for the messenger. If not provided a secure random UUID will be generated. | [required] |
**messenger_request** | Option<[**MessengerRequest**](MessengerRequest.md)> |  |  |

### Return type

[**models::MessengerResponse**](MessengerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_o_auth_scope

> models::ApplicationOAuthScopeResponse create_o_auth_scope(application_id, x_fusion_auth_tenant_id, application_o_auth_scope_request)


Creates a new custom OAuth scope for an application. You must specify the Id of the application you are creating the scope for. You can optionally specify an Id for the OAuth scope on the URL, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application to create the OAuth scope on. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**application_o_auth_scope_request** | Option<[**ApplicationOAuthScopeRequest**](ApplicationOAuthScopeRequest.md)> |  |  |

### Return type

[**models::ApplicationOAuthScopeResponse**](ApplicationOAuthScopeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_o_auth_scope_with_id

> models::ApplicationOAuthScopeResponse create_o_auth_scope_with_id(application_id, scope_id, x_fusion_auth_tenant_id, application_o_auth_scope_request)


Creates a new custom OAuth scope for an application. You must specify the Id of the application you are creating the scope for. You can optionally specify an Id for the OAuth scope on the URL, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application to create the OAuth scope on. | [required] |
**scope_id** | **String** | The Id of the OAuth scope. If not provided a secure random UUID will be generated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**application_o_auth_scope_request** | Option<[**ApplicationOAuthScopeRequest**](ApplicationOAuthScopeRequest.md)> |  |  |

### Return type

[**models::ApplicationOAuthScopeResponse**](ApplicationOAuthScopeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tenant

> models::TenantResponse create_tenant(x_fusion_auth_tenant_id, tenant_request)


Creates a tenant. You can optionally specify an Id for the tenant, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**tenant_request** | Option<[**TenantRequest**](TenantRequest.md)> |  |  |

### Return type

[**models::TenantResponse**](TenantResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tenant_with_id

> models::TenantResponse create_tenant_with_id(tenant_id, x_fusion_auth_tenant_id, tenant_request)


Creates a tenant. You can optionally specify an Id for the tenant, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | The Id for the tenant. If not provided a secure random UUID will be generated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**tenant_request** | Option<[**TenantRequest**](TenantRequest.md)> |  |  |

### Return type

[**models::TenantResponse**](TenantResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_theme

> models::ThemeResponse create_theme(theme_request)


Creates a Theme. You can optionally specify an Id for the theme, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme_request** | Option<[**ThemeRequest**](ThemeRequest.md)> |  |  |

### Return type

[**models::ThemeResponse**](ThemeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_theme_with_id

> models::ThemeResponse create_theme_with_id(theme_id, theme_request)


Creates a Theme. You can optionally specify an Id for the theme, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme_id** | **String** | The Id for the theme. If not provided a secure random UUID will be generated. | [required] |
**theme_request** | Option<[**ThemeRequest**](ThemeRequest.md)> |  |  |

### Return type

[**models::ThemeResponse**](ThemeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_token

> models::AccessToken create_token()


Exchanges an OAuth authorization code and code_verifier for an access token. Makes a request to the Token endpoint to exchange the authorization code returned from the Authorize endpoint and a code_verifier for an access token. OR Make a Client Credentials grant request to obtain an access token. OR Exchange a Refresh Token for an Access Token. If you will be using the Refresh Token Grant, you will make a request to the Token endpoint to exchange the user’s refresh token for an access token. OR Exchange User Credentials for a Token. If you will be using the Resource Owner Password Credential Grant, you will make a request to the Token endpoint to exchange the user’s email and password for an access token. OR Exchanges an OAuth authorization code for an access token. Makes a request to the Token endpoint to exchange the authorization code returned from the Authorize endpoint for an access token.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccessToken**](AccessToken.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> models::UserResponse create_user(x_fusion_auth_tenant_id, user_request)


Creates a user. You can optionally specify an Id for the user, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**user_request** | Option<[**UserRequest**](UserRequest.md)> |  |  |

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_action

> models::UserActionResponse create_user_action(x_fusion_auth_tenant_id, user_action_request)


Creates a user action. This action cannot be taken on a user until this call successfully returns. Anytime after that the user action can be applied to any user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**user_action_request** | Option<[**UserActionRequest**](UserActionRequest.md)> |  |  |

### Return type

[**models::UserActionResponse**](UserActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_action_reason

> models::UserActionReasonResponse create_user_action_reason(user_action_reason_request)


Creates a user reason. This user action reason cannot be used when actioning a user until this call completes successfully. Anytime after that the user action reason can be used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_action_reason_request** | Option<[**UserActionReasonRequest**](UserActionReasonRequest.md)> |  |  |

### Return type

[**models::UserActionReasonResponse**](UserActionReasonResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_action_reason_with_id

> models::UserActionReasonResponse create_user_action_reason_with_id(user_action_reason_id, user_action_reason_request)


Creates a user reason. This user action reason cannot be used when actioning a user until this call completes successfully. Anytime after that the user action reason can be used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_action_reason_id** | **String** | The Id for the user action reason. If not provided a secure random UUID will be generated. | [required] |
**user_action_reason_request** | Option<[**UserActionReasonRequest**](UserActionReasonRequest.md)> |  |  |

### Return type

[**models::UserActionReasonResponse**](UserActionReasonResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_action_with_id

> models::UserActionResponse create_user_action_with_id(user_action_id, x_fusion_auth_tenant_id, user_action_request)


Creates a user action. This action cannot be taken on a user until this call successfully returns. Anytime after that the user action can be applied to any user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_action_id** | **String** | The Id for the user action. If not provided a secure random UUID will be generated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**user_action_request** | Option<[**UserActionRequest**](UserActionRequest.md)> |  |  |

### Return type

[**models::UserActionResponse**](UserActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_consent

> models::UserConsentResponse create_user_consent(user_consent_request)


Creates a single User consent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_consent_request** | Option<[**UserConsentRequest**](UserConsentRequest.md)> |  |  |

### Return type

[**models::UserConsentResponse**](UserConsentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_consent_with_id

> models::UserConsentResponse create_user_consent_with_id(user_consent_id, user_consent_request)


Creates a single User consent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_consent_id** | **String** | The Id for the User consent. If not provided a secure random UUID will be generated. | [required] |
**user_consent_request** | Option<[**UserConsentRequest**](UserConsentRequest.md)> |  |  |

### Return type

[**models::UserConsentResponse**](UserConsentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_link_with_id

> models::IdentityProviderLinkResponse create_user_link_with_id(identity_provider_link_request)


Link an external user from a 3rd party identity provider to a FusionAuth user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_link_request** | Option<[**IdentityProviderLinkRequest**](IdentityProviderLinkRequest.md)> |  |  |

### Return type

[**models::IdentityProviderLinkResponse**](IdentityProviderLinkResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_verify_email

> create_user_verify_email(verify_email_request)


Administratively verify a user's email address. Use this method to bypass email verification for the user.  The request body will contain the userId to be verified. An API key is required when sending the userId in the request body. OR Confirms a user's email address.   The request body will contain the verificationId. You may also be required to send a one-time use code based upon your configuration. When  the tenant is configured to gate a user until their email address is verified, this procedures requires two values instead of one.  The verificationId is a high entropy value and the one-time use code is a low entropy value that is easily entered in a user interactive form. The  two values together are able to confirm a user's email address and mark the user's email address as verified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_email_request** | Option<[**VerifyEmailRequest**](VerifyEmailRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_with_id

> models::UserResponse create_user_with_id(user_id, x_fusion_auth_tenant_id, user_request)


Creates a user. You can optionally specify an Id for the user, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id for the user. If not provided a secure random UUID will be generated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**user_request** | Option<[**UserRequest**](UserRequest.md)> |  |  |

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_webhook

> models::WebhookResponse create_webhook(webhook_request)


Creates a webhook. You can optionally specify an Id for the webhook, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_request** | Option<[**WebhookRequest**](WebhookRequest.md)> |  |  |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_webhook_with_id

> models::WebhookResponse create_webhook_with_id(webhook_id, webhook_request)


Creates a webhook. You can optionally specify an Id for the webhook, if not provided one will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The Id for the webhook. If not provided a secure random UUID will be generated. | [required] |
**webhook_request** | Option<[**WebhookRequest**](WebhookRequest.md)> |  |  |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key_with_id

> delete_api_key_with_id(key_id)


Deletes the API key for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The Id of the authentication API key to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_role_with_id

> delete_application_role_with_id(application_id, role_id, x_fusion_auth_tenant_id)


Hard deletes an application role. This is a dangerous operation and should not be used in most circumstances. This permanently removes the given role from all users that had it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application that the role belongs to. | [required] |
**role_id** | **String** | The Id of the role to delete. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_with_id

> delete_application_with_id(application_id, hard_delete, x_fusion_auth_tenant_id)


Hard deletes an application. This is a dangerous operation and should not be used in most circumstances. This will delete the application, any registrations for that application, metrics and reports for the application, all the roles for the application, and any other data associated with the application. This operation could take a very long time, depending on the amount of data in your database. OR Deactivates the application with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application to delete. | [required] |
**hard_delete** | Option<**String**> |  |  |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connector_with_id

> delete_connector_with_id(connector_id)


Deletes the connector for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** | The Id of the connector to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_consent_with_id

> delete_consent_with_id(consent_id, x_fusion_auth_tenant_id)


Deletes the consent for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | The Id of the consent to delete. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_email_template_with_id

> delete_email_template_with_id(email_template_id, x_fusion_auth_tenant_id)


Deletes the email template for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_id** | **String** | The Id of the email template to delete. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_entity_grant_with_id

> delete_entity_grant_with_id(entity_id, recipient_entity_id, user_id, x_fusion_auth_tenant_id)


Deletes an Entity Grant for the given User or Entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | The Id of the Entity that the Entity Grant is being deleted for. | [required] |
**recipient_entity_id** | Option<**String**> | The Id of the Entity that the Entity Grant is for. |  |
**user_id** | Option<**String**> | The Id of the User that the Entity Grant is for. |  |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_entity_type_permission_with_id

> delete_entity_type_permission_with_id(entity_type_id, permission_id)


Hard deletes a permission. This is a dangerous operation and should not be used in most circumstances. This permanently removes the given permission from all grants that had it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_id** | **String** | The Id of the entityType the the permission belongs to. | [required] |
**permission_id** | **String** | The Id of the permission to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_entity_type_with_id

> delete_entity_type_with_id(entity_type_id)


Deletes the Entity Type for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_id** | **String** | The Id of the Entity Type to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_entity_with_id

> delete_entity_with_id(entity_id, x_fusion_auth_tenant_id)


Deletes the Entity for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | The Id of the Entity to delete. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_form_field_with_id

> delete_form_field_with_id(field_id)


Deletes the form field for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The Id of the form field to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_form_with_id

> delete_form_with_id(form_id)


Deletes the form for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | The Id of the form to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_members_with_id

> delete_group_members_with_id(member_delete_request)


Removes users as members of a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_delete_request** | Option<[**MemberDeleteRequest**](MemberDeleteRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_with_id

> delete_group_with_id(group_id, x_fusion_auth_tenant_id)


Deletes the group for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The Id of the group to delete. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_identity_provider_with_id

> delete_identity_provider_with_id(identity_provider_id)


Deletes the identity provider for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_id** | **String** | The Id of the identity provider to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ip_access_control_list_with_id

> delete_ip_access_control_list_with_id(ip_access_control_list_id)


Deletes the IP Access Control List for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_access_control_list_id** | **String** | The Id of the IP Access Control List to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_jwt_refresh

> delete_jwt_refresh(user_id, application_id, token, refresh_token_revoke_request)


Revoke all refresh tokens that belong to a user by user Id. OR Revoke all refresh tokens that belong to a user by user Id for a specific application by applicationId. OR Revoke all refresh tokens that belong to an application by applicationId. OR Revokes refresh tokens using the information in the JSON body. The handling for this method is the same as the revokeRefreshToken method and is based on the information you provide in the RefreshDeleteRequest object. See that method for additional information. OR Revokes a single refresh token by using the actual refresh token value. This refresh token value is sensitive, so  be careful with this API request. OR Revokes refresh tokens.  Usage examples:   - Delete a single refresh token, pass in only the token.       revokeRefreshToken(token)    - Delete all refresh tokens for a user, pass in only the userId.       revokeRefreshToken(null, userId)    - Delete all refresh tokens for a user for a specific application, pass in both the userId and the applicationId.       revokeRefreshToken(null, userId, applicationId)    - Delete all refresh tokens for an application       revokeRefreshToken(null, null, applicationId)  Note: <code>null</code> may be handled differently depending upon the programming language.  See also: (method names may vary by language... but you'll figure it out)   - revokeRefreshTokenById  - revokeRefreshTokenByToken  - revokeRefreshTokensByUserId  - revokeRefreshTokensByApplicationId  - revokeRefreshTokensByUserIdForApplication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The unique Id of the user that you want to delete all refresh tokens for. |  |
**application_id** | Option<**String**> | The unique Id of the application that you want to delete refresh tokens for. |  |
**token** | Option<**String**> | The refresh token to delete. |  |
**refresh_token_revoke_request** | Option<[**RefreshTokenRevokeRequest**](RefreshTokenRevokeRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_key_with_id

> delete_key_with_id(key_id)


Deletes the key for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The Id of the key to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lambda_with_id

> delete_lambda_with_id(lambda_id)


Deletes the lambda for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lambda_id** | **String** | The Id of the lambda to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_message_template_with_id

> delete_message_template_with_id(message_template_id)


Deletes the message template for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_template_id** | **String** | The Id of the message template to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_messenger_with_id

> delete_messenger_with_id(messenger_id)


Deletes the messenger for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messenger_id** | **String** | The Id of the messenger to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_o_auth_scope_with_id

> delete_o_auth_scope_with_id(application_id, scope_id, x_fusion_auth_tenant_id)


Hard deletes a custom OAuth scope. OAuth workflows that are still requesting the deleted OAuth scope may fail depending on the application's unknown scope policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application that the OAuth scope belongs to. | [required] |
**scope_id** | **String** | The Id of the OAuth scope to delete. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant_with_id

> delete_tenant_with_id(tenant_id, x_fusion_auth_tenant_id, r#async, tenant_delete_request)


Deletes the tenant based on the given Id on the URL. This permanently deletes all information, metrics, reports and data associated with the tenant and everything under the tenant (applications, users, etc). OR Deletes the tenant for the given Id asynchronously. This method is helpful if you do not want to wait for the delete operation to complete. OR Deletes the tenant based on the given request (sent to the API as JSON). This permanently deletes all information, metrics, reports and data associated with the tenant and everything under the tenant (applications, users, etc).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | The Id of the tenant to delete. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**r#async** | Option<**String**> |  |  |
**tenant_delete_request** | Option<[**TenantDeleteRequest**](TenantDeleteRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_theme_with_id

> delete_theme_with_id(theme_id)


Deletes the theme for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme_id** | **String** | The Id of the theme to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_action_reason_with_id

> delete_user_action_reason_with_id(user_action_reason_id)


Deletes the user action reason for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_action_reason_id** | **String** | The Id of the user action reason to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_action_with_id

> delete_user_action_with_id(user_action_id, x_fusion_auth_tenant_id, hard_delete)


Deactivates the user action with the given Id. OR Deletes the user action for the given Id. This permanently deletes the user action and also any history and logs of the action being applied to any users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_action_id** | **String** | The Id of the user action to deactivate. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**hard_delete** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_bulk

> models::UserDeleteResponse delete_user_bulk(user_ids, dry_run, hard_delete, user_delete_request)


Deactivates the users with the given ids. OR Deletes the users with the given ids, or users matching the provided JSON query or queryString. The order of preference is ids, query and then queryString, it is recommended to only provide one of the three for the request.  This method can be used to deactivate or permanently delete (hard-delete) users based upon the hardDelete boolean in the request body. Using the dryRun parameter you may also request the result of the action without actually deleting or deactivating any users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_ids** | Option<**String**> | The ids of the users to deactivate. |  |
**dry_run** | Option<**String**> |  |  |
**hard_delete** | Option<**String**> |  |  |
**user_delete_request** | Option<[**UserDeleteRequest**](UserDeleteRequest.md)> |  |  |

### Return type

[**models::UserDeleteResponse**](UserDeleteResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_link_with_id

> models::IdentityProviderLinkResponse delete_user_link_with_id(identity_provider_id, identity_provider_user_id, user_id)


Remove an existing link that has been made from a 3rd party identity provider to a FusionAuth user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_id** | Option<**String**> | The unique Id of the identity provider. |  |
**identity_provider_user_id** | Option<**String**> | The unique Id of the user in the 3rd party identity provider to unlink. |  |
**user_id** | Option<**String**> | The unique Id of the FusionAuth user to unlink. |  |

### Return type

[**models::IdentityProviderLinkResponse**](IdentityProviderLinkResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_registration_with_id

> delete_user_registration_with_id(user_id, application_id, x_fusion_auth_tenant_id, registration_delete_request)


Deletes the user registration for the given user and application. OR Deletes the user registration for the given user and application along with the given JSON body that contains the event information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user whose registration is being deleted. | [required] |
**application_id** | **String** | The Id of the application to remove the registration for. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**registration_delete_request** | Option<[**RegistrationDeleteRequest**](RegistrationDeleteRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_two_factor_with_id

> delete_user_two_factor_with_id(user_id, method_id, code, two_factor_disable_request)


Disable two-factor authentication for a user using a JSON body rather than URL parameters. OR Disable two-factor authentication for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the User for which you're disabling two-factor authentication. | [required] |
**method_id** | Option<**String**> | The two-factor method identifier you wish to disable |  |
**code** | Option<**String**> | The two-factor code used verify the the caller knows the two-factor secret. |  |
**two_factor_disable_request** | Option<[**TwoFactorDisableRequest**](TwoFactorDisableRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_with_id

> delete_user_with_id(user_id, x_fusion_auth_tenant_id, hard_delete, user_delete_single_request)


Deletes the user based on the given request (sent to the API as JSON). This permanently deletes all information, metrics, reports and data associated with the user. OR Deletes the user for the given Id. This permanently deletes all information, metrics, reports and data associated with the user. OR Deactivates the user with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user to delete (required). | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**hard_delete** | Option<**String**> |  |  |
**user_delete_single_request** | Option<[**UserDeleteSingleRequest**](UserDeleteSingleRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_web_authn_credential_with_id

> delete_web_authn_credential_with_id(id)


Deletes the WebAuthn credential for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The Id of the WebAuthn credential to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook_with_id

> delete_webhook_with_id(webhook_id)


Deletes the webhook for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The Id of the webhook to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_two_factor_with_id

> models::TwoFactorResponse enable_two_factor_with_id(user_id, two_factor_request)


Enable two-factor authentication for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user to enable two-factor authentication. | [required] |
**two_factor_request** | Option<[**TwoFactorRequest**](TwoFactorRequest.md)> |  |  |

### Return type

[**models::TwoFactorResponse**](TwoFactorResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchange_refresh_token_for_jwt_with_id

> models::JwtRefreshResponse exchange_refresh_token_for_jwt_with_id(refresh_request)


Exchange a refresh token for a new JWT.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_request** | Option<[**RefreshRequest**](RefreshRequest.md)> |  |  |

### Return type

[**models::JwtRefreshResponse**](JWTRefreshResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forgot_password_with_id

> models::ForgotPasswordResponse forgot_password_with_id(forgot_password_request)


Begins the forgot password sequence, which kicks off an email to the user so that they can reset their password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**forgot_password_request** | Option<[**ForgotPasswordRequest**](ForgotPasswordRequest.md)> |  |  |

### Return type

[**models::ForgotPasswordResponse**](ForgotPasswordResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_key

> models::KeyResponse generate_key(key_request)


Generate a new RSA or EC key pair or an HMAC secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_request** | Option<[**KeyRequest**](KeyRequest.md)> |  |  |

### Return type

[**models::KeyResponse**](KeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_key_with_id

> models::KeyResponse generate_key_with_id(key_id, key_request)


Generate a new RSA or EC key pair or an HMAC secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The Id for the key. If not provided a secure random UUID will be generated. | [required] |
**key_request** | Option<[**KeyRequest**](KeyRequest.md)> |  |  |

### Return type

[**models::KeyResponse**](KeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_two_factor_recovery_codes_with_id

> models::TwoFactorRecoveryCodeResponse generate_two_factor_recovery_codes_with_id(user_id)


Generate two-factor recovery codes for a user. Generating two-factor recovery codes will invalidate any existing recovery codes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user to generate new Two Factor recovery codes. | [required] |

### Return type

[**models::TwoFactorRecoveryCodeResponse**](TwoFactorRecoveryCodeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_two_factor_secret_using_jwt_with_id

> models::SecretResponse generate_two_factor_secret_using_jwt_with_id()


Generate a Two Factor secret that can be used to enable Two Factor authentication for a User. The response will contain both the secret and a Base32 encoded form of the secret which can be shown to a User when using a 2 Step Authentication application such as Google Authenticator.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SecretResponse**](SecretResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identity_provider_login_with_id

> models::LoginResponse identity_provider_login_with_id(x_fusion_auth_tenant_id, identity_provider_login_request)


Handles login via third-parties including Social login, external OAuth and OpenID Connect, and other login systems.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**identity_provider_login_request** | Option<[**IdentityProviderLoginRequest**](IdentityProviderLoginRequest.md)> |  |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_key

> models::KeyResponse import_key(key_request)


Import an existing RSA or EC key pair or an HMAC secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_request** | Option<[**KeyRequest**](KeyRequest.md)> |  |  |

### Return type

[**models::KeyResponse**](KeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_key_with_id

> models::KeyResponse import_key_with_id(key_id, key_request)


Import an existing RSA or EC key pair or an HMAC secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The Id for the key. If not provided a secure random UUID will be generated. | [required] |
**key_request** | Option<[**KeyRequest**](KeyRequest.md)> |  |  |

### Return type

[**models::KeyResponse**](KeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_refresh_tokens_with_id

> import_refresh_tokens_with_id(refresh_token_import_request)


Bulk imports refresh tokens. This request performs minimal validation and runs batch inserts of refresh tokens with the expectation that each token represents a user that already exists and is registered for the corresponding FusionAuth Application. This is done to increases the insert performance.  Therefore, if you encounter an error due to a database key violation, the response will likely offer a generic explanation. If you encounter an error, you may optionally enable additional validation to receive a JSON response body with specific validation errors. This will slow the request down but will allow you to identify the cause of the failure. See the validateDbConstraints request parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_token_import_request** | Option<[**RefreshTokenImportRequest**](RefreshTokenImportRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_users_with_id

> import_users_with_id(import_request)


Bulk imports users. This request performs minimal validation and runs batch inserts of users with the expectation that each user does not yet exist and each registration corresponds to an existing FusionAuth Application. This is done to increases the insert performance.  Therefore, if you encounter an error due to a database key violation, the response will likely offer a generic explanation. If you encounter an error, you may optionally enable additional validation to receive a JSON response body with specific validation errors. This will slow the request down but will allow you to identify the cause of the failure. See the validateDbConstraints request parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_request** | Option<[**ImportRequest**](ImportRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_web_authn_credential_with_id

> import_web_authn_credential_with_id(web_authn_credential_import_request)


Import a WebAuthn credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_authn_credential_import_request** | Option<[**WebAuthnCredentialImportRequest**](WebAuthnCredentialImportRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_jwt_with_id

> models::IssueResponse issue_jwt_with_id(application_id, refresh_token)


Issue a new access token (JWT) for the requested Application after ensuring the provided JWT is valid. A valid access token is properly signed and not expired. <p> This API may be used in an SSO configuration to issue new tokens for another application after the user has obtained a valid token from authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | Option<**String**> | The Application Id for which you are requesting a new access token be issued. |  |
**refresh_token** | Option<**String**> | An existing refresh token used to request a refresh token in addition to a JWT in the response. <p>The target application represented by the applicationId request parameter must have refresh tokens enabled in order to receive a refresh token in the response.</p> |  |

### Return type

[**models::IssueResponse**](IssueResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_ping_with_id

> models::LoginResponse login_ping_with_id(user_id, application_id, caller_ip_address, x_fusion_auth_tenant_id)


Sends a ping to FusionAuth indicating that the user was automatically logged into an application. When using FusionAuth's SSO or your own, you should call this if the user is already logged in centrally, but accesses an application where they no longer have a session. This helps correctly track login counts, times and helps with reporting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user that was logged in. | [required] |
**application_id** | **String** | The Id of the application that they logged into. | [required] |
**caller_ip_address** | Option<**String**> | The IP address of the end-user that is logging in. If a null value is provided the IP address will be that of the client or last proxy that sent the request. |  |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_ping_with_request_with_id

> models::LoginResponse login_ping_with_request_with_id(x_fusion_auth_tenant_id, login_ping_request)


Sends a ping to FusionAuth indicating that the user was automatically logged into an application. When using FusionAuth's SSO or your own, you should call this if the user is already logged in centrally, but accesses an application where they no longer have a session. This helps correctly track login counts, times and helps with reporting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**login_ping_request** | Option<[**LoginPingRequest**](LoginPingRequest.md)> |  |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_with_id

> models::LoginResponse login_with_id(x_fusion_auth_tenant_id, login_request)


Authenticates a user to FusionAuth.   This API optionally requires an API key. See <code>Application.loginConfiguration.requireAuthentication</code>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**login_request** | Option<[**LoginRequest**](LoginRequest.md)> |  |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_identity_provider_with_id

> models::LookupResponse lookup_identity_provider_with_id(domain)


Retrieves the identity provider for the given domain. A 200 response code indicates the domain is managed by a registered identity provider. A 404 indicates the domain is not managed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | Option<**String**> | The domain or email address to lookup. |  |

### Return type

[**models::LookupResponse**](LookupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_action_with_id

> models::ActionResponse modify_action_with_id(action_id, action_request)


Modifies a temporal user action by changing the expiration of the action and optionally adding a comment to the action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | The Id of the action to modify. This is technically the user action log id. | [required] |
**action_request** | Option<[**ActionRequest**](ActionRequest.md)> |  |  |

### Return type

[**models::ActionResponse**](ActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## passwordless_login_with_id

> models::LoginResponse passwordless_login_with_id(passwordless_login_request)


Complete a login request using a passwordless code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**passwordless_login_request** | Option<[**PasswordlessLoginRequest**](PasswordlessLoginRequest.md)> |  |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_application_role_with_id

> models::ApplicationResponse patch_application_role_with_id(application_id, role_id, x_fusion_auth_tenant_id, application_request)


Updates, via PATCH, the application role with the given Id for the application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application that the role belongs to. | [required] |
**role_id** | **String** | The Id of the role to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**application_request** | Option<[**ApplicationRequest**](ApplicationRequest.md)> |  |  |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_application_with_id

> models::ApplicationResponse patch_application_with_id(application_id, x_fusion_auth_tenant_id, application_request)


Updates, via PATCH, the application with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**application_request** | Option<[**ApplicationRequest**](ApplicationRequest.md)> |  |  |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_connector_with_id

> models::ConnectorResponse patch_connector_with_id(connector_id, connector_request)


Updates, via PATCH, the connector with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** | The Id of the connector to update. | [required] |
**connector_request** | Option<[**ConnectorRequest**](ConnectorRequest.md)> |  |  |

### Return type

[**models::ConnectorResponse**](ConnectorResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_consent_with_id

> models::ConsentResponse patch_consent_with_id(consent_id, x_fusion_auth_tenant_id, consent_request)


Updates, via PATCH, the consent with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | The Id of the consent to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**consent_request** | Option<[**ConsentRequest**](ConsentRequest.md)> |  |  |

### Return type

[**models::ConsentResponse**](ConsentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_email_template_with_id

> models::EmailTemplateResponse patch_email_template_with_id(email_template_id, x_fusion_auth_tenant_id, email_template_request)


Updates, via PATCH, the email template with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_id** | **String** | The Id of the email template to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**email_template_request** | Option<[**EmailTemplateRequest**](EmailTemplateRequest.md)> |  |  |

### Return type

[**models::EmailTemplateResponse**](EmailTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_entity_type_permission_with_id

> models::EntityTypeResponse patch_entity_type_permission_with_id(entity_type_id, permission_id, entity_type_request)


Patches the permission with the given Id for the entity type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_id** | **String** | The Id of the entityType that the permission belongs to. | [required] |
**permission_id** | **String** | The Id of the permission to patch. | [required] |
**entity_type_request** | Option<[**EntityTypeRequest**](EntityTypeRequest.md)> |  |  |

### Return type

[**models::EntityTypeResponse**](EntityTypeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_entity_type_with_id

> models::EntityTypeResponse patch_entity_type_with_id(entity_type_id, entity_type_request)


Updates, via PATCH, the Entity Type with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_id** | **String** | The Id of the Entity Type to update. | [required] |
**entity_type_request** | Option<[**EntityTypeRequest**](EntityTypeRequest.md)> |  |  |

### Return type

[**models::EntityTypeResponse**](EntityTypeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_entity_with_id

> models::EntityResponse patch_entity_with_id(entity_id, x_fusion_auth_tenant_id, entity_request)


Updates, via PATCH, the Entity with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | The Id of the Entity Type to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**entity_request** | Option<[**EntityRequest**](EntityRequest.md)> |  |  |

### Return type

[**models::EntityResponse**](EntityResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_form_field_with_id

> models::FormFieldResponse patch_form_field_with_id(field_id, form_field_request)


Patches the form field with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The Id of the form field to patch. | [required] |
**form_field_request** | Option<[**FormFieldRequest**](FormFieldRequest.md)> |  |  |

### Return type

[**models::FormFieldResponse**](FormFieldResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_form_with_id

> models::FormResponse patch_form_with_id(form_id, form_request)


Patches the form with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | The Id of the form to patch. | [required] |
**form_request** | Option<[**FormRequest**](FormRequest.md)> |  |  |

### Return type

[**models::FormResponse**](FormResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_group_with_id

> models::GroupResponse patch_group_with_id(group_id, x_fusion_auth_tenant_id, group_request)


Updates, via PATCH, the group with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The Id of the group to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**group_request** | Option<[**GroupRequest**](GroupRequest.md)> |  |  |

### Return type

[**models::GroupResponse**](GroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_identity_provider_with_id

> models::IdentityProviderResponse patch_identity_provider_with_id(identity_provider_id, identity_provider_request)


Updates, via PATCH, the identity provider with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_id** | **String** | The Id of the identity provider to update. | [required] |
**identity_provider_request** | Option<[**IdentityProviderRequest**](IdentityProviderRequest.md)> |  |  |

### Return type

[**models::IdentityProviderResponse**](IdentityProviderResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_integrations_with_id

> models::IntegrationResponse patch_integrations_with_id(integration_request)


Updates, via PATCH, the available integrations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_request** | Option<[**IntegrationRequest**](IntegrationRequest.md)> |  |  |

### Return type

[**models::IntegrationResponse**](IntegrationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_ip_access_control_list_with_id

> models::IpAccessControlListResponse patch_ip_access_control_list_with_id(access_control_list_id, ip_access_control_list_request)


Update the IP Access Control List with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_control_list_id** | **String** | The Id of the IP Access Control List to patch. | [required] |
**ip_access_control_list_request** | Option<[**IpAccessControlListRequest**](IpAccessControlListRequest.md)> |  |  |

### Return type

[**models::IpAccessControlListResponse**](IPAccessControlListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lambda_with_id

> models::LambdaResponse patch_lambda_with_id(lambda_id, lambda_request)


Updates, via PATCH, the lambda with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lambda_id** | **String** | The Id of the lambda to update. | [required] |
**lambda_request** | Option<[**LambdaRequest**](LambdaRequest.md)> |  |  |

### Return type

[**models::LambdaResponse**](LambdaResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_message_template_with_id

> models::MessageTemplateResponse patch_message_template_with_id(message_template_id, message_template_request)


Updates, via PATCH, the message template with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_template_id** | **String** | The Id of the message template to update. | [required] |
**message_template_request** | Option<[**MessageTemplateRequest**](MessageTemplateRequest.md)> |  |  |

### Return type

[**models::MessageTemplateResponse**](MessageTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_messenger_with_id

> models::MessengerResponse patch_messenger_with_id(messenger_id, messenger_request)


Updates, via PATCH, the messenger with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messenger_id** | **String** | The Id of the messenger to update. | [required] |
**messenger_request** | Option<[**MessengerRequest**](MessengerRequest.md)> |  |  |

### Return type

[**models::MessengerResponse**](MessengerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_o_auth_scope_with_id

> models::ApplicationOAuthScopeResponse patch_o_auth_scope_with_id(application_id, scope_id, x_fusion_auth_tenant_id, application_o_auth_scope_request)


Updates, via PATCH, the custom OAuth scope with the given Id for the application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application that the OAuth scope belongs to. | [required] |
**scope_id** | **String** | The Id of the OAuth scope to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**application_o_auth_scope_request** | Option<[**ApplicationOAuthScopeRequest**](ApplicationOAuthScopeRequest.md)> |  |  |

### Return type

[**models::ApplicationOAuthScopeResponse**](ApplicationOAuthScopeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_registration_with_id

> models::RegistrationResponse patch_registration_with_id(user_id, x_fusion_auth_tenant_id, registration_request)


Updates, via PATCH, the registration for the user with the given Id and the application defined in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user whose registration is going to be updated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**registration_request** | Option<[**RegistrationRequest**](RegistrationRequest.md)> |  |  |

### Return type

[**models::RegistrationResponse**](RegistrationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_system_configuration_with_id

> models::SystemConfigurationResponse patch_system_configuration_with_id(system_configuration_request)


Updates, via PATCH, the system configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_configuration_request** | Option<[**SystemConfigurationRequest**](SystemConfigurationRequest.md)> |  |  |

### Return type

[**models::SystemConfigurationResponse**](SystemConfigurationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_tenant_with_id

> models::TenantResponse patch_tenant_with_id(tenant_id, x_fusion_auth_tenant_id, tenant_request)


Updates, via PATCH, the tenant with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | The Id of the tenant to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**tenant_request** | Option<[**TenantRequest**](TenantRequest.md)> |  |  |

### Return type

[**models::TenantResponse**](TenantResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_theme_with_id

> models::ThemeResponse patch_theme_with_id(theme_id, theme_request)


Updates, via PATCH, the theme with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme_id** | **String** | The Id of the theme to update. | [required] |
**theme_request** | Option<[**ThemeRequest**](ThemeRequest.md)> |  |  |

### Return type

[**models::ThemeResponse**](ThemeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_action_reason_with_id

> models::UserActionReasonResponse patch_user_action_reason_with_id(user_action_reason_id, user_action_reason_request)


Updates, via PATCH, the user action reason with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_action_reason_id** | **String** | The Id of the user action reason to update. | [required] |
**user_action_reason_request** | Option<[**UserActionReasonRequest**](UserActionReasonRequest.md)> |  |  |

### Return type

[**models::UserActionReasonResponse**](UserActionReasonResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_action_with_id

> models::UserActionResponse patch_user_action_with_id(user_action_id, x_fusion_auth_tenant_id, user_action_request)


Updates, via PATCH, the user action with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_action_id** | **String** | The Id of the user action to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**user_action_request** | Option<[**UserActionRequest**](UserActionRequest.md)> |  |  |

### Return type

[**models::UserActionResponse**](UserActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_consent_with_id

> models::UserConsentResponse patch_user_consent_with_id(user_consent_id, user_consent_request)


Updates, via PATCH, a single User consent by Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_consent_id** | **String** | The User Consent Id | [required] |
**user_consent_request** | Option<[**UserConsentRequest**](UserConsentRequest.md)> |  |  |

### Return type

[**models::UserConsentResponse**](UserConsentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_with_id

> models::UserResponse patch_user_with_id(user_id, x_fusion_auth_tenant_id, user_request)


Updates, via PATCH, the user with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**user_request** | Option<[**UserRequest**](UserRequest.md)> |  |  |

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_webhook_with_id

> models::WebhookResponse patch_webhook_with_id(webhook_id, webhook_request)


Patches the webhook with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The Id of the webhook to update. | [required] |
**webhook_request** | Option<[**WebhookRequest**](WebhookRequest.md)> |  |  |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reconcile_jwt_with_id

> models::LoginResponse reconcile_jwt_with_id(identity_provider_login_request)


Reconcile a User to FusionAuth using JWT issued from another Identity Provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_login_request** | Option<[**IdentityProviderLoginRequest**](IdentityProviderLoginRequest.md)> |  |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register

> models::RegistrationResponse register(x_fusion_auth_tenant_id, registration_request)


Registers a user for an application. If you provide the User and the UserRegistration object on this request, it will create the user as well as register them for the application. This is called a Full Registration. However, if you only provide the UserRegistration object, then the user must already exist and they will be registered for the application. The user Id can also be provided and it will either be used to look up an existing user or it will be used for the newly created User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**registration_request** | Option<[**RegistrationRequest**](RegistrationRequest.md)> |  |  |

### Return type

[**models::RegistrationResponse**](RegistrationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_with_id

> models::RegistrationResponse register_with_id(user_id, x_fusion_auth_tenant_id, registration_request)


Registers a user for an application. If you provide the User and the UserRegistration object on this request, it will create the user as well as register them for the application. This is called a Full Registration. However, if you only provide the UserRegistration object, then the user must already exist and they will be registered for the application. The user Id can also be provided and it will either be used to look up an existing user or it will be used for the newly created User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user being registered for the application and optionally created. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**registration_request** | Option<[**RegistrationRequest**](RegistrationRequest.md)> |  |  |

### Return type

[**models::RegistrationResponse**](RegistrationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reindex_with_id

> reindex_with_id(reindex_request)


Requests Elasticsearch to delete and rebuild the index for FusionAuth users or entities. Be very careful when running this request as it will  increase the CPU and I/O load on your database until the operation completes. Generally speaking you do not ever need to run this operation unless  instructed by FusionAuth support, or if you are migrating a database another system and you are not brining along the Elasticsearch index.   You have been warned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reindex_request** | Option<[**ReindexRequest**](ReindexRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_family_with_id

> remove_user_from_family_with_id(family_id, user_id, x_fusion_auth_tenant_id)


Removes a user from the family with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**family_id** | **String** | The Id of the family to remove the user from. | [required] |
**user_id** | **String** | The Id of the user to remove from the family. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_action_with_id

> models::ActionResponse retrieve_action_with_id(action_id)


Retrieves a single action log (the log of a user action that was taken on a user previously) for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | The Id of the action to retrieve. | [required] |

### Return type

[**models::ActionResponse**](ActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_api_key_with_id

> models::ApiKeyResponse retrieve_api_key_with_id(key_id)


Retrieves an authentication API key for the given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The Id of the API key to retrieve. | [required] |

### Return type

[**models::ApiKeyResponse**](APIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_application

> models::ApplicationResponse retrieve_application(x_fusion_auth_tenant_id, inactive)


Retrieves the application for the given Id or all the applications if the Id is null. OR Retrieves all the applications that are currently inactive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**inactive** | Option<**String**> |  |  |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_application_with_id

> models::ApplicationResponse retrieve_application_with_id(application_id, x_fusion_auth_tenant_id)


Retrieves the application for the given Id or all the applications if the Id is null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The application id. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_audit_log_with_id

> models::AuditLogResponse retrieve_audit_log_with_id(audit_log_id)


Retrieves a single audit log for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audit_log_id** | **String** | The Id of the audit log to retrieve. | [required] |

### Return type

[**models::AuditLogResponse**](AuditLogResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_connector_with_id

> models::ConnectorResponse retrieve_connector_with_id(connector_id)


Retrieves the connector with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** | The Id of the connector. | [required] |

### Return type

[**models::ConnectorResponse**](ConnectorResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_consent_with_id

> models::ConsentResponse retrieve_consent_with_id(consent_id, x_fusion_auth_tenant_id)


Retrieves the Consent for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | The Id of the consent. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::ConsentResponse**](ConsentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_daily_active_report_with_id

> models::DailyActiveUserReportResponse retrieve_daily_active_report_with_id(application_id, start, end)


Retrieves the daily active user report between the two instants. If you specify an application id, it will only return the daily active counts for that application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | Option<**String**> | The application id. |  |
**start** | Option<**String**> | The start instant as UTC milliseconds since Epoch. |  |
**end** | Option<**String**> | The end instant as UTC milliseconds since Epoch. |  |

### Return type

[**models::DailyActiveUserReportResponse**](DailyActiveUserReportResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_device_user_code

> retrieve_device_user_code()


Retrieve a user_code that is part of an in-progress Device Authorization Grant.  This API is useful if you want to build your own login workflow to complete a device grant.  This request will require an API key. OR Retrieve a user_code that is part of an in-progress Device Authorization Grant.  This API is useful if you want to build your own login workflow to complete a device grant.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_email_template

> models::EmailTemplateResponse retrieve_email_template(x_fusion_auth_tenant_id)


Retrieves the email template for the given Id. If you don't specify the id, this will return all the email templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::EmailTemplateResponse**](EmailTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_email_template_preview_with_id

> models::PreviewResponse retrieve_email_template_preview_with_id(preview_request)


Creates a preview of the email template provided in the request. This allows you to preview an email template that hasn't been saved to the database yet. The entire email template does not need to be provided on the request. This will create the preview based on whatever is given.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preview_request** | Option<[**PreviewRequest**](PreviewRequest.md)> |  |  |

### Return type

[**models::PreviewResponse**](PreviewResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_email_template_with_id

> models::EmailTemplateResponse retrieve_email_template_with_id(email_template_id, x_fusion_auth_tenant_id)


Retrieves the email template for the given Id. If you don't specify the id, this will return all the email templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_id** | **String** | The Id of the email template. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::EmailTemplateResponse**](EmailTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_entity_grant_with_id

> models::EntityGrantResponse retrieve_entity_grant_with_id(entity_id, recipient_entity_id, user_id, x_fusion_auth_tenant_id)


Retrieves an Entity Grant for the given Entity and User/Entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | The Id of the Entity. | [required] |
**recipient_entity_id** | Option<**String**> | The Id of the Entity that the Entity Grant is for. |  |
**user_id** | Option<**String**> | The Id of the User that the Entity Grant is for. |  |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::EntityGrantResponse**](EntityGrantResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_entity_type_with_id

> models::EntityTypeResponse retrieve_entity_type_with_id(entity_type_id)


Retrieves the Entity Type for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_id** | **String** | The Id of the Entity Type. | [required] |

### Return type

[**models::EntityTypeResponse**](EntityTypeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_entity_with_id

> models::EntityResponse retrieve_entity_with_id(entity_id, x_fusion_auth_tenant_id)


Retrieves the Entity for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | The Id of the Entity. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::EntityResponse**](EntityResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_event_log_with_id

> models::EventLogResponse retrieve_event_log_with_id(event_log_id)


Retrieves a single event log for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_log_id** | **String** | The Id of the event log to retrieve. | [required] |

### Return type

[**models::EventLogResponse**](EventLogResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_families_with_id

> models::FamilyResponse retrieve_families_with_id(user_id, x_fusion_auth_tenant_id)


Retrieves all the families that a user belongs to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The User's id |  |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::FamilyResponse**](FamilyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_family_members_by_family_id_with_id

> models::FamilyResponse retrieve_family_members_by_family_id_with_id(family_id, x_fusion_auth_tenant_id)


Retrieves all the members of a family by the unique Family Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**family_id** | **String** | The unique Id of the Family. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::FamilyResponse**](FamilyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_form_field_with_id

> models::FormFieldResponse retrieve_form_field_with_id(field_id)


Retrieves the form field with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The Id of the form field. | [required] |

### Return type

[**models::FormFieldResponse**](FormFieldResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_form_with_id

> models::FormResponse retrieve_form_with_id(form_id)


Retrieves the form with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | The Id of the form. | [required] |

### Return type

[**models::FormResponse**](FormResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_group_with_id

> models::GroupResponse retrieve_group_with_id(group_id, x_fusion_auth_tenant_id)


Retrieves the group for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The Id of the group. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::GroupResponse**](GroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_identity_provider_by_type_with_id

> models::IdentityProviderResponse retrieve_identity_provider_by_type_with_id(r#type)


Retrieves one or more identity provider for the given type. For types such as Google, Facebook, Twitter and LinkedIn, only a single  identity provider can exist. For types such as OpenID Connect and SAMLv2 more than one identity provider can be configured so this request  may return multiple identity providers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The type of the identity provider. |  |

### Return type

[**models::IdentityProviderResponse**](IdentityProviderResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_identity_provider_link

> models::IdentityProviderLinkResponse retrieve_identity_provider_link(identity_provider_id, identity_provider_user_id, user_id)


Retrieve a single Identity Provider user (link). OR Retrieve all Identity Provider users (links) for the user. Specify the optional identityProviderId to retrieve links for a particular IdP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_id** | Option<**String**> | The unique Id of the identity provider. |  |
**identity_provider_user_id** | Option<**String**> | The unique Id of the user in the 3rd party identity provider. |  |
**user_id** | Option<**String**> | The unique Id of the FusionAuth user. |  |

### Return type

[**models::IdentityProviderLinkResponse**](IdentityProviderLinkResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_identity_provider_with_id

> models::IdentityProviderResponse retrieve_identity_provider_with_id(identity_provider_id)


Retrieves the identity provider for the given Id or all the identity providers if the Id is null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_id** | **String** | The identity provider Id. | [required] |

### Return type

[**models::IdentityProviderResponse**](IdentityProviderResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_ip_access_control_list_with_id

> models::IpAccessControlListResponse retrieve_ip_access_control_list_with_id(ip_access_control_list_id)


Retrieves the IP Access Control List with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_access_control_list_id** | **String** | The Id of the IP Access Control List. | [required] |

### Return type

[**models::IpAccessControlListResponse**](IPAccessControlListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_json_web_key_set_with_id

> models::JwksResponse retrieve_json_web_key_set_with_id()


Returns public keys used by FusionAuth to cryptographically verify JWTs using the JSON Web Key format.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::JwksResponse**](JWKSResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_jwt_public_key

> models::PublicKeyResponse retrieve_jwt_public_key(key_id, application_id)


Retrieves the Public Key configured for verifying JSON Web Tokens (JWT) by the key Id (kid). OR Retrieves the Public Key configured for verifying the JSON Web Tokens (JWT) issued by the Login API by the Application Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | Option<**String**> | The Id of the public key (kid). |  |
**application_id** | Option<**String**> | The Id of the Application for which this key is used. |  |

### Return type

[**models::PublicKeyResponse**](PublicKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_key_with_id

> models::KeyResponse retrieve_key_with_id(key_id)


Retrieves the key for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The Id of the key. | [required] |

### Return type

[**models::KeyResponse**](KeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_keys_with_id

> models::KeyResponse retrieve_keys_with_id()


Retrieves all the keys.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::KeyResponse**](KeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_lambda_with_id

> models::LambdaResponse retrieve_lambda_with_id(lambda_id)


Retrieves the lambda for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lambda_id** | **String** | The Id of the lambda. | [required] |

### Return type

[**models::LambdaResponse**](LambdaResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_lambdas_by_type_with_id

> models::LambdaResponse retrieve_lambdas_by_type_with_id(r#type)


Retrieves all the lambdas for the provided type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The type of the lambda to return. |  |

### Return type

[**models::LambdaResponse**](LambdaResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_message_template

> models::MessageTemplateResponse retrieve_message_template()


Retrieves the message template for the given Id. If you don't specify the id, this will return all the message templates.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MessageTemplateResponse**](MessageTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_message_template_preview_with_id

> models::PreviewMessageTemplateResponse retrieve_message_template_preview_with_id(preview_message_template_request)


Creates a preview of the message template provided in the request, normalized to a given locale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preview_message_template_request** | Option<[**PreviewMessageTemplateRequest**](PreviewMessageTemplateRequest.md)> |  |  |

### Return type

[**models::PreviewMessageTemplateResponse**](PreviewMessageTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_message_template_with_id

> models::MessageTemplateResponse retrieve_message_template_with_id(message_template_id)


Retrieves the message template for the given Id. If you don't specify the id, this will return all the message templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_template_id** | **String** | The Id of the message template. | [required] |

### Return type

[**models::MessageTemplateResponse**](MessageTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_messenger_with_id

> models::MessengerResponse retrieve_messenger_with_id(messenger_id)


Retrieves the messenger with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messenger_id** | **String** | The Id of the messenger. | [required] |

### Return type

[**models::MessengerResponse**](MessengerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_monthly_active_report_with_id

> models::MonthlyActiveUserReportResponse retrieve_monthly_active_report_with_id(application_id, start, end)


Retrieves the monthly active user report between the two instants. If you specify an application id, it will only return the monthly active counts for that application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | Option<**String**> | The application id. |  |
**start** | Option<**String**> | The start instant as UTC milliseconds since Epoch. |  |
**end** | Option<**String**> | The end instant as UTC milliseconds since Epoch. |  |

### Return type

[**models::MonthlyActiveUserReportResponse**](MonthlyActiveUserReportResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_o_auth_scope_with_id

> models::ApplicationOAuthScopeResponse retrieve_o_auth_scope_with_id(application_id, scope_id, x_fusion_auth_tenant_id)


Retrieves a custom OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application that the OAuth scope belongs to. | [required] |
**scope_id** | **String** | The Id of the OAuth scope to retrieve. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::ApplicationOAuthScopeResponse**](ApplicationOAuthScopeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_oauth_configuration_with_id

> models::OAuthConfigurationResponse retrieve_oauth_configuration_with_id(application_id, x_fusion_auth_tenant_id)


Retrieves the Oauth2 configuration for the application for the given Application Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the Application to retrieve OAuth configuration. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::OAuthConfigurationResponse**](OAuthConfigurationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_open_id_configuration_with_id

> models::OpenIdConfiguration retrieve_open_id_configuration_with_id()


Returns the well known OpenID Configuration JSON document

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OpenIdConfiguration**](OpenIdConfiguration.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_password_validation_rules_with_id

> models::PasswordValidationRulesResponse retrieve_password_validation_rules_with_id()


Retrieves the password validation rules for a specific tenant. This method requires a tenantId to be provided  through the use of a Tenant scoped API key or an HTTP header X-FusionAuth-TenantId to specify the Tenant Id.  This API does not require an API key.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PasswordValidationRulesResponse**](PasswordValidationRulesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_password_validation_rules_with_tenant_id_with_id

> models::PasswordValidationRulesResponse retrieve_password_validation_rules_with_tenant_id_with_id(tenant_id)


Retrieves the password validation rules for a specific tenant.  This API does not require an API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | The Id of the tenant. | [required] |

### Return type

[**models::PasswordValidationRulesResponse**](PasswordValidationRulesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_pending_children_with_id

> models::PendingResponse retrieve_pending_children_with_id(parent_email)


Retrieves all the children for the given parent email address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_email** | Option<**String**> | The email of the parent. |  |

### Return type

[**models::PendingResponse**](PendingResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_pending_link_with_id

> models::IdentityProviderPendingLinkResponse retrieve_pending_link_with_id(pending_link_id, user_id)


Retrieve a pending identity provider link. This is useful to validate a pending link and retrieve meta-data about the identity provider link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pending_link_id** | **String** | The pending link Id. | [required] |
**user_id** | Option<**String**> | The optional userId. When provided additional meta-data will be provided to identify how many links if any the user already has. |  |

### Return type

[**models::IdentityProviderPendingLinkResponse**](IdentityProviderPendingLinkResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_reactor_metrics_with_id

> models::ReactorMetricsResponse retrieve_reactor_metrics_with_id()


Retrieves the FusionAuth Reactor metrics.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ReactorMetricsResponse**](ReactorMetricsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_refresh_token_by_id_with_id

> models::RefreshTokenResponse retrieve_refresh_token_by_id_with_id(token_id)


Retrieves a single refresh token by unique Id. This is not the same thing as the string value of the refresh token. If you have that, you already have what you need.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | The Id of the token. | [required] |

### Return type

[**models::RefreshTokenResponse**](RefreshTokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_refresh_tokens_with_id

> models::RefreshTokenResponse retrieve_refresh_tokens_with_id(user_id)


Retrieves the refresh tokens that belong to the user with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The Id of the user. |  |

### Return type

[**models::RefreshTokenResponse**](RefreshTokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_registration_report_with_id

> models::RegistrationReportResponse retrieve_registration_report_with_id(application_id, start, end)


Retrieves the registration report between the two instants. If you specify an application id, it will only return the registration counts for that application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | Option<**String**> | The application id. |  |
**start** | Option<**String**> | The start instant as UTC milliseconds since Epoch. |  |
**end** | Option<**String**> | The end instant as UTC milliseconds since Epoch. |  |

### Return type

[**models::RegistrationReportResponse**](RegistrationReportResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_registration_with_id

> models::RegistrationResponse retrieve_registration_with_id(user_id, application_id, x_fusion_auth_tenant_id)


Retrieves the user registration for the user with the given Id and the given application id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user. | [required] |
**application_id** | **String** | The Id of the application. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::RegistrationResponse**](RegistrationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_report_login

> models::LoginReportResponse retrieve_report_login(application_id, start, end, user_id, login_id)


Retrieves the login report between the two instants. If you specify an application id, it will only return the login counts for that application. OR Retrieves the login report between the two instants for a particular user by Id. If you specify an application id, it will only return the login counts for that application. OR Retrieves the login report between the two instants for a particular user by login Id. If you specify an application id, it will only return the login counts for that application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | Option<**String**> | The application id. |  |
**start** | Option<**String**> | The start instant as UTC milliseconds since Epoch. |  |
**end** | Option<**String**> | The end instant as UTC milliseconds since Epoch. |  |
**user_id** | Option<**String**> | The userId id. |  |
**login_id** | Option<**String**> | The userId id. |  |

### Return type

[**models::LoginReportResponse**](LoginReportResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_status

> serde_json::Value retrieve_status()


Retrieves the FusionAuth system status. This request is anonymous and does not require an API key. When an API key is not provided the response will contain a single value in the JSON response indicating the current health check. OR Retrieves the FusionAuth system status using an API key. Using an API key will cause the response to include the product version, health checks and various runtime metrics.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_system_health_with_id

> retrieve_system_health_with_id()


Retrieves the FusionAuth system health. This API will return 200 if the system is healthy, and 500 if the system is un-healthy.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_tenant_with_id

> models::TenantResponse retrieve_tenant_with_id(tenant_id, x_fusion_auth_tenant_id)


Retrieves the tenant for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | The Id of the tenant. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::TenantResponse**](TenantResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_theme_with_id

> models::ThemeResponse retrieve_theme_with_id(theme_id)


Retrieves the theme for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme_id** | **String** | The Id of the theme. | [required] |

### Return type

[**models::ThemeResponse**](ThemeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_total_report_with_id

> models::TotalsReportResponse retrieve_total_report_with_id()


Retrieves the totals report. This contains all the total counts for each application and the global registration count.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TotalsReportResponse**](TotalsReportResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_two_factor_recovery_codes_with_id

> models::TwoFactorRecoveryCodeResponse retrieve_two_factor_recovery_codes_with_id(user_id)


Retrieve two-factor recovery codes for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user to retrieve Two Factor recovery codes. | [required] |

### Return type

[**models::TwoFactorRecoveryCodeResponse**](TwoFactorRecoveryCodeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_two_factor_status_with_id

> models::TwoFactorStatusResponse retrieve_two_factor_status_with_id(two_factor_trust_id, user_id, application_id)


Retrieve a user's two-factor status.  This can be used to see if a user will need to complete a two-factor challenge to complete a login, and optionally identify the state of the two-factor trust across various applications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_trust_id** | **String** | The optional two-factor trust Id to verify. | [required] |
**user_id** | Option<**String**> | The user Id to retrieve the Two-Factor status. |  |
**application_id** | Option<**String**> | The optional applicationId to verify. |  |

### Return type

[**models::TwoFactorStatusResponse**](TwoFactorStatusResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user

> models::UserResponse retrieve_user(username, x_fusion_auth_tenant_id, verification_id, change_password_id, email, login_id)


Retrieves the user for the given username. OR Retrieves the user by a verificationId. The intended use of this API is to retrieve a user after the forgot password workflow has been initiated and you may not know the user's email or username. OR Retrieves the user by a change password Id. The intended use of this API is to retrieve a user after the forgot password workflow has been initiated and you may not know the user's email or username. OR Retrieves the user for the given Id. This method does not use an API key, instead it uses a JSON Web Token (JWT) for authentication. OR Retrieves the user for the given email. OR Retrieves the user for the loginId. The loginId can be either the username or the email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> | The username of the user. |  |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**verification_id** | Option<**String**> | The unique verification Id that has been set on the user object. |  |
**change_password_id** | Option<**String**> | The unique change password Id that was sent via email or returned by the Forgot Password API. |  |
**email** | Option<**String**> | The email of the user. |  |
**login_id** | Option<**String**> | The email or username of the user. |  |

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_action

> models::UserActionResponse retrieve_user_action(inactive, x_fusion_auth_tenant_id)


Retrieves all the user actions that are currently inactive. OR Retrieves the user action for the given Id. If you pass in null for the id, this will return all the user actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inactive** | Option<**String**> |  |  |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::UserActionResponse**](UserActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_action_reason

> models::UserActionReasonResponse retrieve_user_action_reason()


Retrieves the user action reason for the given Id. If you pass in null for the id, this will return all the user action reasons.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserActionReasonResponse**](UserActionReasonResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_action_reason_with_id

> models::UserActionReasonResponse retrieve_user_action_reason_with_id(user_action_reason_id)


Retrieves the user action reason for the given Id. If you pass in null for the id, this will return all the user action reasons.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_action_reason_id** | **String** | The Id of the user action reason. | [required] |

### Return type

[**models::UserActionReasonResponse**](UserActionReasonResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_action_with_id

> models::UserActionResponse retrieve_user_action_with_id(user_action_id, x_fusion_auth_tenant_id)


Retrieves the user action for the given Id. If you pass in null for the id, this will return all the user actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_action_id** | **String** | The Id of the user action. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::UserActionResponse**](UserActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_actioning

> models::ActionResponse retrieve_user_actioning(user_id, preventing_login, active)


Retrieves all the actions for the user with the given Id that are currently preventing the User from logging in. OR Retrieves all the actions for the user with the given Id. This will return all time based actions that are active, and inactive as well as non-time based actions. OR Retrieves all the actions for the user with the given Id that are currently active. An active action means one that is time based and has not been canceled, and has not ended. OR Retrieves all the actions for the user with the given Id that are currently inactive. An inactive action means one that is time based and has been canceled or has expired, or is not time based.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The Id of the user to fetch the actions for. |  |
**preventing_login** | Option<**String**> |  |  |
**active** | Option<**String**> |  |  |

### Return type

[**models::ActionResponse**](ActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_change_password

> retrieve_user_change_password(login_id)


Check to see if the user must obtain a Trust Request Id in order to complete a change password request. When a user has enabled Two-Factor authentication, before you are allowed to use the Change Password API to change your password, you must obtain a Trust Request Id by completing a Two-Factor Step-Up authentication.  An HTTP status code of 400 with a general error code of [TrustTokenRequired] indicates that a Trust Token is required to make a POST request to this API. OR Check to see if the user must obtain a Trust Token Id in order to complete a change password request. When a user has enabled Two-Factor authentication, before you are allowed to use the Change Password API to change your password, you must obtain a Trust Token by completing a Two-Factor Step-Up authentication.  An HTTP status code of 400 with a general error code of [TrustTokenRequired] indicates that a Trust Token is required to make a POST request to this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_id** | Option<**String**> | The loginId of the User that you intend to change the password for. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_comments_with_id

> models::UserCommentResponse retrieve_user_comments_with_id(user_id, x_fusion_auth_tenant_id)


Retrieves all the comments for the user with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::UserCommentResponse**](UserCommentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_consent_with_id

> models::UserConsentResponse retrieve_user_consent_with_id(user_consent_id)


Retrieve a single User consent by Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_consent_id** | **String** | The User consent Id | [required] |

### Return type

[**models::UserConsentResponse**](UserConsentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_consents_with_id

> models::UserConsentResponse retrieve_user_consents_with_id(user_id)


Retrieves all the consents for a User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The User's Id |  |

### Return type

[**models::UserConsentResponse**](UserConsentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_info_from_access_token_with_id

> serde_json::Value retrieve_user_info_from_access_token_with_id()


Call the UserInfo endpoint to retrieve User Claims from the access token issued by FusionAuth.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_recent_login

> models::RecentLoginResponse retrieve_user_recent_login(offset, limit, user_id)


Retrieves the last number of login records. OR Retrieves the last number of login records for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | The initial record. e.g. 0 is the last login, 100 will be the 100th most recent login. |  |
**limit** | Option<**String**> | (Optional, defaults to 10) The number of records to retrieve. |  |
**user_id** | Option<**String**> | The Id of the user. |  |

### Return type

[**models::RecentLoginResponse**](RecentLoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user_with_id

> models::UserResponse retrieve_user_with_id(user_id, x_fusion_auth_tenant_id)


Retrieves the user for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_version_with_id

> models::VersionResponse retrieve_version_with_id()


Retrieves the FusionAuth version string.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::VersionResponse**](VersionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_web_authn_credential_with_id

> models::WebAuthnCredentialResponse retrieve_web_authn_credential_with_id(id)


Retrieves the WebAuthn credential for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The Id of the WebAuthn credential. | [required] |

### Return type

[**models::WebAuthnCredentialResponse**](WebAuthnCredentialResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_web_authn_credentials_for_user_with_id

> models::WebAuthnCredentialResponse retrieve_web_authn_credentials_for_user_with_id(user_id)


Retrieves all WebAuthn credentials for the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user's ID. |  |

### Return type

[**models::WebAuthnCredentialResponse**](WebAuthnCredentialResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_webhook

> models::WebhookResponse retrieve_webhook()


Retrieves the webhook for the given Id. If you pass in null for the id, this will return all the webhooks.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_webhook_attempt_log_with_id

> models::WebhookAttemptLogResponse retrieve_webhook_attempt_log_with_id(webhook_attempt_log_id)


Retrieves a single webhook attempt log for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_attempt_log_id** | **String** | The Id of the webhook attempt log to retrieve. | [required] |

### Return type

[**models::WebhookAttemptLogResponse**](WebhookAttemptLogResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_webhook_event_log_with_id

> models::WebhookEventLogResponse retrieve_webhook_event_log_with_id(webhook_event_log_id)


Retrieves a single webhook event log for the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_event_log_id** | **String** | The Id of the webhook event log to retrieve. | [required] |

### Return type

[**models::WebhookEventLogResponse**](WebhookEventLogResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_webhook_with_id

> models::WebhookResponse retrieve_webhook_with_id(webhook_id)


Retrieves the webhook for the given Id. If you pass in null for the id, this will return all the webhooks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The Id of the webhook. | [required] |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_refresh_token_by_id_with_id

> revoke_refresh_token_by_id_with_id(token_id)


Revokes a single refresh token by the unique Id. The unique Id is not sensitive as it cannot be used to obtain another JWT.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | The unique Id of the token to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_user_consent_with_id

> revoke_user_consent_with_id(user_consent_id)


Revokes a single User consent by Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_consent_id** | **String** | The User Consent Id | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_applications_with_id

> models::ApplicationSearchResponse search_applications_with_id(application_search_request)


Searches applications with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_search_request** | Option<[**ApplicationSearchRequest**](ApplicationSearchRequest.md)> |  |  |

### Return type

[**models::ApplicationSearchResponse**](ApplicationSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_audit_logs_with_id

> models::AuditLogSearchResponse search_audit_logs_with_id(audit_log_search_request)


Searches the audit logs with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audit_log_search_request** | Option<[**AuditLogSearchRequest**](AuditLogSearchRequest.md)> |  |  |

### Return type

[**models::AuditLogSearchResponse**](AuditLogSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_consents_with_id

> models::ConsentSearchResponse search_consents_with_id(consent_search_request)


Searches consents with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_search_request** | Option<[**ConsentSearchRequest**](ConsentSearchRequest.md)> |  |  |

### Return type

[**models::ConsentSearchResponse**](ConsentSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_email_templates_with_id

> models::EmailTemplateSearchResponse search_email_templates_with_id(email_template_search_request)


Searches email templates with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_search_request** | Option<[**EmailTemplateSearchRequest**](EmailTemplateSearchRequest.md)> |  |  |

### Return type

[**models::EmailTemplateSearchResponse**](EmailTemplateSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_entities_by_ids_with_id

> models::EntitySearchResponse search_entities_by_ids_with_id(ids)


Retrieves the entities for the given ids. If any Id is invalid, it is ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<**String**> | The entity ids to search for. |  |

### Return type

[**models::EntitySearchResponse**](EntitySearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_entities_with_id

> models::EntitySearchResponse search_entities_with_id(entity_search_request)


Searches entities with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_search_request** | Option<[**EntitySearchRequest**](EntitySearchRequest.md)> |  |  |

### Return type

[**models::EntitySearchResponse**](EntitySearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_entity_grants_with_id

> models::EntityGrantSearchResponse search_entity_grants_with_id(entity_grant_search_request)


Searches Entity Grants with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_grant_search_request** | Option<[**EntityGrantSearchRequest**](EntityGrantSearchRequest.md)> |  |  |

### Return type

[**models::EntityGrantSearchResponse**](EntityGrantSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_entity_types_with_id

> models::EntityTypeSearchResponse search_entity_types_with_id(entity_type_search_request)


Searches the entity types with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_search_request** | Option<[**EntityTypeSearchRequest**](EntityTypeSearchRequest.md)> |  |  |

### Return type

[**models::EntityTypeSearchResponse**](EntityTypeSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_event_logs_with_id

> models::EventLogSearchResponse search_event_logs_with_id(event_log_search_request)


Searches the event logs with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_log_search_request** | Option<[**EventLogSearchRequest**](EventLogSearchRequest.md)> |  |  |

### Return type

[**models::EventLogSearchResponse**](EventLogSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_group_members_with_id

> models::GroupMemberSearchResponse search_group_members_with_id(group_member_search_request)


Searches group members with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_member_search_request** | Option<[**GroupMemberSearchRequest**](GroupMemberSearchRequest.md)> |  |  |

### Return type

[**models::GroupMemberSearchResponse**](GroupMemberSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_groups_with_id

> models::GroupSearchResponse search_groups_with_id(group_search_request)


Searches groups with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_search_request** | Option<[**GroupSearchRequest**](GroupSearchRequest.md)> |  |  |

### Return type

[**models::GroupSearchResponse**](GroupSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_identity_providers_with_id

> models::IdentityProviderSearchResponse search_identity_providers_with_id(identity_provider_search_request)


Searches identity providers with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_search_request** | Option<[**IdentityProviderSearchRequest**](IdentityProviderSearchRequest.md)> |  |  |

### Return type

[**models::IdentityProviderSearchResponse**](IdentityProviderSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_ip_access_control_lists_with_id

> models::IpAccessControlListSearchResponse search_ip_access_control_lists_with_id(ip_access_control_list_search_request)


Searches the IP Access Control Lists with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_access_control_list_search_request** | Option<[**IpAccessControlListSearchRequest**](IpAccessControlListSearchRequest.md)> |  |  |

### Return type

[**models::IpAccessControlListSearchResponse**](IPAccessControlListSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_keys_with_id

> models::KeySearchResponse search_keys_with_id(key_search_request)


Searches keys with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_search_request** | Option<[**KeySearchRequest**](KeySearchRequest.md)> |  |  |

### Return type

[**models::KeySearchResponse**](KeySearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_lambdas_with_id

> models::LambdaSearchResponse search_lambdas_with_id(lambda_search_request)


Searches lambdas with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lambda_search_request** | Option<[**LambdaSearchRequest**](LambdaSearchRequest.md)> |  |  |

### Return type

[**models::LambdaSearchResponse**](LambdaSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_login_records_with_id

> models::LoginRecordSearchResponse search_login_records_with_id(login_record_search_request)


Searches the login records with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_record_search_request** | Option<[**LoginRecordSearchRequest**](LoginRecordSearchRequest.md)> |  |  |

### Return type

[**models::LoginRecordSearchResponse**](LoginRecordSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_tenants_with_id

> models::TenantSearchResponse search_tenants_with_id(tenant_search_request)


Searches tenants with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_search_request** | Option<[**TenantSearchRequest**](TenantSearchRequest.md)> |  |  |

### Return type

[**models::TenantSearchResponse**](TenantSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_themes_with_id

> models::ThemeSearchResponse search_themes_with_id(theme_search_request)


Searches themes with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme_search_request** | Option<[**ThemeSearchRequest**](ThemeSearchRequest.md)> |  |  |

### Return type

[**models::ThemeSearchResponse**](ThemeSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_user_comments_with_id

> models::UserCommentSearchResponse search_user_comments_with_id(user_comment_search_request)


Searches user comments with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_comment_search_request** | Option<[**UserCommentSearchRequest**](UserCommentSearchRequest.md)> |  |  |

### Return type

[**models::UserCommentSearchResponse**](UserCommentSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users_by_ids_with_id

> models::SearchResponse search_users_by_ids_with_id(ids)


Retrieves the users for the given ids. If any Id is invalid, it is ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<**String**> | The user ids to search for. |  |

### Return type

[**models::SearchResponse**](SearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users_by_query_with_id

> models::SearchResponse search_users_by_query_with_id(search_request)


Retrieves the users for the given search criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_request** | Option<[**SearchRequest**](SearchRequest.md)> |  |  |

### Return type

[**models::SearchResponse**](SearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_webhook_event_logs_with_id

> models::WebhookEventLogSearchResponse search_webhook_event_logs_with_id(webhook_event_log_search_request)


Searches the webhook event logs with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_event_log_search_request** | Option<[**WebhookEventLogSearchRequest**](WebhookEventLogSearchRequest.md)> |  |  |

### Return type

[**models::WebhookEventLogSearchResponse**](WebhookEventLogSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_webhooks_with_id

> models::WebhookSearchResponse search_webhooks_with_id(webhook_search_request)


Searches webhooks with the specified criteria and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_search_request** | Option<[**WebhookSearchRequest**](WebhookSearchRequest.md)> |  |  |

### Return type

[**models::WebhookSearchResponse**](WebhookSearchResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_email_with_id

> models::SendResponse send_email_with_id(email_template_id, send_request)


Send an email using an email template id. You can optionally provide <code>requestData</code> to access key value pairs in the email template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_id** | **String** | The Id for the template. | [required] |
**send_request** | Option<[**SendRequest**](SendRequest.md)> |  |  |

### Return type

[**models::SendResponse**](SendResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_family_request_email_with_id

> send_family_request_email_with_id(family_email_request)


Sends out an email to a parent that they need to register and create a family or need to log in and add a child to their existing family.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**family_email_request** | Option<[**FamilyEmailRequest**](FamilyEmailRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_passwordless_code_with_id

> send_passwordless_code_with_id(passwordless_send_request)


Send a passwordless authentication code in an email to complete login.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**passwordless_send_request** | Option<[**PasswordlessSendRequest**](PasswordlessSendRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_two_factor_code_for_enable_disable_with_id

> send_two_factor_code_for_enable_disable_with_id(two_factor_send_request)


Send a Two Factor authentication code to assist in setting up Two Factor authentication or disabling.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_send_request** | Option<[**TwoFactorSendRequest**](TwoFactorSendRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_two_factor_code_for_login_using_method_with_id

> send_two_factor_code_for_login_using_method_with_id(two_factor_id, two_factor_send_request)


Send a Two Factor authentication code to allow the completion of Two Factor authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_id** | **String** | The Id returned by the Login API necessary to complete Two Factor authentication. | [required] |
**two_factor_send_request** | Option<[**TwoFactorSendRequest**](TwoFactorSendRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_identity_provider_login_with_id

> models::IdentityProviderStartLoginResponse start_identity_provider_login_with_id(identity_provider_start_login_request)


Begins a login request for a 3rd party login that requires user interaction such as HYPR.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_start_login_request** | Option<[**IdentityProviderStartLoginRequest**](IdentityProviderStartLoginRequest.md)> |  |  |

### Return type

[**models::IdentityProviderStartLoginResponse**](IdentityProviderStartLoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_passwordless_login_with_id

> models::PasswordlessStartResponse start_passwordless_login_with_id(passwordless_start_request)


Start a passwordless login request by generating a passwordless code. This code can be sent to the User using the Send Passwordless Code API or using a mechanism outside of FusionAuth. The passwordless login is completed by using the Passwordless Login API with this code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**passwordless_start_request** | Option<[**PasswordlessStartRequest**](PasswordlessStartRequest.md)> |  |  |

### Return type

[**models::PasswordlessStartResponse**](PasswordlessStartResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_two_factor_login_with_id

> models::TwoFactorStartResponse start_two_factor_login_with_id(two_factor_start_request)


Start a Two-Factor login request by generating a two-factor identifier. This code can then be sent to the Two Factor Send  API (/api/two-factor/send)in order to send a one-time use code to a user. You can also use one-time use code returned  to send the code out-of-band. The Two-Factor login is completed by making a request to the Two-Factor Login  API (/api/two-factor/login). with the two-factor identifier and the one-time use code.  This API is intended to allow you to begin a Two-Factor login outside a normal login that originated from the Login API (/api/login).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_start_request** | Option<[**TwoFactorStartRequest**](TwoFactorStartRequest.md)> |  |  |

### Return type

[**models::TwoFactorStartResponse**](TwoFactorStartResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_web_authn_login_with_id

> models::WebAuthnStartResponse start_web_authn_login_with_id(web_authn_start_request)


Start a WebAuthn authentication ceremony by generating a new challenge for the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_authn_start_request** | Option<[**WebAuthnStartRequest**](WebAuthnStartRequest.md)> |  |  |

### Return type

[**models::WebAuthnStartResponse**](WebAuthnStartResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_web_authn_registration_with_id

> models::WebAuthnRegisterStartResponse start_web_authn_registration_with_id(web_authn_register_start_request)


Start a WebAuthn registration ceremony by generating a new challenge for the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_authn_register_start_request** | Option<[**WebAuthnRegisterStartRequest**](WebAuthnRegisterStartRequest.md)> |  |  |

### Return type

[**models::WebAuthnRegisterStartResponse**](WebAuthnRegisterStartResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## two_factor_login_with_id

> models::LoginResponse two_factor_login_with_id(two_factor_login_request)


Complete login using a 2FA challenge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**two_factor_login_request** | Option<[**TwoFactorLoginRequest**](TwoFactorLoginRequest.md)> |  |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_key_with_id

> models::ApiKeyResponse update_api_key_with_id(api_key_id, api_key_request)


Updates an API key by given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **String** | The Id of the API key to update. | [required] |
**api_key_request** | Option<[**ApiKeyRequest**](ApiKeyRequest.md)> |  |  |

### Return type

[**models::ApiKeyResponse**](APIKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_role_with_id

> models::ApplicationResponse update_application_role_with_id(application_id, role_id, x_fusion_auth_tenant_id, application_request)


Updates the application role with the given Id for the application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application that the role belongs to. | [required] |
**role_id** | **String** | The Id of the role to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**application_request** | Option<[**ApplicationRequest**](ApplicationRequest.md)> |  |  |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_with_id

> models::ApplicationResponse update_application_with_id(application_id, x_fusion_auth_tenant_id, reactivate, application_request)


Updates the application with the given Id. OR Reactivates the application with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**reactivate** | Option<**String**> |  |  |
**application_request** | Option<[**ApplicationRequest**](ApplicationRequest.md)> |  |  |

### Return type

[**models::ApplicationResponse**](ApplicationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_connector_with_id

> models::ConnectorResponse update_connector_with_id(connector_id, connector_request)


Updates the connector with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connector_id** | **String** | The Id of the connector to update. | [required] |
**connector_request** | Option<[**ConnectorRequest**](ConnectorRequest.md)> |  |  |

### Return type

[**models::ConnectorResponse**](ConnectorResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_consent_with_id

> models::ConsentResponse update_consent_with_id(consent_id, x_fusion_auth_tenant_id, consent_request)


Updates the consent with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | The Id of the consent to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**consent_request** | Option<[**ConsentRequest**](ConsentRequest.md)> |  |  |

### Return type

[**models::ConsentResponse**](ConsentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_email_template_with_id

> models::EmailTemplateResponse update_email_template_with_id(email_template_id, x_fusion_auth_tenant_id, email_template_request)


Updates the email template with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_id** | **String** | The Id of the email template to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**email_template_request** | Option<[**EmailTemplateRequest**](EmailTemplateRequest.md)> |  |  |

### Return type

[**models::EmailTemplateResponse**](EmailTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_entity_type_permission_with_id

> models::EntityTypeResponse update_entity_type_permission_with_id(entity_type_id, permission_id, entity_type_request)


Updates the permission with the given Id for the entity type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_id** | **String** | The Id of the entityType that the permission belongs to. | [required] |
**permission_id** | **String** | The Id of the permission to update. | [required] |
**entity_type_request** | Option<[**EntityTypeRequest**](EntityTypeRequest.md)> |  |  |

### Return type

[**models::EntityTypeResponse**](EntityTypeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_entity_type_with_id

> models::EntityTypeResponse update_entity_type_with_id(entity_type_id, entity_type_request)


Updates the Entity Type with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_type_id** | **String** | The Id of the Entity Type to update. | [required] |
**entity_type_request** | Option<[**EntityTypeRequest**](EntityTypeRequest.md)> |  |  |

### Return type

[**models::EntityTypeResponse**](EntityTypeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_entity_with_id

> models::EntityResponse update_entity_with_id(entity_id, x_fusion_auth_tenant_id, entity_request)


Updates the Entity with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | The Id of the Entity to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**entity_request** | Option<[**EntityRequest**](EntityRequest.md)> |  |  |

### Return type

[**models::EntityResponse**](EntityResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_form_field_with_id

> models::FormFieldResponse update_form_field_with_id(field_id, form_field_request)


Updates the form field with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The Id of the form field to update. | [required] |
**form_field_request** | Option<[**FormFieldRequest**](FormFieldRequest.md)> |  |  |

### Return type

[**models::FormFieldResponse**](FormFieldResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_form_with_id

> models::FormResponse update_form_with_id(form_id, form_request)


Updates the form with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | The Id of the form to update. | [required] |
**form_request** | Option<[**FormRequest**](FormRequest.md)> |  |  |

### Return type

[**models::FormResponse**](FormResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_members_with_id

> models::MemberResponse update_group_members_with_id(member_request)


Creates a member in a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_request** | Option<[**MemberRequest**](MemberRequest.md)> |  |  |

### Return type

[**models::MemberResponse**](MemberResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_with_id

> models::GroupResponse update_group_with_id(group_id, x_fusion_auth_tenant_id, group_request)


Updates the group with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The Id of the group to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**group_request** | Option<[**GroupRequest**](GroupRequest.md)> |  |  |

### Return type

[**models::GroupResponse**](GroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_identity_provider_with_id

> models::IdentityProviderResponse update_identity_provider_with_id(identity_provider_id, identity_provider_request)


Updates the identity provider with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_provider_id** | **String** | The Id of the identity provider to update. | [required] |
**identity_provider_request** | Option<[**IdentityProviderRequest**](IdentityProviderRequest.md)> |  |  |

### Return type

[**models::IdentityProviderResponse**](IdentityProviderResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_integrations_with_id

> models::IntegrationResponse update_integrations_with_id(integration_request)


Updates the available integrations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_request** | Option<[**IntegrationRequest**](IntegrationRequest.md)> |  |  |

### Return type

[**models::IntegrationResponse**](IntegrationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ip_access_control_list_with_id

> models::IpAccessControlListResponse update_ip_access_control_list_with_id(access_control_list_id, ip_access_control_list_request)


Updates the IP Access Control List with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_control_list_id** | **String** | The Id of the IP Access Control List to update. | [required] |
**ip_access_control_list_request** | Option<[**IpAccessControlListRequest**](IpAccessControlListRequest.md)> |  |  |

### Return type

[**models::IpAccessControlListResponse**](IPAccessControlListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_key_with_id

> models::KeyResponse update_key_with_id(key_id, key_request)


Updates the key with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The Id of the key to update. | [required] |
**key_request** | Option<[**KeyRequest**](KeyRequest.md)> |  |  |

### Return type

[**models::KeyResponse**](KeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_lambda_with_id

> models::LambdaResponse update_lambda_with_id(lambda_id, lambda_request)


Updates the lambda with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lambda_id** | **String** | The Id of the lambda to update. | [required] |
**lambda_request** | Option<[**LambdaRequest**](LambdaRequest.md)> |  |  |

### Return type

[**models::LambdaResponse**](LambdaResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_message_template_with_id

> models::MessageTemplateResponse update_message_template_with_id(message_template_id, message_template_request)


Updates the message template with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_template_id** | **String** | The Id of the message template to update. | [required] |
**message_template_request** | Option<[**MessageTemplateRequest**](MessageTemplateRequest.md)> |  |  |

### Return type

[**models::MessageTemplateResponse**](MessageTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_messenger_with_id

> models::MessengerResponse update_messenger_with_id(messenger_id, messenger_request)


Updates the messenger with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messenger_id** | **String** | The Id of the messenger to update. | [required] |
**messenger_request** | Option<[**MessengerRequest**](MessengerRequest.md)> |  |  |

### Return type

[**models::MessengerResponse**](MessengerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_o_auth_scope_with_id

> models::ApplicationOAuthScopeResponse update_o_auth_scope_with_id(application_id, scope_id, x_fusion_auth_tenant_id, application_o_auth_scope_request)


Updates the OAuth scope with the given Id for the application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The Id of the application that the OAuth scope belongs to. | [required] |
**scope_id** | **String** | The Id of the OAuth scope to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**application_o_auth_scope_request** | Option<[**ApplicationOAuthScopeRequest**](ApplicationOAuthScopeRequest.md)> |  |  |

### Return type

[**models::ApplicationOAuthScopeResponse**](ApplicationOAuthScopeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_registration_with_id

> models::RegistrationResponse update_registration_with_id(user_id, x_fusion_auth_tenant_id, registration_request)


Updates the registration for the user with the given Id and the application defined in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user whose registration is going to be updated. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**registration_request** | Option<[**RegistrationRequest**](RegistrationRequest.md)> |  |  |

### Return type

[**models::RegistrationResponse**](RegistrationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_system_configuration_with_id

> models::SystemConfigurationResponse update_system_configuration_with_id(system_configuration_request)


Updates the system configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_configuration_request** | Option<[**SystemConfigurationRequest**](SystemConfigurationRequest.md)> |  |  |

### Return type

[**models::SystemConfigurationResponse**](SystemConfigurationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant_with_id

> models::TenantResponse update_tenant_with_id(tenant_id, x_fusion_auth_tenant_id, tenant_request)


Updates the tenant with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | The Id of the tenant to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**tenant_request** | Option<[**TenantRequest**](TenantRequest.md)> |  |  |

### Return type

[**models::TenantResponse**](TenantResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_theme_with_id

> models::ThemeResponse update_theme_with_id(theme_id, theme_request)


Updates the theme with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme_id** | **String** | The Id of the theme to update. | [required] |
**theme_request** | Option<[**ThemeRequest**](ThemeRequest.md)> |  |  |

### Return type

[**models::ThemeResponse**](ThemeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_action_reason_with_id

> models::UserActionReasonResponse update_user_action_reason_with_id(user_action_reason_id, user_action_reason_request)


Updates the user action reason with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_action_reason_id** | **String** | The Id of the user action reason to update. | [required] |
**user_action_reason_request** | Option<[**UserActionReasonRequest**](UserActionReasonRequest.md)> |  |  |

### Return type

[**models::UserActionReasonResponse**](UserActionReasonResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_action_with_id

> models::UserActionResponse update_user_action_with_id(user_action_id, reactivate, x_fusion_auth_tenant_id, user_action_request)


Reactivates the user action with the given Id. OR Updates the user action with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_action_id** | **String** | The Id of the user action to reactivate. | [required] |
**reactivate** | Option<**String**> |  |  |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**user_action_request** | Option<[**UserActionRequest**](UserActionRequest.md)> |  |  |

### Return type

[**models::UserActionResponse**](UserActionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_consent_with_id

> models::UserConsentResponse update_user_consent_with_id(user_consent_id, user_consent_request)


Updates a single User consent by Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_consent_id** | **String** | The User Consent Id | [required] |
**user_consent_request** | Option<[**UserConsentRequest**](UserConsentRequest.md)> |  |  |

### Return type

[**models::UserConsentResponse**](UserConsentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_family_with_id

> models::FamilyResponse update_user_family_with_id(family_id, x_fusion_auth_tenant_id, family_request)


Updates a family with a given Id. OR Adds a user to an existing family. The family Id must be specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**family_id** | **String** | The Id of the family to update. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**family_request** | Option<[**FamilyRequest**](FamilyRequest.md)> |  |  |

### Return type

[**models::FamilyResponse**](FamilyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_verify_email

> models::VerifyEmailResponse update_user_verify_email(email, application_id, send_verify_email)


Re-sends the verification email to the user. OR Re-sends the verification email to the user. If the Application has configured a specific email template this will be used instead of the tenant configuration. OR Generate a new Email Verification Id to be used with the Verify Email API. This API will not attempt to send an email to the User. This API may be used to collect the verificationId for use with a third party system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | Option<**String**> | The email address of the user that needs a new verification email. |  |
**application_id** | Option<**String**> | The unique Application Id to used to resolve an application specific email template. |  |
**send_verify_email** | Option<**String**> |  |  |

### Return type

[**models::VerifyEmailResponse**](VerifyEmailResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_verify_registration

> models::VerifyRegistrationResponse update_user_verify_registration(email, send_verify_password_email, application_id)


Generate a new Application Registration Verification Id to be used with the Verify Registration API. This API will not attempt to send an email to the User. This API may be used to collect the verificationId for use with a third party system. OR Re-sends the application registration verification email to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | Option<**String**> | The email address of the user that needs a new verification email. |  |
**send_verify_password_email** | Option<**String**> |  |  |
**application_id** | Option<**String**> | The Id of the application to be verified. |  |

### Return type

[**models::VerifyRegistrationResponse**](VerifyRegistrationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_with_id

> models::UserResponse update_user_with_id(user_id, reactivate, x_fusion_auth_tenant_id, user_request)


Reactivates the user with the given Id. OR Updates the user with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The Id of the user to reactivate. | [required] |
**reactivate** | Option<**String**> |  |  |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**user_request** | Option<[**UserRequest**](UserRequest.md)> |  |  |

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_with_id

> models::WebhookResponse update_webhook_with_id(webhook_id, webhook_request)


Updates the webhook with the given Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The Id of the webhook to update. | [required] |
**webhook_request** | Option<[**WebhookRequest**](WebhookRequest.md)> |  |  |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_entity_grant_with_id

> upsert_entity_grant_with_id(entity_id, x_fusion_auth_tenant_id, entity_grant_request)


Creates or updates an Entity Grant. This is when a User/Entity is granted permissions to an Entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** | The Id of the Entity that the User/Entity is being granted access to. | [required] |
**x_fusion_auth_tenant_id** | Option<**uuid::Uuid**> | The unique Id of the tenant used to scope this API request. Only required when there is more than one tenant and the API key is not tenant-scoped. |  |
**entity_grant_request** | Option<[**EntityGrantRequest**](EntityGrantRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_device_with_id

> validate_device_with_id(user_code, client_id)


Validates the end-user provided user_code from the user-interaction of the Device Authorization Grant. If you build your own activation form you should validate the user provided code prior to beginning the Authorization grant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_code** | Option<**String**> | The end-user verification code. |  |
**client_id** | Option<**String**> | The client id. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_jwt_with_id

> models::ValidateResponse validate_jwt_with_id()


Validates the provided JWT (encoded JWT string) to ensure the token is valid. A valid access token is properly signed and not expired. <p> This API may be used to verify the JWT as well as decode the encoded JWT into human readable identity claims.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ValidateResponse**](ValidateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vend_jwt_with_id

> models::JwtVendResponse vend_jwt_with_id(jwt_vend_request)


It's a JWT vending machine!  Issue a new access token (JWT) with the provided claims in the request. This JWT is not scoped to a tenant or user, it is a free form  token that will contain what claims you provide. <p> The iat, exp and jti claims will be added by FusionAuth, all other claims must be provided by the caller.  If a TTL is not provided in the request, the TTL will be retrieved from the default Tenant or the Tenant specified on the request either  by way of the X-FusionAuth-TenantId request header, or a tenant scoped API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jwt_vend_request** | Option<[**JwtVendRequest**](JwtVendRequest.md)> |  |  |

### Return type

[**models::JwtVendResponse**](JWTVendResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_user_registration_with_id

> verify_user_registration_with_id(verify_registration_request)


Confirms a user's registration.   The request body will contain the verificationId. You may also be required to send a one-time use code based upon your configuration. When  the application is configured to gate a user until their registration is verified, this procedures requires two values instead of one.  The verificationId is a high entropy value and the one-time use code is a low entropy value that is easily entered in a user interactive form. The  two values together are able to confirm a user's registration and mark the user's registration as verified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_registration_request** | Option<[**VerifyRegistrationRequest**](VerifyRegistrationRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

