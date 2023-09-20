use std::fmt::Debug;
use std::hash::Hash;
use std::marker::{Destruct, Send};

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::error_type::ET;
use crate::node_id::NID;
use crate::res::Res;

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
pub enum Direction {
    C2S,
    S2C,
    LOCAL,
}

// message with source and dest node id
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
              F: ~ const FnOnce(M) -> U + 'static,
              F: ~ const Destruct,
    {
        let payload = f(self.payload);
        Message {
            source: self.source,
            dest: self.dest,
            payload,
        }
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

impl MsgTrait for u64 {}

impl MsgTrait for u32 {}

impl MsgTrait for () {}