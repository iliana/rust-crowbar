#[cfg(test)]
mod tests;

use super::*;

use chrono::Duration;

#[derive(Serialize,Deserialize)]
#[serde(untagged)]
pub enum AutoScalingEvent {
    Action(LifecycleAction),
    Event(LifecycleEvent),
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct LifecycleAction {
    pub account: String,
    pub id: String,
    pub detail: ActionDetail,
    pub detail_type: String,
    pub region: String,
    pub resources: Vec<String>,
    pub time: DateTime<Utc>,
    pub version: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct ActionDetail {
    #[serde(rename="AutoScalingGroupName")]
    pub autoscaling_group_name: String,
    #[serde(rename="EC2InstanceId")]
    pub ec2_instance_id: String,
    pub lifecycle_action_token: String,
    pub lifecycle_hook_name: String,
    pub lifecycle_transition: LifecycleTransition,
}

#[derive(Debug,Eq,PartialEq,Serialize,Deserialize)]
pub enum LifecycleTransition {
    #[serde(rename="autoscaling:EC2_INSTANCE_LAUNCHING")]
    InstanceLaunching,
    #[serde(rename="autoscaling:EC2_INSTANCE_TERMINATING")]
    InstanceTerminating,
}

#[derive(Serialize,Deserialize)]
pub struct LifecycleEvent {
    pub id: String,
    #[serde(rename="detail-type")]
    pub event_type: EventType,
    pub account: String,
    pub time: DateTime<Utc>,
    pub region: String,
    pub resources: Vec<String>,
    pub detail: EventDetail,
}

impl LifecycleEvent {

    pub fn kind(&self) -> EventKind {
        self.event_type.kind()
    }

    pub fn status(&self) -> EventStatus {
        self.event_type.status()
    }

    pub fn duration(&self) -> Duration {
        self.detail.duration()
    }
}

#[derive(Debug,Eq,PartialEq)]
pub enum EventStatus {
    Success,
    Failure,
    Unknown,
}

#[derive(Debug,Eq,PartialEq)]
pub enum EventKind {
    Launch,
    Terminate,
    Unknown,
}

#[derive(Serialize,Deserialize)]
pub enum EventType {
    #[serde(rename="EC2 Instance Terminate Unsuccessful")]
    TerminateFailed,
    #[serde(rename="EC2 Instance Terminate Successful")]
    TerminateSuccess,
    #[serde(rename="EC2 Instance Launch Successful")]
    LaunchSuccess,
    #[serde(rename="EC2 Instance Launch Unsuccessful")]
    LaunchFailed,
    Unknown(String),
}

impl EventType {

    pub fn status(&self) -> EventStatus {
        match self {
            &EventType::LaunchFailed => EventStatus::Failure,
            &EventType::LaunchSuccess => EventStatus::Success,
            &EventType::TerminateFailed => EventStatus::Failure,
            &EventType::TerminateSuccess => EventStatus::Success,
            &EventType::Unknown(_) => EventStatus::Unknown
        }
    }

    pub fn kind(&self) -> EventKind {
        match self {
            &EventType::LaunchFailed => EventKind::Launch,
            &EventType::LaunchSuccess => EventKind::Launch,
            &EventType::TerminateFailed => EventKind::Terminate,
            &EventType::TerminateSuccess => EventKind::Terminate,
            &EventType::Unknown(_) => EventKind::Unknown
        }
    }
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct EventDetail {
    pub status_code: StatusCode,
    #[serde(rename="AutoScalingGroupName")]
    pub autoscaling_group_name: String,
    pub activity_id: String,
    pub request_id: String,
    pub end_time: DateTime<Utc>,
    pub start_time: DateTime<Utc>,
    #[serde(rename="EC2InstanceId")]
    pub ec2_instance_id: String,
    pub cause: String,
    pub details: SubnetDetails,
}

#[derive(Serialize,Deserialize)]
pub enum StatusCode {
    InProgress,
    Failed,
}

impl EventDetail {

    /// Return the amount of time this event took from start time to end time.
    pub fn duration(&self) -> Duration {
        let start_time = Duration::nanoseconds(
            (self.start_time.timestamp() as i64 * 1_000_000_000) +
                self.start_time.timestamp_subsec_nanos() as i64
        );

        let end_time = Duration::nanoseconds(
            (self.end_time.timestamp() as i64 * 1_000_000_000) +
                self.end_time.timestamp_subsec_nanos() as i64
        );

        let difference = if end_time > start_time {
            end_time - start_time
        } else {
            start_time - end_time
        };

        return difference
    }
}

#[derive(Serialize,Deserialize)]
pub struct SubnetDetails {
    #[serde(rename="Availability Zone")]
    pub availability_zone: String,
    #[serde(rename="Subnet ID")]
    pub subnet_id: String,
}
