use serde_cbor;

use fail;
use fail1;

use std::process::exit;
use std::net::UdpSocket;
use std::net::ToSocketAddrs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Packet {
	Add { hash: String, username: String, reason: Option<String>, amount: i32 },
	Rem { hash: String, username: String, reason: Option<String>, amount: i32 },
	New { hash: String, username: String, name: String, amount: i32 },
	Del { hash: String, username: String },

	Error { msg: String },
	Ok
}


impl Packet {
	pub fn read(source: &[u8]) -> Result<Packet, serde_cbor::error::Error> {
		serde_cbor::de::from_slice(source)
	}

	pub fn make(self) -> Result<Vec<u8>, serde_cbor::Error> {
		serde_cbor::ser::to_vec(&self)
	}

	pub fn send(self) -> Packet {
		let sock = match UdpSocket::bind("0.0.0.0:0") {
			Ok(s) => s,
			Err(_) => fail("failed to bind to socket", 2)
		};

		if sock.connect("magnusi.tech:9008").is_err() {
			fail("failed to connect to remote host. are you connected to the internet?", 3);
		}

		let bytes = match self.clone().make() {
			Ok(b) => b,
			Err(_) => fail1("failed to serialize packet", format!("{:?}", self), 4)
		};

		loop {
			if let Err(e) = sock.send(&bytes)
				{fail1("failed to send data: {}", e, 5);}

			let mut res_buf = [0; 64 * 1024]; // maximum response size is 60kb

			let res_size = match sock.recv(&mut res_buf) {
				Ok(s) => s,
				Err(_) => continue,
			};
			let res_buf = &mut res_buf[..res_size];

			let res = match Packet::read(&res_buf.to_vec()) {
				Ok(p) => p,
				Err(_) => fail("failed to deserialize packet", 6)
			};

			return res;
		}
	}

	pub fn ssend<A: ToSocketAddrs>(self, addr: A, socket: &UdpSocket) {
		let bytes = match self.clone().make() {
			Ok(b) => b,
			Err(_) => {
				println!("  error: failed to serialize packet. {:?}", self);
				exit(-1);
			}
		};

		if let Err(e) = socket.send_to(&bytes, addr) {
			println!("{}", e);
		}
	}

	pub fn new(hash: &str, username: &str, name: &str, amount: i32) -> Self {
		Packet::New {
			hash: hash.to_string(),
			username: username.to_string(),
			name: name.to_string(),
			amount: amount
		}
	}

	pub fn del(hash: &str, username: &str) -> Self {
		Packet::Del {
			hash: hash.to_string(),
			username: username.to_string()
		}
	}

	pub fn add(hash: &str, username: &str, reason: Option<String>, amount: i32) -> Self {
		Packet::Add {
			hash: hash.to_string(),
			username: username.to_string(),
			reason: reason,
			amount: amount
		}
	}

	pub fn rem(hash: &str, username: &str, reason: Option<String>, amount: i32) -> Self {
		Packet::Rem {
			hash: hash.to_string(),
			username: username.to_string(),
			reason: reason,
			amount: amount
		}
	}

	pub fn error(msg: &str) -> Self {
		Packet::Error {
			msg: msg.to_string()
		}
	}
}
