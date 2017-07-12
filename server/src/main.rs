extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

use tokio_proto::TcpServer;

mod proto;
mod service;



fn main() {
    let addr = std::env::var("QUOKKA_ADDR").or(Ok(String::from("127.0.0.1:3333"))).and_then(|s| s.parse()).expect("server address");
    TcpServer::new(proto::IMProto, addr).serve(|| Ok(service::IMService));
}
