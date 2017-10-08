use rustgrade::packets::Packet;
use rustgrade::database::{Index, LogEntry};
use rustgrade::get_time_stamp;

use std::process::exit;
use std::net::ToSocketAddrs;

use AUTH_TOKEN;
use SOCKET;

pub fn add<A: ToSocketAddrs + Clone>(packet: Packet, addr: A) {
	let mut index = match Index::read() {
		Ok(i) => i,
		Err(e) => {println!("fatal error: {}", e); exit(-1)} 
	};

	if let Packet::Add { hash, username, reason, amount } = packet {
		if hash != AUTH_TOKEN {
			Packet::error("invalid hash")
				.ssend(&addr, &SOCKET);
			return;
		}

		match index.users.iter_mut().filter(|x| x.username == username).next() {
			Some(u) => {
				u.log.push(LogEntry {
					amount: amount,
					reason: if let Some(s) = reason {s.clone()} else {"none".to_string()},
					timestamp: get_time_stamp().unwrap()
				});

				u.points += amount;
			},
			None => { Packet::error("user not found").ssend(&addr, &SOCKET); return; }
		}

		index.write();

		Packet::Ok.ssend(&addr, &SOCKET);
	}
}
