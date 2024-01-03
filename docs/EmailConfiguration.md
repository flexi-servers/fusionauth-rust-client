# EmailConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_headers** | Option<[**Vec<crate::models::EmailHeader>**](EmailHeader.md)> |  | [optional]
**debug** | Option<**bool**> |  | [optional]
**default_from_email** | Option<**String**> |  | [optional]
**default_from_name** | Option<**String**> |  | [optional]
**email_update_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**email_verified_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**forgot_password_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**host** | Option<**String**> |  | [optional]
**implicit_email_verification_allowed** | Option<**bool**> |  | [optional]
**login_id_in_use_on_create_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**login_id_in_use_on_update_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**login_new_device_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**login_suspicious_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**password** | Option<**String**> |  | [optional]
**password_reset_success_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**password_update_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**passwordless_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**port** | Option<**i32**> |  | [optional]
**properties** | Option<**String**> |  | [optional]
**security** | Option<[**crate::models::EmailSecurityType**](EmailSecurityType.md)> |  | [optional]
**set_password_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**two_factor_method_add_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**two_factor_method_remove_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**unverified** | Option<[**crate::models::EmailUnverifiedOptions**](EmailUnverifiedOptions.md)> |  | [optional]
**username** | Option<**String**> |  | [optional]
**verification_email_template_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**verification_strategy** | Option<[**crate::models::VerificationStrategy**](VerificationStrategy.md)> |  | [optional]
**verify_email** | Option<**bool**> |  | [optional]
**verify_email_when_changed** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


