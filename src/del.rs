use AUTH_TOKEN;
use rustgrade::packets::Packet;

use std::process::exit;

pub fn del(username: &str) {
	if let Packet::Error{msg} = Packet::del(AUTH_TOKEN, username).send() {
		println!("error: {}", msg);
		exit(-1);
	}
}
