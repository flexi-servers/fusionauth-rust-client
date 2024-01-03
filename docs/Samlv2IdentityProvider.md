# Samlv2IdentityProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domains** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**assertion_configuration** | Option<[**crate::models::Samlv2AssertionConfiguration**](SAMLv2AssertionConfiguration.md)> |  | [optional]
**button_image_url** | Option<**String**> |  | [optional]
**button_text** | Option<**String**> |  | [optional]
**idp_endpoint** | Option<**String**> |  | [optional]
**idp_initiated_configuration** | Option<[**crate::models::Samlv2IdpInitiatedConfiguration**](SAMLv2IdpInitiatedConfiguration.md)> |  | [optional]
**issuer** | Option<**String**> |  | [optional]
**login_hint_configuration** | Option<[**crate::models::LoginHintConfiguration**](LoginHintConfiguration.md)> |  | [optional]
**name_id_format** | Option<**String**> |  | [optional]
**post_request** | Option<**bool**> |  | [optional]
**request_signing_key_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**sign_request** | Option<**bool**> |  | [optional]
**xml_signature_c14n_method** | Option<[**crate::models::CanonicalizationMethod**](CanonicalizationMethod.md)> |  | [optional]
**email_claim** | Option<**String**> |  | [optional]
**key_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**unique_id_claim** | Option<**String**> |  | [optional]
**use_name_id_for_email** | Option<**bool**> |  | [optional]
**username_claim** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


