
use std::io::Error as IOError;
use futures::{future, Future, BoxFuture};
use tokio_service::Service;

use proto::{Request, Response};

pub struct IMService;

impl Service for IMService {
    type Request = Request;
    type Response = Response;

    type Error = IOError;

    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(Response::ACK).boxed()
    }
}
