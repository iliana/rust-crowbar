/// CloudWatch Event Types: https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/EventTypes.html

use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Serialize,Deserialize)]
pub struct Event {
    pub account: String,
    // TODO implement concrete data types maybe using an enum as elsewhere
    pub detail: BTreeMap<String,Value>,
    #[serde(rename="detail-type")]
    pub detail_type: String,
    pub id: String,
    pub region: String,
    pub resources: Vec<String>,
    pub source: String,
    pub time: String,
    pub version: String,
}
