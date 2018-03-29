#[cfg(test)]
mod tests;

use super::*;

#[derive(Serialize,Deserialize)]
pub struct APICall {
    pub version: String,
    pub id: String,
    pub account: String,
    pub time: DateTime<Utc>,
    pub region: String,
    pub resources: Vec<String>,
    pub detail: APICallDetail,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct APICallDetail {
    pub event_version: String,
    pub user_identity: UserIdentity,
    pub event_time: DateTime<Utc>,
    pub event_source: String,
    pub event_name: String,
    pub aws_region: String,
    #[serde(rename="sourceIPAddress")]
    pub source_ip_address: String,
    pub user_agent: String,
    pub request_parameters: Option<BTreeMap<String, String>>,
    pub response_elements: Option<Value>,
    #[serde(rename="requestID")]
    pub request_id: String,
    #[serde(rename="eventID")]
    pub event_id: String,
    pub event_type: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UserIdentity {
    #[serde(rename="type")]
    pub user_type: String,
    pub principal_id: String,
    pub arn: String,
    pub account_id: String,
    pub session_context: SessionContext,
}

#[derive(Serialize,Deserialize)]
pub struct SessionContext {
    pub attributes: SessionContextAttributes,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SessionContextAttributes {
    pub mfa_authenticated: String,
    pub creation_date: DateTime<Utc>,
}
