use std::{collections::HashMap, time::UNIX_EPOCH};
use std::time::{Duration, SystemTime};
use crate::LoxError;
use crate::types::values::{callable::LoxCallable, Value};

pub fn get_stdlib() -> HashMap<String, Value> {
    HashMap::from([
        (
            String::from("clock"),
            Value::Callable(LoxCallable::Native(String::from("clock"), clock, 0))
        )
    ])
}

/// Returns the current time in miliseconds.
fn clock(_: Vec<Value>) -> Result<Value, LoxError> {
    let now: SystemTime = SystemTime::now();
    let duration: Duration = now.duration_since(UNIX_EPOCH).unwrap();
    Ok(Value::Float(duration.as_millis() as f64))
}