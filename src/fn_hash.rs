use std::hash::Hasher;

use crate::res::Res;

pub trait FnHash<K: 'static>: Clone + Send + Sync {
    fn hash<H: Hasher>(&self, key: &K, state: &mut H) -> Res<()>;
}