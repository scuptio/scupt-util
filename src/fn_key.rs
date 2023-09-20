use std::marker::PhantomData;

use crate::fn_compare::{FnCompare, FnEqual};
use crate::fn_hash::FnHash;
use crate::slice::Slice;

pub struct FnKey<
    KEY: Slice + 'static,
    EQ: FnEqual<KEY> + 'static,
    CMP: FnCompare<KEY> + 'static,
    HSH: FnHash<KEY> + 'static
> {
    _phantom: PhantomData<KEY>,
    fn_equal: EQ,
    fn_compare: CMP,
    fn_hash: HSH,
}

impl<
    KEY: Slice + 'static,
    EQ: FnEqual<KEY> + 'static,
    CMP: FnCompare<KEY> + 'static,
    HSH: FnHash<KEY> + 'static
>
FnKey<KEY, EQ, CMP, HSH> {
    pub fn new(is_equal: EQ, compare: CMP, hash: HSH) -> Self {
        Self {
            _phantom: Default::default(),
            fn_equal: is_equal,
            fn_compare: compare,
            fn_hash: hash,
        }
    }

    pub fn func_hash(&self) -> &HSH {
        &self.fn_hash
    }

    pub fn func_equal(&self) -> &EQ {
        &self.fn_equal
    }

    pub fn func_compare(&self) -> &CMP {
        &self.fn_compare
    }
}