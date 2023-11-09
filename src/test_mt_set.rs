#[cfg(test)]
mod tests {
    use std::collections::HashSet;
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
}