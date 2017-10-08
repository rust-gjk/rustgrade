use rustgrade::packets::Packet;
use rustgrade::database::{Index, User};

use std::process::exit;
use std::net::ToSocketAddrs;

use AUTH_TOKEN;
use SOCKET;

pub fn new<A: ToSocketAddrs + Clone>(packet: Packet, addr: A) {
	let mut index = match Index::read() {
		Ok(i) => i,
		Err(e) => {println!("fatal error: {}", e); exit(-1)} 
	};

	if let Packet::New { hash, username, name, amount } = packet {
		if hash != AUTH_TOKEN {
			Packet::error("invalid hash")
				.ssend(&addr, &SOCKET);
			return;
		}

		index.users.push(
			User {
				username: username.clone(),
				name: name.clone(),
				points: amount,
				log: Vec::new()
			}
		);

		Packet::Ok.ssend(&addr, &SOCKET);
	}
}
