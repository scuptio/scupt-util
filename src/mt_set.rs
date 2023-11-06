use crate::message::MsgTrait;
use std::collections::HashSet;
use std::hash::Hash;
use serde::{Deserialize, Serialize};
use bincode::{Decode, Encode};
use std::fmt::Debug;
use serde_json::{Map, Value};
use crate::error_type::ET;
use crate::res::Res;
use crate::sj_value_ref::SJValueRef;


/// struct member name, use zzz prefix
pub const STR_SET:&str = "zzz_set";



#[derive(
Default,
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
pub struct MTSet<K:MsgTrait + 'static> {
    #[serde(bound = "K:MsgTrait")]
    pub zzz_set: Vec<K>
}

impl <K:MsgTrait + 'static> MsgTrait for MTSet<K> {

}

impl <K:MsgTrait + 'static> MTSet<K> {
    pub fn new(&self, set:HashSet<K>) -> Self {
        let mut vec = vec![];
        for e in set {
            vec.push(e);
        }
        Self {
            zzz_set:vec,
        }
    }

    pub fn to_hash_map(&self) -> HashSet<K> {
        let mut set = HashSet::new();
        for e in self.zzz_set.iter() {
            let ok = set.insert(e.clone());
            if ok {
                panic!("existing key {:?}", e);
            }
        }
        set
    }
}


pub fn mt_set_from_value(set:Vec<Value>) -> Res<Value> {
    let mut _set = HashSet::new();
    for e in set.iter() {
        let ok = _set.insert(SJValueRef::from(e));
        if !ok {
            return Err(ET::ExistingSuchKey);
        }
    }

    let mut vec = Vec::new();
    for e in set {
        vec.push(e);
    }
    let mut map = Map::new();
    map.insert(STR_SET.to_string(), Value::Array(vec));
    Ok(Value::Object(map))
}