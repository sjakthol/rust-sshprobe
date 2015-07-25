use std;
use std::io::{Error, ErrorKind};
use std::io::Read;
use std::net::TcpStream;

/// Reads bytes from the TCP stream until CRLF is found or the given limit is
/// reached.
pub fn read_line(stream: &TcpStream, limit: usize) -> std::io::Result<String> {
  let mut res = vec!();
  for (i, value) in stream.bytes().enumerate() {
    let byte = try!(value);
    if byte == '\n' as u8 {
      // The end of the line. Make sure it actually was CRLF.
      let second_to_last = res.pop().unwrap_or(0);
      if second_to_last != '\r' as u8 {
        return Err(Error::new(ErrorKind::Other, "LF without CR."));
      }

      return match String::from_utf8(res) {
        Ok(s) => Ok(s),
        Err(_) => Err(Error::new(ErrorKind::Other, "Invalid UTF8.")),
      }
    }

    // Add the byte to this line.
    res.push(byte);

    // Check the limit.
    if i == limit {
      return Err(Error::new(ErrorKind::Other, "Line too long."));
    }
  }

  Err(Error::new(ErrorKind::Other, "Unexpected end of stream."))
}

#[cfg(test)]
mod tests {
  use super::read_line;
  use test::create_tcp_stream;

  #[test]
  fn test_readline_basic() {
    let stream = create_tcp_stream("foo\r\n".as_bytes());
    assert_eq!(read_line(&stream, 255).unwrap(), "foo");
  }

  #[test]
  fn test_readline_empty_line() {
    let stream = create_tcp_stream("\r\n".as_bytes());
    assert_eq!(read_line(&stream, 255).unwrap(), "");
  }

  #[test]
  fn test_readline_multiple_lines_reads_only_first() {
    let stream = create_tcp_stream("bar\r\nfoo\r\n".as_bytes());
    assert_eq!(read_line(&stream, 255).unwrap(), "bar");
    assert_eq!(read_line(&stream, 255).unwrap(), "foo");
  }

  #[test]
  fn test_readline_line_too_long() {
    let stream = create_tcp_stream("barbazfoo\r\n".as_bytes());
    assert!(read_line(&stream, 5).is_err());
  }

  #[test]
  fn test_readline_line_at_limit() {
    let stream = create_tcp_stream("123\r\n".as_bytes());
    assert_eq!(read_line(&stream, 5).unwrap(), "123");
  }

  #[test]
  fn test_readline_no_cr_before_lf() {
    let stream = create_tcp_stream("barbazfoo\n".as_bytes());
    assert!(read_line(&stream, 255).is_err());
  }

  #[test]
  fn test_readline_no_crlf() {
    let stream = create_tcp_stream("barbazfoo".as_bytes());
    assert!(read_line(&stream, 255).is_err());
  }

  #[test]
  fn test_readline_no_lf() {
    let stream = create_tcp_stream("barb\razfoo".as_bytes());
    assert!(read_line(&stream, 255).is_err());
  }

  #[test]
  fn test_readline_empty() {
    let stream = create_tcp_stream("".as_bytes());
    assert!(read_line(&stream, 255).is_err());
  }

  #[test]
  fn test_readline_only_lf() {
    let stream = create_tcp_stream("\n".as_bytes());
    assert!(read_line(&stream, 255).is_err());
  }
}
