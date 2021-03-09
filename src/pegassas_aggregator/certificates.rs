let mut config = rustls::ClientConfig::new();
use std::fs::File;
use std:io:BufReader;
use std::io;

let certfile = File::open("")
  .unwrap();// Need to designate certificate 
            // file location
let mut reader = BufReader::new(certfile);
config.root_store.add_pem_file(&mut reader)
  .unwrap();

let rc_config = Arc::new(config);
let mut client = rustls::ClientSession::new(&rc_config, "example.com");

client.write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
let mut socket = connect("example.com", 443);
loop {
  if client.wants_read() && socket.ready_for_read() {
    client.read_tls(&mut socket).unwrap();
    client.process_new_packets().unwrap();

    let mut plaintext = Vec::new();
    client.read_to_end(&mut plaintext).unwrap();
    io::stdout().write(&plaintext).unwrap();
  }

  if client.wants_write() && socket.ready_for_write() {
    client.write_tls(&mut socket).unwrap();
  }

  socket.wait_for_something_to_happen();
}