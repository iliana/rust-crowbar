use super::*;

use serde_json;

#[test]
fn test_lifecycle_action_launch() {
    let action: Action = serde_json::from_str(
        include_str!("fixtures/instance-lifecycle-launch.json")
    ).unwrap();

    assert_eq!(action.detail.lifecycle_transition, LifecycleTransition::InstanceLaunching);
}

#[test]
fn test_lifecycle_action_terminate() {
    let action: Action = serde_json::from_str(
        include_str!("fixtures/instance-lifecycle-terminate.json")
    ).unwrap();

    assert_eq!(action.detail.lifecycle_transition, LifecycleTransition::InstanceTerminating);
}

#[test]
fn test_lifecycle_event_launch_failure() {
    let event: LifecycleEvent = serde_json::from_str(
        include_str!("fixtures/instance-launch-failure.json")
    ).unwrap();

    assert_eq!(event.status(), EventStatus::Failure);
    assert_eq!(event.kind(), EventKind::Launch);

    assert_eq!(Duration::nanoseconds(698_000_000), event.duration());
}

#[test]
fn test_lifecycle_event_launch_success() {
    let event: LifecycleEvent = serde_json::from_str(
        include_str!("fixtures/instance-launch-success.json")
    ).unwrap();

    assert_eq!(event.status(), EventStatus::Success);
    assert_eq!(event.kind(), EventKind::Launch);

    assert_eq!(Duration::nanoseconds(33_537_000_000), event.duration());
}

#[test]
fn test_lifecycle_event_terminate_failure() {
    let event: LifecycleEvent = serde_json::from_str(
        include_str!("fixtures/instance-terminate-failure.json")
    ).unwrap();

    assert_eq!(event.status(), EventStatus::Failure);
    assert_eq!(event.kind(), EventKind::Terminate);

    assert_eq!(Duration::nanoseconds(69_232_000_000), event.duration());
}

#[test]
fn test_lifecycle_event_terminate_success() {
    let event: LifecycleEvent = serde_json::from_str(
        include_str!("fixtures/instance-terminate-success.json")
    ).unwrap();

    assert_eq!(event.status(), EventStatus::Success);
    assert_eq!(event.kind(), EventKind::Terminate);

    assert_eq!(Duration::nanoseconds(44_849_000_000), event.duration());
}
