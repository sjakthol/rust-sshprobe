pub mod socket;
pub mod reader;
pub mod test;

use std::io::{Error, ErrorKind};
use std::net::TcpStream;
use reader::read_line;

static SSH_IDENTIFIER_MAX_LINE_LEN: usize = 255;
static SSH_MAX_LINES_BEFORE_IDENTIFIER: usize = 5;

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

#[cfg(test)]
mod tests {
  use super::read_identifier;
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
}
