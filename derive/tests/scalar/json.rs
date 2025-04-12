use std::collections::HashMap;

use dynamic_graphql::Name;
use dynamic_graphql::Scalar;
use dynamic_graphql::ScalarValue;
use dynamic_graphql::Value;

#[derive(Scalar)]
pub struct JsonValue(pub Value);

impl ScalarValue for JsonValue {
    fn from_value(value: Value) -> dynamic_graphql::Result<Self>
    where
        Self: Sized,
    {
        Ok(JsonValue(value))
    }

    fn to_value(&self) -> Value {
        self.0.clone()
    }
}

#[derive(Scalar)]
pub struct JsonObject(pub HashMap<String, Value>);

impl ScalarValue for JsonObject {
    fn from_value(value: Value) -> dynamic_graphql::Result<Self>
    where
        Self: Sized,
    {
        match value {
            Value::Object(o) => {
                let mut map = HashMap::new();
                for (k, v) in o {
                    map.insert(k.to_string(), v);
                }
                Ok(JsonObject(map))
            }
            _ => Err(dynamic_graphql::Error::new(
                "Expected an object value".to_string(),
            )),
        }
    }

    fn to_value(&self) -> Value {
        Value::Object(
            self.0
                .clone()
                .into_iter()
                .map(|(k, v)| (Name::new(k), v))
                .collect(),
        )
    }
}

#[derive(Scalar)]
pub struct KeyValue(HashMap<String, String>);

impl ScalarValue for KeyValue {
    fn from_value(value: Value) -> dynamic_graphql::Result<Self>
    where
        Self: Sized,
    {
        match value {
            Value::Object(o) => {
                let mut map = HashMap::new();
                for (k, v) in o {
                    match v {
                        Value::String(s) => {
                            map.insert(k.to_string(), s);
                        }
                        _ => {
                            return Err(dynamic_graphql::Error::new(
                                "Expected a string value".to_string(),
                            ));
                        }
                    }
                }
                Ok(KeyValue(map))
            }
            _ => Err(dynamic_graphql::Error::new(
                "Expected an object value".to_string(),
            )),
        }
    }

    fn to_value(&self) -> Value {
        Value::Object(
            self.0
                .clone()
                .into_iter()
                .map(|(k, v)| (Name::new(k), Value::String(v)))
                .collect(),
        )
    }
}
