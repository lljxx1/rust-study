


use mini_redis::{Connection, Db, Frame, Parse};
use tracing::{debug, instrument};
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


    pub async fn apply(self, dst: &mut Connection) -> Result<()> {
        let reponse = Frame::Error(format!("Err unknow command '{}'", self.comand_name));

        debug!(?reponse);

        dst.write_frame(&reponse).await?;
        Ok({})
    }
}







pub enum Command {
    Get(Get),
    Unknown(Unknown)
}

impl Command {

    pub fn new(frame: Frame) -> Result<Command> {
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
        }

        parse.finish()?;

        Ok(comand)
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

    pub fn parse_frames(parse: &mut Parse) -> Result<Get> {
        let key = parse.next_string()?;
        Ok(Get { key })
    }


}