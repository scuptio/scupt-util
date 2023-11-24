use std::hash::{Hash, Hasher};

use serde_json::Value;
use crate::sj_value_ref::SJValueRef;

use crate::serde_json_string::SerdeJsonString;

#[derive(Clone, Debug)]
pub struct SerdeJsonValue {
    value: Value,
}

impl SerdeJsonValue {
    pub fn new(json: Value) -> Self {
        Self {
            value: json,
        }
    }

    pub fn to_serde_json_string(&self) -> SerdeJsonString {
        SerdeJsonString::new( self.value.to_string())
    }

    pub fn serde_json_value_ref(&self) -> &Value {
        &self.value
    }

    pub fn into_serde_json_value(self) -> Value {
        self.value
    }
}

impl Hash for SerdeJsonValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let v = SJValueRef::from(&self.value);
        v.hash(state);
    }
}

impl PartialEq for SerdeJsonValue {
    fn eq(&self, other: &Self) -> bool {
        SJValueRef::from(&self.value).eq(&SJValueRef::from(&other.value))
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for SerdeJsonValue {

}
