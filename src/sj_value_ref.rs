use std::cmp::{min, Ordering};
use serde_json::Value;
use std::hash::{Hash, Hasher};
use crate::{mt_map, mt_set};


/// Serde Json Value Reference
/// implement some utility functions of serde_json::Value
pub struct SJValueRef<'a> {
    value : &'a Value,
}

impl <'a> SJValueRef<'a> {
    pub fn from(value:&'a Value) -> Self {
        Self {
            value,
        }
    }
}


fn sj_value_id(value:&Value) -> i32 {
    match value {
        Value::Null => { 0 }
        Value::Bool(_) => { 1 }
        Value::Number(_) => { 2 }
        Value::String(_) => { 3 }
        Value::Array(_) => { 4 }
        Value::Object(_) => { 5 }
    }
}
impl <'a> PartialOrd<Self> for SJValueRef<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <'a> Ord for SJValueRef<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = match (self.value, other.value) {
            (Value::Null, Value::Null) => { Ordering::Equal }
            (Value::Bool(b1), Value::Bool(b2)) => {
                b1.cmp(b2)
            }
            (Value::Number(n1), Value::Number(n2)) => {
                n1.to_string().cmp(&n2.to_string())
            }
            (Value::String(s1), Value::String(s2)) => {
                s1.cmp(s2)
            }
            (Value::Array(v1), Value::Array(v2)) => {
                let len = min(v1.len(), v2.len());
                let mut opt_ord = None;
                for i in 0..len {
                    let o = Self::from(&v1[i]).cmp(&Self::from(&v2[i]));
                    if o != Ordering::Equal {
                        opt_ord = Some(o);
                        break;
                    }
                }
                let ord = match opt_ord {
                    Some(o) => { o }
                    None => { v1.len().cmp(&v2.len()) }
                };
                ord
            }
            (Value::Object(s1), Value::Object(s2)) => {
                let mut vec1 = vec![];
                let mut vec2 = vec![];
                for (k, v) in s1.iter() {
                    vec1.push((k, v))
                }
                for (k, v) in s2.iter() {
                    vec2.push((k, v))
                }

                for vec in [&mut vec1, &mut vec2] {
                    vec.sort_by(|(k1, v1), (k2, v2)| {
                        let ord = k1.cmp(k2);
                        let ord = if ord == Ordering::Equal {
                            Self::from(v1).cmp(&Self::from(v2))
                        } else {
                            ord
                        };
                        ord
                    });
                }

                let len = min(vec1.len(), vec2.len());
                let mut opt_ord = None;
                for i in 0..len {
                    let key_order = vec1[i].0.cmp(&vec2[i].0);
                    if key_order == Ordering::Equal {
                        let key = vec1[i].0;
                        let v1 = vec1[i].1;
                        let v2 = vec2[i].1;
                        let value_order = if need_normalized(key) {
                            compare_normalized(v1, v2)
                        } else {
                            Self::from(vec1[i].1).cmp(&Self::from(vec2[i].1))
                        };
                        if value_order != Ordering::Equal {
                            opt_ord = Some(value_order);
                            break;
                        }
                    } else {
                        opt_ord = Some(key_order);
                        break;
                    }
                }
                match opt_ord {
                    None => { vec1.len().cmp(&vec2.len()) }
                    Some(o) => {o}
                }
            }
            (v1, v2) => {
                let id1 = sj_value_id(v1);
                let id2 = sj_value_id(v2);
                id1.cmp(&id2)
            }
        };
        cmp
    }
}
impl <'a> PartialEq for SJValueRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl <'a> Eq for SJValueRef<'a> {

}

impl <'a> Hash for SJValueRef<'a> {
    fn hash<H: Hasher>(& self, state: &mut H) {
        match self.value {
            Value::Null => {
                2.hash(state);
            }
            Value::Bool(v) => {
                3.hash(state);
                v.hash(state);
            }
            Value::Number(n) => {
                5.hash(state);
                n.hash(state)
            }
            Value::String(s) => {
                7.hash(state);
                s.hash(state)
            }
            Value::Array(array) => {
                11.hash(state);
                for v in array.iter() {
                    let v_ref = SJValueRef::from(v);
                    v_ref.hash(state);
                }
            }
            Value::Object(map) => {
                13.hash(state);
                let mut vec = vec![];
                for (k, v) in map.iter() {
                    vec.push((k, v));
                }
                vec.sort_by(|x, y|
                    { x.0.cmp(y.0) }
                );
                for (k, v) in vec {
                    k.hash(state);
                    if need_normalized(k) {
                        hash_normalize(v, state)
                    } else {
                        let v_ref = SJValueRef::from(v);
                        v_ref.hash(state);
                    }
                }
            }
        }
    }
}

fn need_normalized(s:&String) -> bool {
    s.eq(mt_map::STR_MAP) || s.eq(mt_set::STR_SET)
}

fn compare_normalized(v1:&Value, v2:&Value) -> Ordering {
    if !v1.is_array() || !v2.is_array() {
        panic!("must be a array");
    }
    let mut a1 = v1.as_array().unwrap().clone();
    let mut a2 = v2.as_array().unwrap().clone();
    array_normalize(&mut a1);
    array_normalize(&mut a2);
    let va1 = Value::Array(a1);
    let va2 = Value::Array(a2);
    let r1 = SJValueRef::from(&va1);
    let r2 = SJValueRef::from(&va2);
    r1.cmp(&r2)
}

fn array_normalize(v:&mut Vec<Value>) {
   v.sort_by(|x, y| {
       SJValueRef::from(x).cmp(&SJValueRef::from(y))
   })
}

fn hash_normalize<H: Hasher>(v:&Value, state: &mut H) {
    if !v.is_array()  {
        panic!("map member must be array");
    }
    let mut a1 = v.as_array().unwrap().clone();
    array_normalize(&mut a1);
    let va1 = Value::Array(a1);
    let r1 = SJValueRef::from(&va1);
    r1.hash(state);
}