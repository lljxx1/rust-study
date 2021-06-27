use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use mini_redis::cmd::Command;
use simpleredis::database::Databse;



#[tokio::main]
async fn main() {
    let mut db = Databse::new();
    let listenner = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, _) = listenner.accept().await.unwrap();
        process(socket, &db).await;
    }
}


async fn process(socket: TcpStream, db: &Databse) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        // println!("GOT: {:?}", frame);

        // let cmdName = cmd.get_name();
        match Command::from_frame(frame) {
            Err(er) => println!("get comand name failed"),
            Ok(cmd) => {
                match cmd {
                    Command::Publish(_) => {},
                    Command::Set(_) => {},
                    Command::Subscribe(_) => {},
                    Command::Unsubscribe(_) => {},
                    Command::Unknown(cmd) => {},
                    Command::Get(_)=> {
                        let key = _.key();
                        println!("get comand {}", key);
                    }
                }
            }
        }

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }

}
