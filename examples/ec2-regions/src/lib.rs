#[macro_use(lambda)]
extern crate crowbar;
#[macro_use]
extern crate cpython;
extern crate rusoto;

use crowbar::{Value, LambdaContext, LambdaResult};
use rusoto::ec2::{Ec2Client, DescribeRegionsRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use std::default::Default;
use std::env;
use std::str::FromStr;

fn list_regions(_: Value, _: LambdaContext) -> LambdaResult {
    let provider = DefaultCredentialsProvider::new()?;
    let region_str = env::var("AWS_DEFAULT_REGION")?;
    let client = Ec2Client::new(provider, Region::from_str(&region_str)?);
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
