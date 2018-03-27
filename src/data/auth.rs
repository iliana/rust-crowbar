use data::HttpEventRequestContext;

use std::collections::BTreeMap;
use std::fmt;

#[serde(rename_all="SCREAMING_SNAKE_CASE")]
#[derive(Debug,Eq,PartialEq,Serialize,Deserialize)]
pub enum EventType {
    Request,
    Token
}

#[serde(rename_all="camelCase")]
#[derive(Serialize,Deserialize)]
pub struct Event {
    pub headers: Option<BTreeMap<String, String>>,
    pub http_method: String,
    pub method_arn: String,
    pub path: String,
    pub path_parameters: Option<BTreeMap<String, String>>,
    pub query_string_parameters: Option<BTreeMap<String, String>>,
    pub resource: String,
    pub request_context: HttpEventRequestContext,
    pub stage_variables: Option<BTreeMap<String, String>>,
    #[serde(rename="type")]
    pub event_type: EventType,
}

#[derive(Serialize,Deserialize)]
pub enum Effect {
    Allow,
    Deny
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Effect::Allow => write!(f, "Allow"),
           Effect::Deny  => write!(f, "Deny")
       }
    }
}
