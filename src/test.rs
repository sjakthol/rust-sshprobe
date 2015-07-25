use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

/// Binds a TcpListener, TcpStream and connects those to together. The
/// listener writes the given data to the socket. The method returns a handle
/// to the client stream.
pub fn create_tcp_stream(data: &[u8]) -> TcpStream {
  let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
  let addr = listener.local_addr().unwrap();
  let client_streamer = TcpStream::connect(addr).unwrap();
  let (mut server_streamer, _) = listener.accept().unwrap();
  let _ = server_streamer.write_all(data);

  client_streamer
}
