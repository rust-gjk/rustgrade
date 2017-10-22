extern crate time;
extern crate toml;
extern crate serde;
extern crate serde_cbor;
#[macro_use] extern crate serde_derive;

pub mod packets;
pub mod database;

use time::{strftime, now};

use std::fmt::Display;
use std::process::exit;

pub fn get_time_stamp() -> Result<String, String> {
    match strftime("%Y-%m-%d %H:%M:%S", &now()) {
        Ok(t) => Ok(format!("[{}]", t)),
        Err(e) => Err(format!("invalid timestamp format: {}", e))
    }
}


pub fn fail(msg: &str, code: i32) -> ! {
    eprintln!("  {} {}",
        "error",
        msg
    );
    exit(code);
}

pub fn fail1<T: Display>(msg: &str, arg: T, code: i32) -> ! {
    eprintln!("  {} {}",
        "error",
        msg.replace("{}", &arg.to_string())
    );
    exit(code);
}
