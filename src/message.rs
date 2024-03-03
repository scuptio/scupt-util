use std::fmt::Debug;
use std::hash::Hash;
use std::marker::Send;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use serde_json::{json, Value};
use tracing::error;

use crate::serde_json_string::SerdeJsonString;
use crate::error_type::ET;

use crate::node_id::NID;
use crate::res::Res;
use crate::serde_json_value::SerdeJsonValue;

pub trait MsgTrait:
Eq
+ PartialEq
+ Debug
+ Hash
+ Clone
+ Serialize
+ DeserializeOwned
+ Encode
+ Decode
+ Send
+ Sync
+ 'static
{}


// message with source and dest node id
#[derive(
Clone,
Hash,
PartialEq,
Eq,
Debug,
Serialize,
Deserialize,
Decode,
Encode,
)]
pub struct Message<M: MsgTrait + 'static> {
    source: NID,
    dest: NID,
    #[serde(bound = "M: MsgTrait")]
    payload: M,
}

impl<M: MsgTrait> MsgTrait for Message<M> {}

impl<M: MsgTrait> Message<M> {
    pub fn new(m: M, source: NID, dest: NID) -> Message<M> {
        Message {
            source,
            dest,
            payload: m,
        }
    }

    pub fn source(&self) -> NID {
        self.source
    }

    pub fn dest(&self) -> NID {
        self.dest
    }

    pub fn payload_ref(&self) -> &M {
        &self.payload
    }

    pub fn payload(self) -> M {
        self.payload
    }

    pub fn map<U, F>(self, f: F) -> Message<U>
        where U: MsgTrait + 'static,
              F:  Fn(M) -> U + 'static
    {
        let payload = f(self.payload);
        Message {
            source: self.source,
            dest: self.dest,
            payload,
        }
    }

    pub fn build_json_value(payload_s:String, source:NID, dest:NID) -> Res<Value> {
        let payload:Value = serde_json::from_str(payload_s.as_str())
            .map_err(|e| ET::ParseError(e.to_string()) )?;
        let v = json!({
            "source":source,
            "dest":dest,
            "payload":payload
        });
        Ok(v)
    }

    pub fn build_json_str(payload_s:String, source:NID, dest:NID) -> Res<String> {
        let j = Self::build_json_value(payload_s, source, dest)?;
        Ok(j.to_string())
    }
}

pub fn decode_message<M: MsgTrait + 'static>(slice: &[u8]) -> Res<(M, usize)> {
    let r_decode = bincode::decode_from_slice::<M, _>(
        slice,
        bincode::config::standard());
    match r_decode {
        Ok(r) => { Ok(r) }
        Err(_e) => { return Err(ET::FatalError("message decode error".to_string())); }
    }
}

pub fn encode_message<M: MsgTrait + 'static, >(m: M) -> Res<Vec<u8>> {
    let r_encode =
        bincode::encode_to_vec(
            m,
            bincode::config::standard(),
        );
    match r_encode {
        Ok(r) => { Ok(r) }
        Err(_e) => { return Err(ET::FatalError("message encode error".to_string())); }
    }
}


impl MsgTrait for i64 {}

impl MsgTrait for i32 {}

impl MsgTrait for i128 {}

impl MsgTrait for u64 {}

impl MsgTrait for u32 {}

impl MsgTrait for u128 {}

impl MsgTrait for () {}

impl MsgTrait for String {

}

pub fn message_source_id(s:&SerdeJsonValue) -> Option<NID>{
    let map = s.serde_json_value_ref().as_object()?;
    let v = map.get(&"source".to_string())?;
    let nid = v.as_u64()?;
    return Some(nid)
}

pub fn message_dest_id(s:&SerdeJsonValue) -> Option<NID> {
    let map = s.serde_json_value_ref().as_object()?;
    let v = map.get(&"dest".to_string())?;
    let nid = v.as_u64()?;
    return Some(nid)
}

impl <T:MsgTrait + 'static> MsgTrait for Vec<T> {

}

// utility for testing message
pub fn test_check_message<M:MsgTrait + 'static>(m: M) -> bool {
    let debug_str = format!("{:?}", m);
    if debug_str.is_empty() {
        return false;
    }
    let mut set = HashSet::new();
    let r = serde_json::to_string_pretty(&m);
    let s = if let Ok(s) = r {
        s
    } else {
        error!("to json string error {:?}", m);
        return false;
    };

    set.insert(m.clone());
    let ss = SerdeJsonString::new(s);
    let sv = ss.to_serde_json_value();
    let v = sv.into_serde_json_value();
    let m1 : M = serde_json::from_value(v).unwrap();
    if !m1.eq(&m) {
        error!("message is not equal {:?} {:?}", m1, m);
        return false;
    }

    let ok = set.remove(&m1);
    if !ok {
        error!("no such message {:?}", m1);
        return false;
    }
    let encoded = encode_message(m1.clone()).unwrap();
    let (m2, _)= decode_message::<M>(encoded.as_slice()).unwrap();
    if !m2.eq(&m) {
        error!("message is not equal {:?} {:?}", m2, m);
        return false;
    }

    return true;
}

