use bytes::Bytes;
use chrono::Duration;
use relaymail_core::message_source::MessageSource;
use relaymail_core::object_store::{ObjectStore, TagSet};
use relaymail_core::{Clock, ObjectId, ObjectMetadata, RawEnvelope};
use relaymail_testing::{
    FakeClock, FakeEnvelopeBuilder, FakeMessageSource, FakeObjectStore, TagRecord,
};

#[tokio::test]
async fn fake_message_source_roundtrip() {
    let src = FakeMessageSource::new();
    let env = FakeEnvelopeBuilder::new("m1")
        .with_body(Bytes::from_static(b"body"))
        .build();
    src.enqueue(env);
    let pulled = src.receive().await.unwrap();
    assert_eq!(pulled.len(), 1);
    let env2 = RawEnvelope::new("m2", Bytes::new(), "h2");
    src.ack(&env2).await.unwrap();
    src.nack(&env2).await.unwrap();
    src.extend_visibility(&env2, 60).await.unwrap();
    assert_eq!(src.acks(), vec!["m2"]);
    assert_eq!(src.nacks(), vec!["m2"]);
    assert_eq!(src.extensions(), vec![("m2".to_string(), 60)]);
}

#[tokio::test]
async fn fake_object_store_records_operations() {
    let store = FakeObjectStore::new();
    let id = ObjectId::new("b", "k");
    store.put(
        id.clone(),
        Bytes::from_static(b"X"),
        ObjectMetadata::new("e", 1),
    );
    let fetched = store.fetch(&id, 10).await.unwrap();
    assert_eq!(fetched.bytes.as_ref(), b"X");
    let mut tags = TagSet::new();
    tags.insert("a", "1");
    store.tag(&id, &tags).await.unwrap();
    store.move_to(&id, "dest").await.unwrap();
    store.delete(&id).await.unwrap();
    let records: Vec<TagRecord> = store.tag_records();
    assert_eq!(records.len(), 1);
    assert_eq!(store.moves()[0].1, "dest");
    assert_eq!(store.deletes()[0], id);
}

#[test]
fn fake_clock_advances() {
    let c = FakeClock::epoch();
    let t1 = c.now();
    c.advance(Duration::try_seconds(3600).unwrap());
    let t2 = c.now();
    assert!(t2 > t1);
}
