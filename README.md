An utility for reading algorithms supported by an SSH server.

The program connects to a server, performs a protocol version exchange and reads
the SSH_MSG_KEXINIT message of the server. It parses the fields of that message
and prints out
* key exchange algorithms
* available host key types
* symmetric encryption algorithms for both directions
* MAC algorithms for both directions
* compression algorithms for both directions

# Usage
The program is written in [Rust](http://rust-lang.org) and it should work with
stable Rust versions. The target can be specified as an IP adrress or hostname
either with or without a port (if port is not specified it defauls to 22).

## Example
```
$ cargo run localhost
[Local] Resolving address 'localhost'
[Local] Connecting to '127.0.0.1:22'
[Local] Connection established.
[Remote] Version: SSH-2.0-OpenSSH_6.6.1p1 Ubuntu-2ubuntu2
Key Exchange Algorithms:
  curve25519-sha256@libssh.org
  ...
  diffie-hellman-group1-sha1
Host Keys:
  ssh-rsa
  ...
  ssh-ed25519
Encryption (client -> server):
  aes128-ctr
  ...
  rijndael-cbc@lysator.liu.se
Encryption (server -> client):
  aes128-ctr
  ...
  rijndael-cbc@lysator.liu.se
MAC (client -> server):
  hmac-md5-etm@openssh.com
  ...
  hmac-md5-96
MAC (server -> client):
  hmac-md5-etm@openssh.com
  ...
  hmac-md5-96
Compression (client -> server):
  none
  zlib@openssh.com
Compression (server -> client):
  none
  zlib@openssh.com
```

