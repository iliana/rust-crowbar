#[cfg(test)]
mod tests;

use super::*;

use chrono::prelude::*;

#[derive(Serialize,Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct MessageAttribute {
    #[serde(rename="Type")]
    pub attribute_type: MessageAttributeType,
    pub value: String,
}

#[derive(Serialize,Deserialize)]
pub enum MessageAttributeType {
    #[serde(rename="String")]
    UTF8,
    Binary,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct Record {
    pub event_version: String,
    pub event_subscription_arn: String,
    #[serde(rename="Sns")]
    pub event: Event,
}

// See: https://docs.aws.amazon.com/sns/latest/dg/json-formats.html#http-notification-json
#[derive(Serialize,Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct Event {
    pub message: String,
    pub message_attributes: Option<BTreeMap<String, MessageAttribute>>,
    pub message_id: String,
    pub message_type: Option<EventType>,
    pub signature: String,
    pub signature_version: String,
    pub signing_cert_url: String,
    pub subject: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub topic_arn: String,
    pub unsubscribe_url: String,
}

#[derive(Serialize,Deserialize)]
pub enum EventType {
    Notification
}
