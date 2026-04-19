use relaymail_runtime::ReadinessTracker;

#[test]
fn snapshot_reflects_registration_and_marking() {
    let t = ReadinessTracker::new();
    t.register("a");
    t.register("b");
    let snap = t.snapshot();
    assert_eq!(snap.len(), 2);
    assert!(snap.iter().all(|(_, ready)| !ready));

    t.mark_ready("a");
    t.mark_ready("b");
    assert!(t.is_ready());
}

#[test]
fn empty_tracker_is_not_ready() {
    let t = ReadinessTracker::new();
    assert!(!t.is_ready());
}
