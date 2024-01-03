/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.49.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailTemplateErrors {
    #[serde(rename = "parseErrors", skip_serializing_if = "Option::is_none")]
    pub parse_errors: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "renderErrors", skip_serializing_if = "Option::is_none")]
    pub render_errors: Option<::std::collections::HashMap<String, String>>,
}

impl EmailTemplateErrors {
    pub fn new() -> EmailTemplateErrors {
        EmailTemplateErrors {
            parse_errors: None,
            render_errors: None,
        }
    }
}


