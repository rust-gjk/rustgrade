extern crate rustgrade;
extern crate serde_cbor;
#[macro_use]
extern crate lazy_static;

use rustgrade::packets::Packet;

use std::net::UdpSocket;
use std::process::exit;
use std::thread;

mod add;
mod del;
mod new;
mod rem;

lazy_static! {
    pub static ref SOCKET: UdpSocket = match UdpSocket::bind("0.0.0.0:9008") {
        Ok(s) => s,
        Err(_) => {
            println!(" error: failed to bind to socket");
            exit(-1);
        }
    };
}

pub const AUTH_TOKEN: &'static str = include!("../auth_token");

fn main() {
    loop {
        let mut res = [0; 2 * 1024 * 1024];
        let (amt, src) = match SOCKET.recv_from(&mut res) {
            Ok((a,s)) => (a,s),
            Err(_) => {
                println!("  error: failed to receive packet");
                exit(-1);
            }
        };

        let res = &mut res[..amt];
        let packet: Packet = match serde_cbor::de::from_slice(res) {
            Ok(p) => p,
            Err(_) => {
                println!("  error: failed to deserialize packet");
                exit(-1);
            }
        };

        println!("{:?} from {}", &packet, &src);
        thread::spawn( move || {
                match packet {
                    Packet::New {..} => new::new(packet, src),
                    Packet::Del {..} => del::del(packet, src),
                    Packet::Add {..} => add::add(packet, src),
                    Packet::Rem {..} => rem::rem(packet, src),
                    
                    // these are not to be sent and so they shan't be received
                    Packet::Error {..} => (),
                    Packet::Ok => ()
                }
            }
        );
    }
}
