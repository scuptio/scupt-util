#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};
    use crate::message::MsgTrait;
    use crate::mt_set::MTSet;

    #[test]
    fn test_mt_set() {
        let mut set = HashSet::new();
        for i in 0..5 {
            set.insert(i);
        }
        let mt_set = MTSet::new(set.clone());
        let set1 = mt_set.to_set();
        assert_eq!(set, set1);
    }

    #[derive(
        Clone,
        Hash,
        PartialEq,
        Eq,
        Debug,
        Serialize,
        Deserialize,
        Decode,
        Encode,
    )]
    struct V {
        x : i32,
    }

    impl MsgTrait for V {

    }
    #[test]
    fn test_mt_set_hash() {
        let mut vec = vec![];
        for i in 0..20 {
            let v = V {
                x:i
            };
            vec.push(v);
        }
        let set1 = MTSet::from_vec(vec.clone());
        let set2 = MTSet::from_vec(vec.clone());
        assert_eq!(set1, set2);
        let mut hash_set = HashSet::new();
        hash_set.insert(set1);
        assert!(hash_set.contains(&set2));
    }
}