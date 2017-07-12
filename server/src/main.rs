extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate protocol;

use std::default::Default;
use tokio_proto::TcpServer;
use r2d2_redis::RedisConnectionManager;

use std::io::{Error as IOError, ErrorKind as IOErrorKind};

mod service;

fn main() {
    let addr = std::env::var("QUOKKA_ADDR").or(Ok(String::from("127.0.0.1:3333"))).and_then(|s| s.parse()).expect("server address");
    let dbm = RedisConnectionManager::new(match std::env::var("QUOKKA_REDIS_URL") {
        Ok(ar) => ar,
        Err(_) => String::from("redis://localhost")
    }.as_ref()).expect("redis db");
    let dbpool = r2d2::Pool::new(Default::default(), dbm).expect("db connection pool");
    TcpServer::new(protocol::IMProto, addr).serve(move || Ok(service::IMService { dbc: dbpool.get().map_err(|_| IOError::new(IOErrorKind::TimedOut, ""))? }));
}
