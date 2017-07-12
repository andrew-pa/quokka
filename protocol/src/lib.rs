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



use std::error::Error;
use bytes::{BufMut, BytesMut};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Framed, Decoder, Encoder};
use tokio_proto::pipeline::ServerProto;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};

use bincode::{serialize, deserialize, Infinite};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Init {
        name: String
    },
    Ping
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    ACK
}

pub struct IMCodec;

impl Decoder for IMCodec {
    type Item = Request;
    type Error = IOError;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        deserialize(&src).map(Some).map_err(|e| IOError::new(IOErrorKind::Other, e))
    }
}

impl Encoder for IMCodec {
    type Item = Response;
    type Error = IOError;
    fn encode(&mut self, item: Self::Item, dest: &mut BytesMut) -> Result<(), Self::Error> {
        serialize(&item, Infinite)
            .and_then(|v| Ok(dest.extend(v)))
            .map_err(|e| IOError::new(IOErrorKind::Other, e))
    }
}

pub struct IMProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for IMProto {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, IMCodec>;
    type BindTransport = Result<Self::Transport, IOError>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(IMCodec))
    }
}
