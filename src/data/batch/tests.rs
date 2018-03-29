use super::*;

use serde_json;

#[test]
fn test_state_change_failed() {
    let event: JobStateChangeEvent = serde_json::from_str(
        include_str!("fixtures/state-change-failed.json")
    ).unwrap();

    assert_eq!(JobStatus::Failed, event.detail.status);
}

#[test]
fn test_state_change_pending() {
    let event: JobStateChangeEvent = serde_json::from_str(
        include_str!("fixtures/state-change-pending.json")
    ).unwrap();

    assert_eq!(JobStatus::Pending, event.detail.status);
}

#[test]
fn test_state_change_runnable() {
    let event: JobStateChangeEvent = serde_json::from_str(
        include_str!("fixtures/state-change-runnable.json")
    ).unwrap();

    assert_eq!(JobStatus::Runnable, event.detail.status);
}

#[test]
fn test_state_change_running() {
    let event: JobStateChangeEvent = serde_json::from_str(
        include_str!("fixtures/state-change-running.json")
    ).unwrap();

    assert_eq!(JobStatus::Running, event.detail.status);
}

#[test]
fn test_state_change_starting() {
    let event: JobStateChangeEvent = serde_json::from_str(
        include_str!("fixtures/state-change-starting.json")
    ).unwrap();

    assert_eq!(JobStatus::Starting, event.detail.status);
}

#[test]
fn test_state_change_submitted() {
    let event: JobStateChangeEvent = serde_json::from_str(
        include_str!("fixtures/state-change-submitted.json")
    ).unwrap();

    assert_eq!(JobStatus::Submitted, event.detail.status);
}

#[test]
fn test_state_change_succeeded() {
    let event: JobStateChangeEvent = serde_json::from_str(
        include_str!("fixtures/state-change-succeeded.json")
    ).unwrap();

    assert_eq!(JobStatus::Succeeded, event.detail.status);
}
