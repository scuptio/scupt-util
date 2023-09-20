use std::cmp::Ordering;

use crate::res::Res;

pub trait FnEqual<S: 'static>: Clone + Send + Sync {
    fn equal(&self, k1: &S, k2: &S) -> Res<bool>;
}

pub trait FnCompare<S: 'static>: Clone + Send + Sync {
    fn compare(&self, k1: &S, k2: &S) -> Res<Ordering>;
}