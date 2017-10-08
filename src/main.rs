extern crate serde_cbor;
extern crate toml;
extern crate clap;

use clap::{App, SubCommand, AppSettings};

mod add;
mod rem;
mod new;
mod del;

extern crate rustgrade;

pub const AUTH_TOKEN: &'static str = include!("auth_token");

fn main() {
	let matches = App::new("rustgrade")
		.version("0.1.0")
		.author("Lukáš Hozda [magnusi] <luk.hozda@gmail.com>")
		.about("manage scores")
		.global_settings(&[AppSettings::ColoredHelp])
		.subcommand(SubCommand::with_name("add")
			.alias("a")
			.about("add points to a student")
			.args_from_usage("<NAME>	'name of the student'
							  <AMOUNT>	'amount of points'
							  [REASON]	'reason for addition'"))
		.subcommand(SubCommand::with_name("rem")
			.alias("r")
			.about("remove points from a student")
			.args_from_usage("<NAME>	'name of the student'
							  <AMOUNT>	'amount of points'
							  [REASON]	'reason for addition'"))
		.subcommand(SubCommand::with_name("new")
			.alias("n")
			.about("add a new student")
			.args_from_usage("<USERNAME>	'username of the student'
							  <NAME>		'name of the student'
							  [POINTS]		'starting amount of points'"))
		.subcommand(SubCommand::with_name("del")
			.alias("d")
			.about("delete a student")
			.args_from_usage("<USERNAME>	'username of the student'"))
		.get_matches();

	match matches.subcommand() {
		("add", Some(m)) => {
			let name = m.value_of("NAME").unwrap();
			let amount: i32 = m.value_of("AMOUNT").unwrap().parse().unwrap();
			let reason = match m.value_of("REASON") {
				Some(p) => Some(p.to_string()),
				None => None,
			};

			add::add(name, amount, reason)
		},
		("rem", Some(m)) => {
			let name = m.value_of("NAME").unwrap();
			let amount: i32 = m.value_of("AMOUNT").unwrap().parse().unwrap();
			let reason = match m.value_of("REASON") {
				Some(p) => Some(p.to_string()),
				None => None,
			};

			rem::rem(name, amount, reason)
		},
		("new", Some(m)) => {
			let username = m.value_of("USERNAME").unwrap();
			let name = m.value_of("NAME").unwrap();
			let points: Option<i32> = match m.value_of("POINTS") {
				Some(p) => Some(p.parse().unwrap()),
				None => None,	
			};

			new::new(username, name, points)
		},
		("del", Some(m)) => {
			let username = m.value_of("USERNAME").unwrap();

			del::del(username)
		},
		_ => ()
	}
}
