/// API Gateway Custom Authorizer Events
use super::*;

use data::apigateway::HttpEventRequestContext;

use std::fmt;

/// The type of an `AuthEvent`.
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
#[derive(Debug,Eq,PartialEq,Serialize,Deserialize)]
pub enum AuthEventType {
    Request,
    Token
}

/// An API Gateway authorization event for a custom Lambda authorizer.
#[serde(rename_all="camelCase")]
#[derive(Serialize,Deserialize)]
pub struct AuthEvent {
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    pub http_method: String,
    pub method_arn: String,
    pub path: String,
    #[serde(default)]
    pub path_parameters: BTreeMap<String, String>,
    #[serde(default)]
    pub query_string_parameters: BTreeMap<String, String>,
    pub resource: String,
    pub request_context: HttpEventRequestContext,
    #[serde(default)]
    pub stage_variables: BTreeMap<String, String>,
    #[serde(rename="type")]
    pub event_type: AuthEventType,
}

/// The response effect for a given `AuthEvent`.
#[derive(Serialize,Deserialize)]
pub enum AuthEffect {
    /// Allow access to the underlying resource.
    Allow,
    /// Reject access to the underlying resource.
    Deny
}

impl fmt::Display for AuthEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           AuthEffect::Allow => write!(f, "Allow"),
           AuthEffect::Deny  => write!(f, "Deny")
       }
    }
}
