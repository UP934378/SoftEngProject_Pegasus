use mailin_embedded::{Server, SslConfig, Handler};

#[derive(Clone)]
struct MyHandler {}
impl Handler for MyHandler{}

let handler = MyHandler {};
let mut server = Server::new(handler);

server.with_name("example.com")
   .with_ssl(SslConfig::None)?
   .with_addr("127.0.0.1:25")?;
server.serve();