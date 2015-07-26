extern crate sshprobe;

use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use sshprobe::{read_identifier, read_kex_payload};
use sshprobe::socket::get_peer_addr;

fn print_algolist(l: Vec<String>) {
  for a in l.iter() {
    println!("  {}", a)
  }
}

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

  let kex_payload = read_kex_payload(&stream).unwrap();
  println!("Key Exchange Algorithms:");
  print_algolist(kex_payload.key_exchange);

  println!("Host Keys:");
  print_algolist(kex_payload.host_keys);

  println!("Encryption (client -> server):");
  print_algolist(kex_payload.encryption_cts);

  println!("Encryption (server -> client):");
  print_algolist(kex_payload.encryption_stc);

  println!("MAC (client -> server):");
  print_algolist(kex_payload.mac_cts);

  println!("MAC (server -> client):");
  print_algolist(kex_payload.mac_stc);

  println!("Compression (client -> server):");
  print_algolist(kex_payload.compression_cts);

  println!("Compression (server -> client):");
  print_algolist(kex_payload.compression_stc);
}
