
use std::collections::{ HashMap };
use std::fs::File;
use std::io::ErrorKind;


#[derive(Debug)]
enum UsState {
    China,
}

enum Coin {
    Penny,
    Dime,
    Quarter(UsState),
}

fn value_in_centes(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 2,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}", state);
            25
        } 
    }
}


fn main() {
    // let coin = Coin(Coin::Penny);
    // let centes = value_in_centes(Coin::Quarter(UsState::China));
    // println!("{}", centes);
    let fd = File::open("zset.rs");
    let fd = match fd {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Problfem when create file {:?}", err)
            }
            other_error => panic!("Open file failed {:?}", other_error)
        }
    };
    // let mut entries = HashMap::new();
    // entries.insert("test", "adbcde");
    // entries.insert("ad", "adbcde");
    // for (book, review) in &entries {
    //     println!("{}: \"{}\"", book, review);
    // }
}
