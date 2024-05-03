use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::message::MsgTrait;
use crate::sj_value_ref::SJValueRef;

/// struct member name
pub const STR_KEY: &str = "key";
pub const STR_VALUE: &str = "value";

pub const STR_MAP_ZZZ: &str = "zzz_array";

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
pub struct KeyValue<K: MsgTrait + 'static, V: MsgTrait + 'static> {
    #[serde(bound = "K: MsgTrait")]
    pub key: K,
    #[serde(bound = "V: MsgTrait")]
    pub value: V,
}


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
pub struct MTMap<K: MsgTrait + 'static, V: MsgTrait + 'static> {
    #[serde(bound = "K:MsgTrait, V: MsgTrait")]
    pub zzz_array: Vec<KeyValue<K, V>>,
}

impl<K: MsgTrait + 'static, V: MsgTrait + 'static> MsgTrait for MTMap<K, V> {}

impl<K: MsgTrait + 'static, V: MsgTrait + 'static> Default for MTMap<K, V> {
    fn default() -> Self {
        Self {
            zzz_array: vec![],
        }
    }
}

impl<K: MsgTrait + 'static, V: MsgTrait + 'static> MTMap<K, V> {
    pub fn from_vec(vec: Vec<(K, V)>) -> Self {
        let mut keys = HashSet::new();
        let mut kv_vec = vec![];
        for (k, v) in vec {
            if !keys.contains(&k) {
                keys.insert(k.clone());
            } else {
                panic!("MTMap, existing key {:?}", k);
            }

            let kv = KeyValue {
                key: k,
                value: v,
            };

            kv_vec.push(kv);
        }
        Self {
            zzz_array: kv_vec,
        }
    }

    pub fn to_vec(&self) -> Vec<(K, V)> {
        self.zzz_array.iter().map(|x| {
            (x.key.clone(), x.value.clone())
        }).collect()
    }

    pub fn new(map: HashMap<K, V>) -> Self {
        let mut vec = vec![];
        for (k, v) in map {
            let kv = KeyValue {
                key: k,
                value: v,
            };
            vec.push(kv);
        }
        Self {
            zzz_array: vec,
        }
    }

    pub fn to_map(&self) -> HashMap<K, V> {
        let mut map = HashMap::new();
        for kv in self.zzz_array.iter() {
            let opt = map.insert(kv.key.clone(), kv.value.clone());
            if opt.is_some() {
                panic!("MTMap, existing key {:?}", kv.key);
            }
        }
        map
    }
}

pub fn mt_map_from_vec(kv: Vec<(Value, Value)>) -> Option<Value> {
    let mut set = HashSet::new();
    for (k, _) in kv.iter() {
        let ok = set.insert(SJValueRef::from(k));
        if !ok {
            return None;
        }
    }

    let mut vec = Vec::new();
    for (k, v) in kv {
        let mut map_kv = Map::new();

        let _o = map_kv.insert(STR_KEY.to_string(), k);

        assert!(_o.is_none());
        let _o = map_kv.insert(STR_VALUE.to_string(), v);
        assert!(_o.is_none());
        vec.push(Value::Object(map_kv));
    }
    let mut map = Map::new();
    map.insert(STR_MAP_ZZZ.to_string(), Value::Array(vec));
    Some(Value::Object(map))
}


pub fn mt_map_to_vec(v: Value) -> Option<Vec<(Value, Value)>> {
    if let Some(map) = v.as_object() {
        if map.len() == 1 && map.contains_key(&STR_MAP_ZZZ.to_string()) {
            if let Some(v) = map.get(&STR_MAP_ZZZ.to_string()) {
                if let Some(vec_kv) = v.as_array() {
                    if vec_kv.is_empty() {
                        return Some(vec![]);
                    }
                    let mut vec = vec![];
                    for kv in vec_kv {
                        if let Some(kv_map) = kv.as_object() {
                            if let (Some(k), Some(v)) = (kv_map.get(STR_KEY), kv_map.get(STR_VALUE)) {
                                vec.push((k.clone(), v.clone()));
                            }
                        } else {
                            return None;
                        }
                    }
                    return Some(vec);
                }
            }
        }
    }
    None
}