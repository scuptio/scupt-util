

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};
    use crate::message::MsgTrait;
    use crate::mt_map::MTMap;


    #[test]
    fn test_mt_map() {
        let mut map = HashMap::new();
        for i in 0..5 {
            map.insert(i, i);
        }
        let mt_map = MTMap::new(map.clone());
        let map1 = mt_map.to_map();
        assert_eq!(map, map1);
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
    struct K {
        x:i32,
    }

    impl MsgTrait for K {

    }
    #[test]
    fn test_mt_map_hash() {
        let mut vec = vec![];
        for i in 0..20 {
            let k = K {
                x:i
            };
            let v = K {
                x:i + 1
            };
            vec.push((k, v));
        }
        let map1 = MTMap::from_vec(vec.clone());
        let map2 =  MTMap::from_vec(vec.clone());
        assert_eq!(map1, map2);
        let mut hash_set = HashSet::new();
        hash_set.insert(map1);
        assert!(hash_set.contains(&map2));
    }
}