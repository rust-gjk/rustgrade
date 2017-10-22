use toml;

use std::fs::File;
use std::process::exit;
use std::io::{Read, Write};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Index {
    pub users: Vec<User>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub name: String,
    pub points: i32,
    pub log: Vec<LogEntry>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub amount: i32,
    pub reason: String,
    pub timestamp: String,
}

impl Index
{
    pub fn read() -> Result<Index, toml::de::Error> {
        let mut contents = String::new();
        let mut me = match File::open("index") {
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                let _ = File::create("index");
                return Ok(Index {
                    users: Vec::new()
                });
            }
        };

        if me.read_to_string(&mut contents).is_err() {
            println!("error: failed to read index");
            exit(-1);
        }

        toml::from_str(contents.as_ref())
    }

    pub fn write(&self) {
        let mut index_f = match File::create("index") {
            Ok(f) => f,
            Err(e) => {
                println!("error: couldn't open index for writing: {}", e);
                exit(-1)
            }
        };

        if let Err(e) = write!(index_f, "{}",
                toml::to_string(&self).unwrap()
                ) {
            println!("error: failed to write to index: {}", e);
            exit(-1)
        }
    }
}
