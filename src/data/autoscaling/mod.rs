#[cfg(test)]
mod tests;

use super::*;

#[derive(Serialize,Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct LifecycleAction {
    pub account: String,
    pub id: String,
    pub detail: LifecycleActionDetail,
    pub detail_type: String,
    pub region: String,
    pub resources: Vec<String>,
    pub time: DateTime<Utc>,
    pub version: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct LifecycleActionDetail {
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
