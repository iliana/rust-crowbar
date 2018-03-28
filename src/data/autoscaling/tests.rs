use super::*;

use serde_json;

#[test]
fn test_lifecycle_action_launch() {
    let action: LifecycleAction = serde_json::from_str(
        include_str!("fixtures/instance-lifecycle-launch.json")
    ).unwrap();

    assert_eq!(action.detail.lifecycle_transition, LifecycleTransition::InstanceLaunching);
}

#[test]
fn test_lifecycle_action_terminate() {
    let action: LifecycleAction = serde_json::from_str(
        include_str!("fixtures/instance-lifecycle-terminate.json")
    ).unwrap();

    assert_eq!(action.detail.lifecycle_transition, LifecycleTransition::InstanceTerminating);
}
