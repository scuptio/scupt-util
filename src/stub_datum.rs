use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use bincode::Decode;
use bincode::Encode;
use serde::Deserialize;
use serde::Serialize;

use crate::datum::Datum;
use crate::datum_msg::DatumMsg;
use crate::fn_compare::FnCompare;
use crate::fn_compare::FnEqual;
use crate::fn_hash::FnHash;
use crate::message::MsgTrait;
use crate::res::Res;
use crate::slice::Slice;

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
pub struct __Integer {
    value: i64,
}

#[derive(Clone)]
pub struct __IntCompare {}

#[derive(Clone)]
pub struct __IntHash {}

#[derive(Clone)]
pub struct __IntEqual {}

impl Slice for __Integer {
    fn as_slice(&self) -> &[u8] {
        unsafe {
            std::mem::transmute::<&i64, &[u8; 8]>(&self.value)
        }
    }
}

impl __Integer {
    pub fn new(value: i64) -> Self {
        Self {
            value,
        }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

impl Datum for __Integer {}

impl MsgTrait for __Integer {}

impl DatumMsg for __Integer {}

impl __IntCompare {
    pub fn new() -> Self {
        Self {}
    }
}

impl __IntHash {
    pub fn new() -> Self {
        Self {}
    }
}

impl __IntEqual {
    pub fn new() -> Self {
        Self {}
    }
}


impl Default for __IntCompare {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for __IntHash {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for __IntEqual {
    fn default() -> Self {
        Self::new()
    }
}

impl FnCompare<__Integer> for __IntCompare {
    fn compare(&self, k1: &__Integer, k2: &__Integer) -> Res<Ordering> {
        Ok(k1.value.cmp(&k2.value))
    }
}

impl FnHash<__Integer> for __IntHash {
    fn hash<H: Hasher>(&self, key: &__Integer, state: &mut H) -> Res<()> {
        key.value.hash(state);
        Ok(())
    }
}

impl FnEqual<__Integer> for __IntEqual {
    fn equal(&self, k1: &__Integer, k2: &__Integer) -> Res<bool> {
        Ok(k1.value.eq(&k2.value))
    }
}