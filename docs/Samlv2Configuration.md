# Samlv2Configuration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assertion_encryption_configuration** | Option<[**models::Samlv2AssertionEncryptionConfiguration**](SAMLv2AssertionEncryptionConfiguration.md)> |  | [optional]
**audience** | Option<**String**> |  | [optional]
**authorized_redirect_urls** | Option<**Vec<String>**> |  | [optional]
**debug** | Option<**bool**> |  | [optional]
**default_verification_key_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**initiated_login** | Option<[**models::Samlv2IdPInitiatedLoginConfiguration**](SAMLv2IdPInitiatedLoginConfiguration.md)> |  | [optional]
**issuer** | Option<**String**> |  | [optional]
**key_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**login_hint_configuration** | Option<[**models::LoginHintConfiguration**](LoginHintConfiguration.md)> |  | [optional]
**logout** | Option<[**models::Samlv2Logout**](SAMLv2Logout.md)> |  | [optional]
**logout_url** | Option<**String**> |  | [optional]
**require_signed_requests** | Option<**bool**> |  | [optional]
**xml_signature_c14n_method** | Option<[**models::CanonicalizationMethod**](CanonicalizationMethod.md)> |  | [optional]
**xml_signature_location** | Option<[**models::XmlSignatureLocation**](XMLSignatureLocation.md)> |  | [optional]
**callback_url** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


