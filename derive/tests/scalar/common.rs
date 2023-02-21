use std::net::IpAddr;

use dynamic_graphql::Scalar;
use dynamic_graphql::ScalarValue;

#[derive(Scalar)]
#[graphql(name = "IP")]
pub struct IP(pub IpAddr);

pub struct StringValue(String);
impl TryFrom<dynamic_graphql::Value> for StringValue {
    type Error = dynamic_graphql::Error;

    fn try_from(value: dynamic_graphql::Value) -> Result<Self, Self::Error> {
        match value {
            dynamic_graphql::Value::String(s) => Ok(StringValue(s)),
            _ => Err(dynamic_graphql::Error::new(
                "Expected a string value".to_string(),
            )),
        }
    }
}

impl ScalarValue for IP {
    fn from_value(value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self>
    where
        Self: Sized,
    {
        let value = StringValue::try_from(value)?.0;
        let value = value.parse::<IpAddr>()?;
        Ok(IP(value))
    }

    fn to_value(&self) -> dynamic_graphql::Value {
        self.0.to_string().into()
    }
}

#[derive(Scalar)]
pub struct MyString(pub String);

impl ScalarValue for MyString {
    fn from_value(value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self>
    where
        Self: Sized,
    {
        let value = StringValue::try_from(value)?.0;
        Ok(MyString(value))
    }

    fn to_value(&self) -> dynamic_graphql::Value {
        self.0.clone().into()
    }
}
