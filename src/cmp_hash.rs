#![allow(deprecated)]
use std::cmp::Ordering;
use std::hash::{Hash, Hasher, SipHasher};

pub fn cmp_hash<K:Hash + Eq>(x:&K, y:&K) -> Ordering{
    let mut ord = Ordering::Equal;
    let mut n = 0u64;
    while ord.is_eq() {
        let mut h1 = SipHasher::new_with_keys(0, n);
        let mut h2 = SipHasher::new_with_keys(0, n);
        x.hash(&mut h1);
        y.hash(&mut h2);

        let v1 = h1.finish();
        let v2 = h2.finish();
        ord = v1.cmp(&v2);
        n += 1;
    }
    ord
}