#[macro_use]
extern crate cpython;
#[macro_use(lambda)]
extern crate crowbar;
extern crate rusoto_core;
extern crate rusoto_ec2;

use crowbar::{LambdaContext, LambdaResult, Value};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ec2::{DescribeRegionsRequest, Ec2, Ec2Client};
use std::default::Default;
use std::env;
use std::str::FromStr;

fn list_regions(_: Value, _: LambdaContext) -> LambdaResult {
    let provider = DefaultCredentialsProvider::new()?;
    let region_str = env::var("AWS_DEFAULT_REGION")?;
    let client = Ec2Client::new(
        default_tls_client()?,
        provider,
        Region::from_str(&region_str)?,
    );
    let input: DescribeRegionsRequest = Default::default();

    match client.describe_regions(&input)?.regions {
        Some(regions) => {
            let mut v = vec![];
            for region in regions {
                match region.region_name {
                    Some(s) => v.push(Value::String(s)),
                    _ => {}
                }
            }
            Ok(Value::Array(v))
        }
        None => Ok(Value::Array(vec![])),
    }
}

lambda!(list_regions);
