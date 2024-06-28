use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{ Hash, Hasher};

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::message::MsgTrait;
use crate::sj_value_ref::SJValueRef;
use crate::cmp_hash::cmp_hash;

/// struct member name, use zzz prefix
pub const STR_SET_ZZZ:&str = "zzz_array";



#[derive(
Clone,
Serialize,
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

impl<K: MsgTrait + 'static> PartialEq<Self> for MTSet<K> {
    fn eq(&self, other: &Self) -> bool {
        let s1 = self.to_set();
        let s2 = other.to_set();
        s1.eq(&s2)
    }
}

impl<K: MsgTrait + 'static> Eq for MTSet<K> {}

impl<K: MsgTrait + 'static> Hash for MTSet<K> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut vec = self.zzz_array.clone();
        vec.sort_by(|x, y|{
            cmp_hash(x, y)
        });
        for k in vec {
            k.hash(state)
        }
    }
}

impl <K:MsgTrait + 'static> MTSet<K> {
    pub fn from_vec(vec:Vec<K>) -> Self {
        let mut keys = HashSet::new();
        for k in vec.iter() {
            if !keys.contains(k) {
                keys.insert(k.clone());
            } else {
                panic!("MTSet, existing key {:?}", k);
            }
        }
        Self {
            zzz_array:vec
        }
    }

    pub fn vec(&self) -> &Vec<K> {
        &self.zzz_array
    }

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
                panic!("MTSet, existing key {:?}", e);
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
