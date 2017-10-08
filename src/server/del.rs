use rustgrade::packets::Packet;
use rustgrade::database::Index;

use std::process::exit;
use std::net::ToSocketAddrs;

use AUTH_TOKEN;
use SOCKET;

pub fn del<A: ToSocketAddrs + Clone>(packet: Packet, addr: A) {
	let mut index = match Index::read() {
		Ok(i) => i,
		Err(_) => {println!("fatal error"); exit(-1)} 
	};

	if let Packet::Del { hash, username } = packet {
		if hash != AUTH_TOKEN {
			Packet::error("invalid hash")
				.ssend(&addr, &SOCKET);
			return;
		}

		index.users.retain(|x| x.username != username);

		Packet::Ok.ssend(&addr, &SOCKET);
	}
}
