use crate::message::MsgTrait;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use serde::{Deserialize, Serialize};
use bincode::{Decode, Encode};
use std::fmt::Debug;
use serde_json::{Map, Value};
use crate::error_type::ET;
use crate::res::Res;
use crate::sj_value_ref::SJValueRef;


/// struct member name
pub const STR_KEY:&str = "key";
pub const STR_VALUE:&str = "value";

pub const STR_MAP:&str = "zzz_array";

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
pub struct KeyValue<K:MsgTrait + 'static, V:MsgTrait + 'static> {
    #[serde(bound = "K: MsgTrait")]
    pub key:K,
    #[serde(bound = "V: MsgTrait")]
    pub value:V
}


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
pub struct MTMap<K:MsgTrait + 'static, V:MsgTrait + 'static> {
    #[serde(bound = "K:MsgTrait, V: MsgTrait")]
    pub zzz_array: Vec<KeyValue<K, V>>
}

impl <K:MsgTrait + 'static, V:MsgTrait + 'static> MsgTrait for MTMap<K, V> {

}

impl <K:MsgTrait + 'static, V:MsgTrait + 'static> MTMap<K, V> {
    pub fn new(map:HashMap<K, V>) -> Self {
        let mut vec = vec![];
        for (k, v) in map {
            let kv = KeyValue {
                key: k,
                value: v,
            };
            vec.push(kv);
        }
        Self {
            zzz_array:vec,
        }
    }

    pub fn to_map(&self) -> HashMap<K, V> {
        let mut map = HashMap::new();
        for kv in self.zzz_array.iter() {
            let opt = map.insert(kv.key.clone(), kv.value.clone());
            if opt.is_some() {
                panic!("existing key {:?}", kv.key);
            }
        }
        map
    }
}

pub fn mt_map_from_value(kv:Vec<(Value, Value)>) -> Res<Value> {
    let mut set = HashSet::new();
    for (k, _) in kv.iter() {
        let ok = set.insert(SJValueRef::from(k));
        if !ok {
            return Err(ET::ExistingSuchKey);
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
    map.insert(STR_MAP.to_string(), Value::Array(vec));
    Ok(Value::Object(map))
}