use core::fmt;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub(crate) fn check_serialization_roundtrip<T>(obj: &T, expected_json: &str, expected_rmp_hex: &str)
where
    T: fmt::Debug + PartialEq + Serialize + DeserializeOwned,
{
    // Check serialization to JSON (human-readable)

    let serialized = serde_json::to_string(obj).unwrap();
    assert_eq!(serialized, expected_json);

    let deserialized: T = serde_json::from_str(&serialized).unwrap();
    assert_eq!(obj, &deserialized);

    // Check serialization to MessagePack (binary)

    let serialized = rmp_serde::to_vec(obj).unwrap();
    assert_eq!(hex::encode(&serialized), expected_rmp_hex);

    let deserialized: T = rmp_serde::from_slice(&serialized).unwrap();
    assert_eq!(obj, &deserialized);
}
