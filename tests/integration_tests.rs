use sinteflake::{next_id, next_id_with_hash, set_instance_id};

#[test]
fn test_basic() {
    set_instance_id(42).unwrap();

    let id_a = next_id().unwrap();
    let id_b = next_id().unwrap();
    assert_ne!(id_a, id_b);
}

#[test]
fn test_with_hash() {
    set_instance_id(42).unwrap();

    let data = [1, 2, 3];
    let id_a = next_id_with_hash(&data).unwrap();
    let id_b = next_id_with_hash(&data).unwrap();
    assert_ne!(id_a, id_b);
}

#[test]
fn test_custom() {
    use sinteflake::sinteflake::SINTEFlake;
    use time::OffsetDateTime;

    let mut instance = SINTEFlake::custom(
        42,                                                       // instance_id
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],  // hash_key
        123,                                                      // counter hash key
        OffsetDateTime::from_unix_timestamp(1719792000).unwrap(), // epoch
    )
    .unwrap();

    let id_a = instance.next_id().unwrap();
    let id_b = instance.next_id().unwrap();
    assert_ne!(id_a, id_b);
}
