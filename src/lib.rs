pub mod socket;
pub mod reader;
pub mod test;

use std::io::{Error, ErrorKind};
use std::net::TcpStream;
use reader::{read_byte, read_bytes, read_line, read_u32};

static SSH_IDENTIFIER_MAX_LINE_LEN: usize = 255;
static SSH_MAX_LINES_BEFORE_IDENTIFIER: usize = 5;
static SSH_MSG_KEXINIT: u8 = 20;

/// Reads the SSH server identifier from the stream. Skips at most 5 lines
/// before giving up. RFC 4253, Section 4.2.
pub fn read_identifier(stream: &TcpStream) -> std::io::Result<String> {
  let mut line = try!(read_line(stream, SSH_IDENTIFIER_MAX_LINE_LEN));
  let mut lines = 0;
  while !line.starts_with("SSH-") {
    line = try!(read_line(stream, SSH_IDENTIFIER_MAX_LINE_LEN));
    lines += 1;

    if lines == SSH_MAX_LINES_BEFORE_IDENTIFIER {
      return Err(Error::new(ErrorKind::Other,
                            "Too many lines before the server identifier."));
    }
  }

  Ok(line)
}

/// Reads an SSH name-list from the stream.
fn read_name_list(stream: &TcpStream) -> std::io::Result<Vec<String>> {
  let len = try!(read_u32(stream));
  let raw = try!(read_bytes(stream, len as usize));
  let s = try!(String::from_utf8(raw)
    .map_err(|_| Error::new(ErrorKind::Other, "Invalid UTF-8.")));

  Ok(s.split(",").map(|v| v.to_string()).collect())
}

#[derive(Debug)]
pub struct KexData {
  pub key_exchange: Vec<String>,
  pub host_keys: Vec<String>,
  pub encryption_cts: Vec<String>,
  pub encryption_stc: Vec<String>,
  pub mac_cts: Vec<String>,
  pub mac_stc: Vec<String>,
  pub compression_cts: Vec<String>,
  pub compression_stc: Vec<String>,
}

/// Reads the SSH_MSG_KEXINIT from the socket and returns the payload.
pub fn read_kex_payload(stream: &TcpStream) -> std::io::Result<KexData> {
  // RFC 4253, section 6.
  let packet_len = try!(read_u32(stream)) as usize;

  if packet_len > 35000 {
    return Err(Error::new(ErrorKind::Other, "Packet too large."));
  }

  // TODO: Do bounds checking on payload length.

  // byte         SSH_MSG_KEXINIT
  let message_type = try!(read_byte(stream));
  if message_type != SSH_MSG_KEXINIT {
    return Err(Error::new(ErrorKind::Other, "Unexpected message."));
  }

  // byte[16]     cookie (random bytes)
  let _ = try!(read_bytes(stream, 16));

  // name-list    kex_algorithms
  // name-list    server_host_key_algorithms
  // name-list    encryption_algorithms_client_to_server
  // name-list    encryption_algorithms_server_to_client
  // name-list    mac_algorithms_client_to_server
  // name-list    mac_algorithms_server_to_client
  // name-list    compression_algorithms_client_to_server
  // name-list    compression_algorithms_server_to_client
  let key_exchange = try!(read_name_list(stream));
  let host_keys = try!(read_name_list(stream));
  let encryption_cts = try!(read_name_list(stream));
  let encryption_stc = try!(read_name_list(stream));
  let mac_cts = try!(read_name_list(stream));
  let mac_stc = try!(read_name_list(stream));
  let compression_cts = try!(read_name_list(stream));
  let compression_stc = try!(read_name_list(stream));

  Ok(KexData {
    key_exchange: key_exchange,
    host_keys: host_keys,
    encryption_cts: encryption_cts,
    encryption_stc: encryption_stc,
    mac_cts: mac_cts,
    mac_stc: mac_stc,
    compression_cts: compression_cts,
    compression_stc: compression_stc,
  })
}

#[cfg(test)]
mod tests {
  use super::{read_identifier, read_name_list};
  use super::test::create_tcp_stream;

  #[test]
  fn test_read_identifier_basic() {
    let stream = create_tcp_stream("SSH-2.0-foobar\r\n".as_bytes());
    assert_eq!(read_identifier(&stream).unwrap(), "SSH-2.0-foobar");
  }

  #[test]
  fn test_read_identifier_skip_extra_lines() {
    let stream = create_tcp_stream("foo\r\nbar\r\nSSH-2.0-foobar\r\n".as_bytes());
    assert_eq!(read_identifier(&stream).unwrap(), "SSH-2.0-foobar");
  }

  #[test]
  #[should_panic]
  fn test_read_identifier_missing() {
    let stream = create_tcp_stream("SHS-2.0-foobar\r\n".as_bytes());
    read_identifier(&stream).unwrap();
  }

  #[test]
  #[should_panic]
  fn test_read_identifier_too_many_extra_lines() {
    let stream = create_tcp_stream("1\r\n2\r\n3\r\n4\r\n5\r\nSSH-2.0-foobar\r\n".as_bytes());
    read_identifier(&stream).unwrap();
  }

  #[test]
  #[should_panic]
  fn test_read_identifier_unexpected_end() {
    let stream = create_tcp_stream("1\r\n2\r\n3\r\n".as_bytes());
    read_identifier(&stream).unwrap();
  }

  #[test]
  fn test_name_list_basic() {
    // From RFC 4251, Section 5 > name-list.
    let stream = create_tcp_stream(&[0, 0, 0, 0x09, 0x7a, 0x6c, 0x69, 0x62, 0x2c, 0x6e, 0x6f, 0x6e, 0x65]);
    assert_eq!(read_name_list(&stream).unwrap(), ["zlib", "none"]);
  }
}
