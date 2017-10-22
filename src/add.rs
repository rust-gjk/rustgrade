use AUTH_TOKEN;
use rustgrade::packets::Packet;

use std::process::exit;

pub fn add(name: &str, points: i32, reason: Option<String>) {
    if let Packet::Error{msg} = Packet::add(AUTH_TOKEN, name, reason, points).send() {
        println!("error: {}", msg);
        exit(-1)
    }
}
