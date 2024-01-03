# SendRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**bcc_addresses** | Option<**Vec<String>**> |  | [optional]
**cc_addresses** | Option<**Vec<String>**> |  | [optional]
**preferred_languages** | Option<**Vec<String>**> |  | [optional]
**request_data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**to_addresses** | Option<[**Vec<crate::models::EmailAddress>**](EmailAddress.md)> |  | [optional]
**user_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


