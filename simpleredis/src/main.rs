
use std::collections::{HashMap};

fn main() {
    
    let mut entries = HashMap::new();
    entries.insert("test", "adbcde");
    entries.insert("ad", "adbcde");

    for (book, review) in &entries {
        println!("{}: \"{}\"", book, review);
    }
}
