//! Serde serialization helpers.

use serde::{Serialize, Serializer};
use serde_json::Value;

/// Serializes any serializable type with snake_case field names.
pub fn serialize_as_snake_case<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    // Serialize to Value first
    let value = serde_json::to_value(value).map_err(serde::ser::Error::custom)?;
    // Recursively rename all keys to snake_case
    let snake_cased = rename_keys_snake_case(value);
    // Result
    snake_cased.serialize(serializer)
}

/// Recursively renames all object keys to snake_case.
fn rename_keys_snake_case(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let new_map = map
                .into_iter()
                .map(|(k, v)| {
                    let snake_key = to_snake_case(&k);
                    (snake_key, rename_keys_snake_case(v))
                })
                .collect();
            Value::Object(new_map)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(rename_keys_snake_case).collect()),
        other => other,
    }
}

/// Converts PascalCase or camelCase to snake_case.
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap_or(c));
    }
    result
}
