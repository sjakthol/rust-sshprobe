extern crate sshprobe;

use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use sshprobe::read_identifier;
use sshprobe::socket::get_peer_addr;

fn main() {
  let mut args = env::args();

  if args.len() != 2 {
    return println!("Usage: ssh2-prober hostname[:port]");
  }

  // Resolve the peer address to connect to.
  let addr = args.nth(1).unwrap();
  println!("[Local] Resolving address '{}'", addr);

  // Use &addr[..] since .as_str() is unstable.
  let peer_addr = get_peer_addr(&addr[..]).unwrap();

  println!("[Local] Connecting to '{}'", peer_addr);
  let mut stream = TcpStream::connect(peer_addr).unwrap();

  println!("[Local] Connection established.");

  // Send the intro header.
  let _ = write!(stream, "SSH-2.0-SSHProbe_0.1\r\n");

  let res = read_identifier(&stream).unwrap();
  println!("[Remote] Version: {}", res);
}
