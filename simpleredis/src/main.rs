use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use simpleredis::database::Databse;


fn main() {

    let mut db = Databse::new();
    for id in 0..100 {
        let keyName = format!("hello{}", id);
        println!("setKey: {}", keyName);
        db.set(keyName, format!("value={}", id));
    }


    let key  = "hello89".to_string();
    match db.get(&key) {
        None => println!("not found"),
        Some(value) => {
            println!("value: {}", value);
        }
    }

    
    // if let Some(value) = db.get(key) {
    //     println!("value: {}", value);
    // }
    // let value = db.get("hello".to_string());
    // println!("valueis: {}", value);

}
// #[tokio::main]
// async fn main() {
//     let listenner = TcpListener::bind("127.0.0.1:6379").await.unwrap();
//     loop {
//         let (socket, _) = listenner.accept().await.unwrap();
//         process(socket).await;
//     }
// }


async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }

}
