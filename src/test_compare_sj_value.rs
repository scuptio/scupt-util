#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};
    use serde_json::{Number, Value};

    use crate::message::MsgTrait;
    use crate::mt_map::{KeyValue, mt_map_from_value, MTMap};
    use crate::mt_set::{mt_set_from_value, MTSet};
    use crate::sj_value_ref::SJValueRef;

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
    struct TestM {
        set:MTSet<i32>,
        map:MTMap<i32, i32>,
    }

    impl MsgTrait for TestM {

    }

    fn gen_set(vec:Vec<i32>) -> TestM {
        let vec_value = gen_values(vec.clone());
        let value = mt_set_from_value(vec_value).unwrap();

        let m1 = TestM {
            set: MTSet{
                zzz_array:vec
            },
            map:Default::default(),
        };
        let m2 = TestM {
            set: serde_json::from_value(value).unwrap(),
            map:Default::default(),
        };

        assert_eq!(m1, m2);
        m1
    }

    fn gen_map(vec:Vec<(i32, i32)>) -> TestM {
        let mut kv = vec![];
        let vec_value_pairs = gen_value_pairs(vec.clone());
        let value = mt_map_from_value(vec_value_pairs).unwrap();
        for (k, v) in vec {
            kv.push(KeyValue{key:k, value:v})
        }
        let m1 = TestM {
            set: Default::default(),
            map:MTMap {
                zzz_array: kv,
            }
        };

        let m2 = TestM {
            set: Default::default(),
            map: serde_json::from_value(value).unwrap()
        };
        assert_eq!(m1, m2);
        m1
    }

    fn gen_values(vec:Vec<i32>) -> Vec<Value> {
        let mut v = vec![];
        for e in vec {
            v.push(Value::Number(Number::from(e)));
        }
        v
    }

    fn gen_value_pairs(vec:Vec<(i32, i32)>) -> Vec<(Value, Value)> {
        let mut v = vec![];
        for (_k, _v) in vec {
            v.push((Value::Number(Number::from(_k)), Value::Number(Number::from(_v))));
        }
        v
    }

    #[test]
    fn test_compare_sj_value_mt_set() {
        let m1 = gen_set(vec![1, 2]);

        let m2 = gen_set(vec![2, 1]);
        let m3 = gen_set(vec![1, 3]);
        let m4 = gen_set(vec![1]);
        let v1 = serde_json::to_value(m1).unwrap();
        let v2 = serde_json::to_value(m2).unwrap();
        let v3 = serde_json::to_value(m3).unwrap();
        let v4 = serde_json::to_value(m4).unwrap();

        let ord = SJValueRef::from(&v1).cmp(&SJValueRef::from(&v2));
        assert_eq!(ord, Ordering::Equal);

        let ord = SJValueRef::from(&v1).cmp(&SJValueRef::from(&v3));
        assert_ne!(ord, Ordering::Equal);

        let ord = SJValueRef::from(&v1).cmp(&SJValueRef::from(&v4));
        assert_ne!(ord, Ordering::Equal);


    }
    #[test]
    fn test_compare_sj_value_mt_map() {
        let m1 = gen_map(vec![(1, 2), (2, 3)]);
        let m2 = gen_map(vec![(2, 3), (1, 2)]);
        let m3 = gen_map(vec![(2, 3), (4, 5)]);
        let m4 = gen_map(vec![(1, 2)]);
        let v1 = serde_json::to_value(m1).unwrap();
        let v2 = serde_json::to_value(m2).unwrap();
        let v3 = serde_json::to_value(m3).unwrap();
        let v4 = serde_json::to_value(m4).unwrap();

        let ord = SJValueRef::from(&v1).cmp(&SJValueRef::from(&v2));
        assert_eq!(ord, Ordering::Equal);

        let ord = SJValueRef::from(&v1).cmp(&SJValueRef::from(&v3));
        assert_ne!(ord, Ordering::Equal);

        let ord = SJValueRef::from(&v1).cmp(&SJValueRef::from(&v4));
        assert_ne!(ord, Ordering::Equal);


    }


}