/*
 * FusionAuth API
 *
 * This is a FusionAuth server. Find out more at [https://fusionauth.io](https://fusionauth.io). You need to [set up an API key](https://fusionauth.io/docs/v1/tech/apis/authentication#managing-api-keys) in the FusionAuth instance you are using to test out the API calls.
 *
 * The version of the OpenAPI document: 1.53.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TokenType : <ul>  <li>Bearer Token type as defined by <a href=\"https:tools.ietf.orghtmlrfc6750\">RFC 6750<a>.<li>  <li>MAC Token type as referenced by <a href=\"https:tools.ietf.orghtmlrfc6749\">RFC 6749<a> and  <a href=\"https:tools.ietf.orghtmldraft-ietf-oauth-v2-http-mac-05\">  Draft RFC on OAuth 2.0 Message Authentication Code (MAC) Tokens<a>  <li>  <ul>
/// <ul>  <li>Bearer Token type as defined by <a href=\"https:tools.ietf.orghtmlrfc6750\">RFC 6750<a>.<li>  <li>MAC Token type as referenced by <a href=\"https:tools.ietf.orghtmlrfc6749\">RFC 6749<a> and  <a href=\"https:tools.ietf.orghtmldraft-ietf-oauth-v2-http-mac-05\">  Draft RFC on OAuth 2.0 Message Authentication Code (MAC) Tokens<a>  <li>  <ul>
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "Bearer")]
    Bearer,
    #[serde(rename = "MAC")]
    Mac,

}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bearer => write!(f, "Bearer"),
            Self::Mac => write!(f, "MAC"),
        }
    }
}

impl Default for TokenType {
    fn default() -> TokenType {
        Self::Bearer
    }
}

