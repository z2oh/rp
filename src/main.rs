use std::env;

use std::io;
use std::io::prelude::*;

use std::str;

extern crate curl;
use curl::easy::Easy;

fn main() {
	// We skip the first argument as it is just the name of the executable.
	for arg in env::args().skip(1) {
		match arg {
			_ if is_key_valid(&*arg) => {
				let res = try_read_paste(&*arg);
				match res {
					Ok(v) => println!("{}", v),
					Err(e) => println!("GET failed with error: {}", e),
				}
				return;
			}
			_ => { println!("Invalid argument! Usage: `rp <key>` where <key> is a hastebin key matching ([a-z]{{10}})"); return; }
		}
	}
	// Reading input to make a new paste.
	let buffer = read_input().unwrap();
	let res2 = try_post_paste(&*buffer);
	let mut key = res2.unwrap().split_off(8);
	key.split_off(10);
	println!("{}", key);
}

// Returns true if the key is 10 lowercase characters long.
fn is_key_valid(arg:&str) -> bool {
	for c in arg.chars() {
		if !char::is_lowercase(c) {
			return false;
		}
	}
	return arg.len() == 10;
}

// Attempts to read a paste on hastebin. If successful, will return a result
// with the paste as a String.
fn try_read_paste(arg:&str) -> Result<String, curl::Error> {
	let mut handle = Easy::new();
	let mut response = Vec::new();
	let url = "https://hastebin.com/raw/".to_string() + arg;
	handle.url(url.as_str()).unwrap();
	{
		let mut handle = handle.transfer();
		handle.write_function(|data| {
			response.extend_from_slice(data);
			Ok(data.len())
		}).unwrap();
		handle.perform()?;
	}
	Ok(String::from_utf8(response).expect("Paste is not valid utf8."))
}

// Attempts to post a string to hastebin. If successful, will return a result
// with the key as a String.
fn try_post_paste(arg:&str) -> Result<String, curl::Error> {
	let mut handle = Easy::new();
	let mut response = Vec::new();
	let url = "https://hastebin.com/documents/";
	handle.url(url)?;
	handle.post(true)?;
	handle.post_fields_copy(arg.as_bytes())?;
	{
		let mut handle = handle.transfer();
		handle.write_function(|data| {
			response.extend_from_slice(data);
			Ok(data.len())
		})?;
		handle.perform()?;
	}
	Ok(String::from_utf8(response).expect("Paste is not valid utf8."))
}

// Reads input from stdin until EOF.
fn read_input() -> io::Result<String> {
	let stdin = io::stdin();
	let mut stdin = stdin.lock();
	let mut buffer = String::new();
	stdin.read_to_string(&mut buffer)?;
	Ok(buffer)
}
