use crate::message::MsgTrait;
use std::collections::HashSet;
use std::hash::Hash;
use serde::{Deserialize, Serialize};
use bincode::{Decode, Encode};
use std::fmt::Debug;
use serde_json::{Map, Value};
use crate::sj_value_ref::SJValueRef;


/// struct member name, use zzz prefix
pub const STR_SET_ZZZ:&str = "zzz_array";



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
pub struct MTSet<K:MsgTrait + 'static> {
    #[serde(bound = "K:MsgTrait")]
    pub zzz_array: Vec<K>
}

impl <K:MsgTrait + 'static> MsgTrait for MTSet<K> {

}

impl <K:MsgTrait + 'static>  Default for MTSet<K> {
    fn default() -> Self {
        Self {
            zzz_array: vec![],
        }
    }
}

impl <K:MsgTrait + 'static> MTSet<K> {
    pub fn new(set:HashSet<K>) -> Self {
        let mut vec = vec![];
        for e in set {
            vec.push(e);
        }
        Self {
            zzz_array:vec,
        }
    }

    pub fn to_set(&self) -> HashSet<K> {
        let mut set = HashSet::new();
        for e in self.zzz_array.iter() {
            let ok = set.insert(e.clone());
            if !ok {
                panic!("existing key {:?}", e);
            }
        }
        set
    }
}


pub fn mt_set_from_vec(set:Vec<Value>) -> Option<Value> {
    let mut _set = HashSet::new();
    for e in set.iter() {
        let ok = _set.insert(SJValueRef::from(e));
        if !ok {
            return None;
        }
    }

    let mut vec = Vec::new();
    for e in set {
        vec.push(e);
    }
    let mut map = Map::new();
    map.insert(STR_SET_ZZZ.to_string(), Value::Array(vec));
    Some(Value::Object(map))
}

pub fn mt_set_to_vec(v:Value) -> Option<Vec<Value>> {
    if let Some(map) = v.as_object() {
        if map.len() == 1 && map.contains_key(&STR_SET_ZZZ.to_string()) {
            if let Some(v) = map.get(&STR_SET_ZZZ.to_string())  {
                if let Some(a) = v.as_array() {
                    return Some(a.clone())
                }
            }
        }
    }
    None
}