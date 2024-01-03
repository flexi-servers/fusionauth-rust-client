/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReloadRequest : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReloadRequest {
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}

impl ReloadRequest {
    /// 
    pub fn new() -> ReloadRequest {
        ReloadRequest {
            names: None,
        }
    }
}


