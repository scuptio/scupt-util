#[cfg(test)]
mod tests {
    use crate::message::Message;

    #[test]
    fn test_message() {
        let m1 = Message::new(1000, 1, 2);
        let s = Message::<i32>::build_json_str(1000.to_string(), 1, 2).unwrap();
        let m2:Message<i32> = serde_json::from_str(s.as_str()).unwrap();
        assert!(m2.eq(&m1));
    }
}