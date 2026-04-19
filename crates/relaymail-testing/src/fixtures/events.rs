pub fn direct_event() -> &'static str {
    include_str!("../../../../examples/events/s3-object-created.json")
}

pub fn sns_event() -> &'static str {
    include_str!("../../../../examples/events/sns-wrapped-s3-event.json")
}

pub fn eventbridge_event() -> &'static str {
    include_str!("../../../../examples/events/eventbridge-s3-event.json")
}
