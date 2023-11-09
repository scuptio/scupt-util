#[cfg(test)]
mod tests {
    use std::collections::HashMap;
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
}