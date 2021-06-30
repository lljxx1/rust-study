


use mini_redis::{Connection, Frame};
use crate::parse::Parse;
use crate::Error;
// use tracing::{debug, instrument};
use bytes::Bytes;

#[derive(Debug)]
pub struct Unknown {
    comand_name: String,
}

impl Unknown {

    pub fn new(key: impl ToString) -> Unknown {
        Unknown {
            comand_name: key.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.comand_name
    }

    pub async fn apply(self) -> Result<(), Error> {
        let reponse = Frame::Error(format!("Err unknow command '{}'", self.comand_name));
        // debug!(?reponse);
        // dst.write_frame(&reponse).await?;
        Ok({})
    }
}





pub struct Get {
    key: String
}


impl Get {

    pub fn new(key: impl ToString) -> Get {
        Get {
            key: key.to_string(),
        }
    }

    pub fn parse_frames(parse: &mut Parse) -> Result<Get, Error> {
        let key = parse.next_string()?;
        Ok(Get { key })
    }

    pub async fn apply(self) -> Result<(), Error> {
        println!("key is {}", self.key);
        Ok(())
    }

    // pub fn into_frame(self) -> Frame {
    //     let mut frame = Frame::array();
    //     frame.push_bulk(Bytes::from("get".as_bytes()));
    //     frame.push_bulk(Bytes::from(self.key.info_bytes()));
    //     frame
    // }

}


pub enum Command {
    Get(Get),
    Unknown(Unknown)
}

impl Command {

    pub fn from_frame(frame: Frame) -> Result<Command, Error> {
        let mut parse = match Parse::new(frame) {
            Ok(p) => p,
            Err(er) => panic!("get comand name failed"),
        };

        let comand_name = parse.next_string()?.to_lowercase();
        let comand = match  &comand_name[..] {
            "get" => Command::Get(Get::parse_frames(&mut parse)?),
            _ => {
                return Ok(Command::Unknown(Unknown::new(comand_name)));
            }
        };

        parse.finish()?;

        Ok(comand)
    }

    pub async fn apply(self) -> Result<(), Error> {
        use Command::*;
        println!("command apply");
        match self {
            Get(cmd) => cmd.apply().await,
            Unknown(cmd) => cmd.apply().await,
        }

    }
}

