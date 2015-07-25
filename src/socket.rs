use std;
use std::io::{Error, ErrorKind};
use std::net::{SocketAddr, ToSocketAddrs};

static SSH_DEFAULT_PORT: u16 = 22;

/// Resolves the peer address to connect to. If the hostname or IP provided by
/// the user does not contain a port, the default SSH port, 22, is used.
pub fn get_peer_addr(addr: &str) -> std::io::Result<SocketAddr> {
  // Resolve the address using default port if the given addr does not specify
  // one.
  let result = if addr.contains(":") {
    addr[..].to_socket_addrs()
  } else {
    (&addr[..], SSH_DEFAULT_PORT).to_socket_addrs()
  };

  // Process the result and take the first found address.
  try!(result).next().ok_or_else(|| {
    Error::new(ErrorKind::Other, "Name resolution did not yield any results.")
  })
}

#[cfg(test)]
mod tests {
  use super::get_peer_addr;

  #[test]
  fn test_get_peer_addr_with_port() {
    let addr = get_peer_addr("127.0.0.1:8080").unwrap();
    assert_eq!(addr.port(), 8080)
  }

  #[test]
  fn test_get_peer_addr_without_port() {
    let addr = get_peer_addr("127.0.0.1").unwrap();
    assert_eq!(addr.port(), 22)
  }

  #[test]
  fn test_get_peer_addr_invalid_input() {
    assert!(get_peer_addr("this.is.not.a.real.address").is_err());
  }
}
