use super::*;

use std::fmt;

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Record {
    pub aws_region: String,
    pub event_name: ObjectEvent,
    pub event_time: String,
    pub event_version: String,
    pub request_parameters: Option<BTreeMap<String, String>>,
    pub response_elements: Option<BTreeMap<String, String>>,
    #[serde(rename="s3")]
    pub event: Event,
    pub user_identity: Option<BTreeMap<String, String>>,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Event {
    pub configuration_id: Option<String>,
    pub object: Object,
    pub bucket: Bucket,
    #[serde(rename="s3SchemaVersion")]
    pub schema_version: String,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Bucket {
    pub arn: String,
    pub name: String,
    pub owner_identity: BucketOwnerIdentity,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct BucketOwnerIdentity {
    pub principal_id: String,
}

#[derive(Serialize,Deserialize)]
pub struct Object {
    pub key: String,
    pub sequencer: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub enum ObjectEvent {
    #[serde(rename="ObjectCreated:Put")]
    Put,
    #[serde(rename="ObjectCreated:Post")]
    Post,
    #[serde(rename="ObjectCreated:Copy")]
    Copied,
    #[serde(rename="ObjectCreated:CompleteMultipartUpload")]
    CompleteMultipartUpload,
    #[serde(rename="ObjectRemoved:Delete")]
    Delete,
    #[serde(rename="ObjectRemoved:DeleteMarkerCreated")]
    DeleteMarkerCreated,
    #[serde(rename="ReducedRedundancyLostObject")]
    LostObject,
}

impl fmt::Display for ObjectEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           ObjectEvent::Put                     => write!(f, "s3:ObjectCreated:Put"),
           ObjectEvent::Post                    => write!(f, "s3:ObjectCreated:Post"),
           ObjectEvent::Copied                  => write!(f, "s3:ObjectCreated:Copy"),
           ObjectEvent::CompleteMultipartUpload => write!(f, "s3:ObjectCreated:CompleteMultipartUpload"),
           ObjectEvent::Delete                  => write!(f, "s3:ObjectRemoved:Delete"),
           ObjectEvent::DeleteMarkerCreated     => write!(f, "s3:ObjectRemvoed:DeleteMarkerCreated"),
           ObjectEvent::LostObject              => write!(f, "s3:ObjectRemoved:LostObject"),
       }
    }
}
