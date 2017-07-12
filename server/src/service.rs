use std::io::Error as IOError;
use futures::{future, Future, BoxFuture};
use tokio_service::Service;
use protocol::{Request, Response};
use r2d2;
use r2d2_redis::RedisConnectionManager;
use redis::{Client, Connection, Commands};

pub struct IMService {
    pub dbc: r2d2::PooledConnection<RedisConnectionManager>
}

impl Service for IMService {
    type Request = Request;
    type Response = Response;

    type Error = IOError;

    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let n: i64 = self.dbc.incr("request_counter", 1).expect("incr req counter");
        println!("{:?}, {}", req, n);
        future::ok(Response::ACK).boxed()
    }
}
