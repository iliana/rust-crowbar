use super::*;

use serde_json;

#[test]
fn test_state_change_failed() {
    let _event: JobStateChangeEvent = serde_json::from_str(
        include_str!("fixtures/state-change-failed.json")
    ).unwrap();
}
