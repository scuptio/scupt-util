use bincode::{Decode, Encode};
use crate::message::MsgTrait;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::serde_json_value::SerdeJsonValue;


#[derive(
Clone,
Serialize,
Hash,
PartialEq,
Eq,
Debug,
Deserialize,
Decode,
Encode,
)]
pub struct SerdeJsonString {
    string: String,
}


impl SerdeJsonString {
    pub fn new(json_string:String) -> Self {
        Self::from_json_string(json_string)
    }

    pub fn from_json_value(value:&Value) -> Self {
        let s = value.to_string();
        Self::new(s)
    }
    pub fn from_json_string(json_string:String) -> Self {
        Self {
            string: json_string,
        }
    }

    pub fn to_serde_json_value(&self) -> SerdeJsonValue {
        let v = serde_json::from_str(&self.string).unwrap();
        SerdeJsonValue::new( v)
    }

    pub fn to_string(&self) -> String {
        self.string.clone()
    }
}

impl MsgTrait for SerdeJsonString {

}