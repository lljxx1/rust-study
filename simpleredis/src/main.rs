use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use mini_redis::cmd::Command;
use simpleredis::database::Databse;
use simpleredis::parse::Parse;

use std::{fmt, str, vec};
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let mut db = Databse::new();
    let listenner = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, _) = listenner.accept().await.unwrap();
        process(socket, &db).await;
    }
}


fn handle_frame(frame: Frame) {
    let mut parse = match Parse::new(frame) {
        Ok(p) => p,
        Err(er) => panic!("get comand name failed"),
    };

    let comand_name = parse.next_string();
    println!("GOT: {:?}", comand_name);

    // Ok(())
}



async fn process(socket: TcpStream, db: &Databse) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        // println!("GOT: {:?}", frame);
        // let cmdName = cmd.get_name();
       
        handle_frame(frame);
        // match Command::from_frame(frame) {
        //     Err(er) => println!("get comand name failed"),
        //     Ok(cmd) => {
        //         match cmd {
        //             Command::Publish(_) => {},
        //             Command::Set(_) => {},
        //             Command::Subscribe(_) => {},
        //             Command::Unsubscribe(_) => {},
        //             Command::Unknown(cmd) => {},
        //             Command::Get(_)=> {
        //                 let key = _.key();
        //                 println!("get comand {}", key);
        //             }
        //         }
        //     }
        // }

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }

}