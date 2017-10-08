use AUTH_TOKEN;
use rustgrade::packets::Packet;

use std::process::exit;

pub fn new(username: &str, name: &str, points: Option<i32>) {
	let points = if let Some(p) = points {p} else {0};
	if let Packet::Error{msg} = Packet::new(AUTH_TOKEN, username, name, points).send() {
		println!("error: {}", msg);
		exit(-1)
	}
}
