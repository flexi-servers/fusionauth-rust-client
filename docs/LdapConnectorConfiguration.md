# LdapConnectorConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_url** | Option<**String**> |  | [optional]
**base_structure** | Option<**String**> |  | [optional]
**connect_timeout** | Option<**i32**> |  | [optional]
**identifying_attribute** | Option<**String**> |  | [optional]
**lambda_configuration** | Option<[**models::ConnectorLambdaConfiguration**](ConnectorLambdaConfiguration.md)> |  | [optional]
**login_id_attribute** | Option<**String**> |  | [optional]
**read_timeout** | Option<**i32**> |  | [optional]
**requested_attributes** | Option<**Vec<String>**> |  | [optional]
**security_method** | Option<[**models::LdapSecurityMethod**](LDAPSecurityMethod.md)> |  | [optional]
**system_account_dn** | Option<**String**> |  | [optional]
**system_account_password** | Option<**String**> |  | [optional]
**data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**debug** | Option<**bool**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**insert_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**last_update_instant** | Option<**i64**> | The number of milliseconds since the unix epoch: January 1, 1970 00:00:00 UTC. This value is always in UTC. | [optional]
**name** | Option<**String**> |  | [optional]
**r#type** | Option<[**models::ConnectorType**](ConnectorType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


