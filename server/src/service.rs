
use std::io::Error as IOError;
use futures::{future, Future, BoxFuture};
use tokio_service::Service;

pub struct IMService;

impl Service for IMService {
    type Request = u8;
    type Response = u8;

    type Error = IOError;

    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}
