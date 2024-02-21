/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectorLambdaConfiguration {
    #[serde(rename = "reconcileId", skip_serializing_if = "Option::is_none")]
    pub reconcile_id: Option<uuid::Uuid>,
}

impl ConnectorLambdaConfiguration {
    pub fn new() -> ConnectorLambdaConfiguration {
        ConnectorLambdaConfiguration {
            reconcile_id: None,
        }
    }
}


