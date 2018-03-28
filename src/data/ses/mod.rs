/// Data mapped from: https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-notifications-contents.html
#[cfg(test)]
mod tests;

pub mod message;

use super::*;

pub use self::message::Action;
pub use self::message::LambdaInvocationType;
pub use self::message::Message;

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Record {
    pub event_version: String,
    #[serde(rename="ses")]
    pub event: Event,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Event {
    #[serde(rename="mail")]
    pub details: message::Details,
    pub receipt: message::Receipt,
}

#[derive(Serialize,Deserialize)]
pub struct Response {
    #[serde(rename="disposition")]
    pub action: ResponseAction,
}

#[derive(Serialize,Deserialize)]
pub enum ResponseAction {
    #[serde(rename="CONTINUE")]
    Continue,
    #[serde(rename="STOP_RULE")]
    StopRule,
    #[serde(rename="STOP_RULE_SET")]
    StopRuleSet,
}

impl Response {

    pub fn from(action: ResponseAction) -> Self {
        Response { action }
    }

    pub fn proceed() -> Self {
        Response::from(ResponseAction::Continue)
    }

    pub fn stop_rule() -> Self {
        Response::from(ResponseAction::StopRule)
    }

    pub fn stop_rule_set() -> Self {
        Response::from(ResponseAction::StopRuleSet)
    }
}
