# Key

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**algorithm** | Option<[**crate::models::KeyAlgorithm**](KeyAlgorithm.md)> |  | [optional]
**certificate** | Option<**String**> |  | [optional]
**certificate_information** | Option<[**crate::models::CertificateInformation**](CertificateInformation.md)> |  | [optional]
**expiration_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**has_private_key** | Option<**bool**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**issuer** | Option<**String**> |  | [optional]
**kid** | Option<**String**> |  | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**length** | Option<**i32**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**private_key** | Option<**String**> |  | [optional]
**public_key** | Option<**String**> |  | [optional]
**secret** | Option<**String**> |  | [optional]
**r#type** | Option<[**crate::models::KeyType**](KeyType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


