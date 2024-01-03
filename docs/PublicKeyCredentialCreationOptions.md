# PublicKeyCredentialCreationOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attestation** | Option<[**crate::models::AttestationConveyancePreference**](AttestationConveyancePreference.md)> |  | [optional]
**authenticator_selection** | Option<[**crate::models::AuthenticatorSelectionCriteria**](AuthenticatorSelectionCriteria.md)> |  | [optional]
**challenge** | Option<**String**> |  | [optional]
**exclude_credentials** | Option<[**Vec<crate::models::PublicKeyCredentialDescriptor>**](PublicKeyCredentialDescriptor.md)> |  | [optional]
**extensions** | Option<[**crate::models::WebAuthnRegistrationExtensionOptions**](WebAuthnRegistrationExtensionOptions.md)> |  | [optional]
**pub_key_cred_params** | Option<[**Vec<crate::models::PublicKeyCredentialParameters>**](PublicKeyCredentialParameters.md)> |  | [optional]
**rp** | Option<[**crate::models::PublicKeyCredentialRelyingPartyEntity**](PublicKeyCredentialRelyingPartyEntity.md)> |  | [optional]
**timeout** | Option<**i64**> |  | [optional]
**user** | Option<[**crate::models::PublicKeyCredentialUserEntity**](PublicKeyCredentialUserEntity.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


