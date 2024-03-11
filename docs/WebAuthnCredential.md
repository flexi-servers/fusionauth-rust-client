# WebAuthnCredential

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**algorithm** | Option<[**models::CoseAlgorithmIdentifier**](CoseAlgorithmIdentifier.md)> |  | [optional]
**attestation_type** | Option<[**models::AttestationType**](AttestationType.md)> |  | [optional]
**authenticator_supports_user_verification** | Option<**bool**> |  | [optional]
**credential_id** | Option<**String**> |  | [optional]
**data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**discoverable** | Option<**bool**> |  | [optional]
**display_name** | Option<**String**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_use_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**name** | Option<**String**> |  | [optional]
**public_key** | Option<**String**> |  | [optional]
**relying_party_id** | Option<**String**> |  | [optional]
**sign_count** | Option<**i32**> |  | [optional]
**tenant_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**transports** | Option<**Vec<String>**> |  | [optional]
**user_agent** | Option<**String**> |  | [optional]
**user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


