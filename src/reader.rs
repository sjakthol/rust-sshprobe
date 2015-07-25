use std;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::net::TcpStream;

/// Reads bytes from the TCP stream until CRLF is found or the given limit is
/// reached.
pub fn read_line(stream: &TcpStream, limit: usize) -> std::io::Result<String> {
  let mut result = String::with_capacity(limit);

  for byte in stream.take(limit as u64).bytes() {
    let raw = try!(byte);
    let chr = raw as char;

    result.push(chr);

    if chr == '\n' {
      break;
    }
  }

  if result.ends_with("\r\n") {
    // .truncate() panics if the new length is not at char boundary. Since we
    // know the string ends with \r\n, that won't happen.
    let new_len = result.len() - 2;
    result.truncate(new_len);

    // Return the result.
    Ok(result)
  } else if result.len() == limit {
    // The line did not fit in the given limit.
    Err(Error::new(ErrorKind::Other, "Line too long."))
  } else {
    // The line ending was invalid or missing.
    Err(Error::new(ErrorKind::Other, "The line must end in CRLF sequence."))
  }
}

/// Reads the given number of bytes from the stream into a Vec<u8> or returns
/// an error if the exact number of bytes cannot be read.
pub fn read_bytes(stream: &TcpStream, amount: usize) -> std::io::Result<Vec<u8>> {
  let mut buf: Vec<u8> = vec![];
  let read = try!(stream.take(amount as u64).read_to_end(&mut buf));

  if read < amount {
    Err(Error::new(ErrorKind::Other, "Unexpected end of stream."))
  } else {
    Ok(buf)
  }
}

/// Reads four bytes from the stream and combines them together to form a 32 bit
/// unsigned integer.
pub fn read_u32(stream: &TcpStream) -> std::io::Result<u32> {
  let mut buf = try!(read_bytes(stream, 4));

  Ok(buf.iter().enumerate().fold(0, |result, (i, value)| {
    result | ((*value as u32) << (3 - i) * 8)
  }))
}

#[cfg(test)]
mod tests {
  use super::*;
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

  #[test]
  fn test_read_u32_basic() {
    let stream = create_tcp_stream(&[0x29, 0xb7, 0xf4, 0xaa, 0xff, 0xff]);
    assert_eq!(read_u32(&stream).unwrap(), 0x29b7f4aa);
  }

  #[test]
  fn test_read_u32_basic_zeroes() {
    let stream = create_tcp_stream(&[0x29, 0x00, 0xf4, 0xaa, 0xff, 0xff]);
    assert_eq!(read_u32(&stream).unwrap(), 0x2900f4aa);
  }

  #[test]
  fn test_read_u32_too_little_data() {
    let stream = create_tcp_stream(&[0x29, 0xb7, 0xf4]);
    assert!(read_u32(&stream).is_err());
  }

  #[test]
  fn test_read_bytes_basic() {
    let stream = create_tcp_stream(&[1,2,3,4,5,6]);
    assert_eq!(read_bytes(&stream, 2).unwrap(), [1,2]);
    assert_eq!(read_bytes(&stream, 3).unwrap(), [3,4,5]);
    assert_eq!(read_bytes(&stream, 1).unwrap(), [6]);
  }

  #[test]
  fn test_read_bytes_too_little_data() {
    let stream = create_tcp_stream(&[1,2,3,4,5,6]);
    assert!(read_bytes(&stream, 10).is_err())
  }
}
