/// Lambda Event Types
/// Defined in https://docs.aws.amazon.com/lambda/latest/dg/eventsources.html
#[cfg(test)]
mod tests;

pub mod apigateway;
pub mod cloudwatch;
pub mod s3;
pub mod ses;
pub mod sns;

use self::apigateway::auth;

use chrono::prelude::*;

use std::collections::BTreeMap;

use serde::de::{self, Deserialize, Deserializer};

use serde_json::Value;
use serde_json::Map;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Event {
    CloudWatch(cloudwatch::Event),
    Auth(auth::Event),
    Http(apigateway::HttpEvent),
    Records(Records),
    Unknown(Value),
}

#[derive(Deserialize)]
#[serde(tag="eventSource", remote="Record")]
pub enum Record {
    #[serde(rename="aws:s3")]
    S3(s3::Record),
    #[serde(rename="aws:ses")]
    Ses(ses::Record),
    #[serde(rename="aws:sns")]
    Sns(sns::Record),
    Unknown(Value),
}

impl<'de> Deserialize<'de> for Record {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: Deserializer<'de> {
        // due to case variance (eventSource|EventSource), we transform to regular camel case to make
        // things consistent.
        let mut map = Map::<String, Value>::deserialize(deserializer)?;
        if let Some(event_source) = map.remove("EventSource") {
            map.insert("eventSource".to_owned(), event_source);
        }
        Record::deserialize(Value::Object(map)).map_err(de::Error::custom)
    }
}

#[derive(Deserialize)]
pub struct Records {
    #[serde(rename="Records")]
    pub entries: Vec<Record>,
}
