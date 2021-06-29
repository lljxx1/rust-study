pub mod database;

#[cfg(test)]
mod tests {
    use super::*;
    use database::Databse;

    #[test]
    fn test_set() {
        let mut db = Databse::new();
        for id in 0..1 {
            let keyName = format!("hello{}", id);
            println!("setKey: {}", keyName);
            db.set(keyName, format!("value={}", id));
        }

        let key  = "hello1".to_string();
        assert_eq!(
            "value=1".to_string(),
            db.get(&key).unwrap()
        );

    }
}