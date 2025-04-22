use rhai::{Dynamic, Map};
use serde_json::Value;
use std::result::Result;

pub fn json_to_dynamic(value: Value) -> Dynamic {
    match value {
        Value::Null => Dynamic::UNIT,
        Value::Bool(b) => Dynamic::from(b),
        Value::Number(n) => {
            if n.is_i64() {
                Dynamic::from(n.as_i64().unwrap())
            } else if n.is_u64() {
                Dynamic::from(n.as_u64().unwrap() as i64)
            } else {
                Dynamic::from(n.as_f64().unwrap())
            }
        }
        Value::String(s) => Dynamic::from(s),
        Value::Array(arr) => {
            let mut rhai_array = Vec::new();
            for item in arr {
                rhai_array.push(json_to_dynamic(item));
            }
            Dynamic::from(rhai_array)
        }
        Value::Object(obj) => {
            let mut rhai_map = Map::new();
            for (key, value) in obj {
                rhai_map.insert(key.into(), json_to_dynamic(value));
            }
            Dynamic::from(rhai_map)
        }
    }
}

pub fn dynamic_to_json(value: Dynamic) -> Result<Value, String> {
    match value.type_name() {
        "()" => Ok(Value::Null),
        "bool" => Ok(Value::Bool(value.as_bool().unwrap())),
        "i32" | "i64" => Ok(Value::Number(serde_json::Number::from(
            value.as_int().unwrap(),
        ))),
        "f32" | "f64" => {
            let float_val = value.as_float().unwrap();
            match serde_json::Number::from_f64(float_val) {
                Some(num) => Ok(Value::Number(num)),
                None => Err(format!("Cannot convert float {} to JSON number", float_val)),
            }
        }
        "string" | "ImmutableString" => {
            Ok(Value::String(value.into_immutable_string()?.to_string()))
        }
        "array" => {
            let array = value.into_array().unwrap();
            let mut json_array = Vec::new();
            for item in array {
                json_array.push(dynamic_to_json(item)?);
            }
            Ok(Value::Array(json_array))
        }
        "map" => {
            let map: Map = value.cast::<Map>();
            let mut json_map = serde_json::Map::new();
            for (key, val) in map {
                json_map.insert(key.to_string(), dynamic_to_json(val)?);
            }
            Ok(Value::Object(json_map))
        }
        other => Err(format!("Cannot convert {} to JSON", other)),
    }
}
