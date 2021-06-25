
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



fn test_file() {
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
}


fn largest_number(list: &[i32]) -> i32 {
    let mut target = list[0];
    for &item in list {
        if item > target {
            target = item;
        } 
    }

    target
}


fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut larg = list[0];

    for &item in list.iter() {
        if item > larg {
            larg = item;
        }
    }

    larg
}

fn test_sort() {
    // let nums = vec![34, 25, 100, 16, 250];
    // let result = largest(&nums);
    // println!("max is {}", result);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['a', 'b', 'y', 'q'];
    // let result = largest(&char_list);
    // println!("largets char is {}", result);
}


pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

fn main() {

    // test_sort();

    let article = NewsArticle {
        headline: String::from("test"),
        author: String::from("fun"),
    };

    println!("article: {}", article.summarize());
    // let coin = Coin(Coin::Penny);
    // let centes = value_in_centes(Coin::Quarter(UsState::China));
    // println!("{}", centes);
  
   
    // let mut entries = HashMap::new();
    // entries.insert("test", "adbcde");
    // entries.insert("ad", "adbcde");
    // for (book, review) in &entries {
    //     println!("{}: \"{}\"", book, review);
    // }

    let r;
    let x = 5;
    r = &x;


    println!("r: {}", r);
}
