extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate protocol;

use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_io::{AsyncRead};
use std::net::ToSocketAddrs;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use futures::{Future, Stream, Sink, future};

fn main() {
    let mut core = Core::new().expect("tokio core");
    let hnd = core.handle();
    let addr = std::env::args().nth(1).unwrap_or(String::from("127.0.0.1:3333")).to_socket_addrs()
        .and_then(|mut v| v.next().ok_or(IOError::new(IOErrorKind::Other, "no addresses"))).expect("server address");

    let socket = TcpStream::connect(&addr, &hnd);
    let frame = socket.and_then(|socket| future::ok(socket.framed(protocol::IMClientCodec)));
    let req = frame.and_then(|frame| frame.send(protocol::Request::Ping));
    let res = req.and_then(|mut frame| frame.for_each(|r| future::ok(println!("={:?}",r))));
    core.run(res).expect("run req");
}
