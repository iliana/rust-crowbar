use super::*;

use serde_json;

#[test]
fn test_deserialize() {
    let _call: APICall = serde_json::from_str(include_str!("fixtures/default.json")).unwrap();
}
