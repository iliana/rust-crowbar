/// More information available at: https://docs.aws.amazon.com/batch/latest/APIReference/API_DescribeJobs.html
#[cfg(test)]
mod tests;

use super::*;

#[derive(Serialize,Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct JobStateChangeEvent {
    pub id: String,
    pub detail_type: String,
    pub source: String,
    pub account: String,
    pub time: DateTime<Utc>,
    pub region: String,
    pub resources: Vec<String>,
    pub detail: JobDetail,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct JobDetail {
    pub job_name: String,
    pub job_id: String,
    pub job_queue: String,
    pub status: JobStatus,
    pub attempts: Vec<JobAttempt>,
    pub created_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub stopped_at: Option<DateTime<Utc>>,
    pub retry_strategy: RetryStrategy,
    pub depends_on: Vec<JobDependency>,
    pub job_definition: String,
    pub parameters: BTreeMap<String, String>,
    pub container: ContainerInfo,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct JobAttempt {
    pub container: JobAttemptContainer,
    pub started_at: DateTime<Utc>,
    pub stopped_at: DateTime<Utc>,
    pub status_reason: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct JobAttemptContainer {
    pub container_instance_arn: String,
    pub exit_code: u16,
    pub log_stream_name: String,
    pub reason: String,
    pub task_arn: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct JobDependency {
    pub job_id: String,
    #[serde(rename="type")]
    pub job_type: String,
}

#[derive(Serialize,Deserialize)]
pub struct RetryStrategy {
    pub attempts: u64,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ContainerInfo {
    pub command: Option<String>,
    pub container_instance_arn: String,
    pub environment: Vec<BTreeMap<String, String>>,
    pub exit_code: u16,
    pub image: String,
    pub job_role_arn: String,
    pub log_stream_name: String,
    pub memory: u64,
    pub mount_points: Vec<ContainerMountPoint>,
    pub privileged: bool,
    pub read_only_root_filesystem: bool,
    pub reason: String,
    pub task_arn: String,
    pub ulimits: Vec<Ulimit>,
    pub user: String,
    pub vcpus: u64,
    pub volumes: Vec<ContainerVolume>,
}

#[derive(Serialize,Deserialize)]
pub struct Ulimit {
    pub hard_limit: u64,
    pub soft_limit: u64,
    pub name: String,
}

#[derive(Serialize,Deserialize)]
pub struct ContainerVolume {
    pub host: ContainerVolumeHost,
    pub name: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ContainerVolumeHost {
    pub source_path: String,
}

#[derive(Serialize,Deserialize)]
pub struct ContainerMountPoint {
    pub container_path: String,
    pub read_only: bool,
    pub source_volume: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    Failed,
    Pending,
    Runnable,
    Running,
    Starting,
    Submitted,
    Suceeded,
}
