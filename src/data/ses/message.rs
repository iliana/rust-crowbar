use super::*;

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Message {
    pub content: String,
    #[serde(rename="mail")]
    pub details: Details,
    pub notification_type: NotificationType,
    pub receipt: Receipt,
}

#[derive(Serialize,Deserialize)]
pub enum NotificationType {
    Received,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Details {
    pub destination: Vec<String>,
    pub headers_truncated: bool,
    pub message_id: String,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub headers: Vec<Header>,
    pub common_headers: CommonHeaders,
}

#[derive(Serialize,Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CommonHeaders {
    // format: %a, %d %b %Y %H:%M:%S %ze
    pub date: String,
    pub from: Vec<String>,
    pub message_id: String,
    pub return_path: String,
    pub subject: String,
    pub to: Vec<String>,
}

#[derive(Serialize,Deserialize)]
pub enum ActionType {
    Bounce,
    Lambda,
    S3,
    #[serde(rename="SNS")]
    Sns,
    Stop,
    WorkMail,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Receipt {
    pub action: Action,
    pub dkim_verdict: DkimVerdict,
    pub dmarc_policy: Option<DmarcPolicy>,
    pub dmarc_verdict: DmarcVerdict,
    pub processing_time_millis: u64,
    pub recipients: Vec<String>,
    pub spam_verdict: SpamVerdict,
    pub spf_verdict: SpfVerdict,
    pub timestamp: DateTime<Utc>,
    pub virus_verdict: VirusVerdict,
}

#[derive(Serialize,Deserialize)]
#[serde(tag="status")]
pub enum Verdict {
    #[serde(rename="FAIL")]
    Fail,
    #[serde(rename="GRAY")]
    Gray,
    #[serde(rename="PASS")]
    Pass,
    #[serde(rename="PROCESSING_FAILED")]
    ProcessingFailed,
}

pub type DkimVerdict = Verdict;
pub type DmarcVerdict = Verdict;
pub type SpamVerdict = Verdict;
pub type SpfVerdict = Verdict;
pub type VirusVerdict = Verdict;

#[derive(Serialize,Deserialize)]
pub enum DmarcPolicy {
    #[serde(rename="NONE")]
    Absent,
    #[serde(rename="QUARANTINE")]
    Quarantine,
    #[serde(rename="REJECT")]
    Reject,
}

#[derive(Serialize,Deserialize)]
#[serde(tag="type")]
pub enum Action {
    Lambda(LambdaAction),
    #[serde(rename="SNS")]
    Sns(SnsAction),
    S3(S3Action),
    Bounce(BounceAction),
    Stop(StopAction),
    WorkMail(WorkMailAction),
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct LambdaAction {
    pub function_arn: Option<String>,
    pub invocation_type: LambdaInvocationType,
}

#[derive(Eq,PartialEq,Serialize,Deserialize)]
pub enum LambdaInvocationType {
    Event,
    RequestResponse,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SnsAction {
    pub topic_arn: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct S3Action {
    pub bucket_name: String,
    pub object_key: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct BounceAction {
    pub smtp_reply_code: String,
    pub status_code: String,
    pub message: String,
    pub sender: String,
}

#[derive(Serialize,Deserialize)]
pub struct StopAction {}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct WorkMailAction {
    pub organization_arn: String,
}
