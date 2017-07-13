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

use bincode::{serialize, serialized_size, deserialize, Infinite};

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
        if src.len() == 0 { return Ok(None); }
        let der: Result<Request,_> = deserialize(src);
        println!("decode {:?} [ {:?} ]", der, src);
        match der {
            Ok(v) => {
                src.split_to(serialized_size(&v) as usize); //consume bytes
                Ok(Some(v))
            },
            Err(e) => Err(IOError::new(IOErrorKind::Other, e))
        }
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

pub struct IMClientCodec;

impl Decoder for IMClientCodec {
    type Item = Response;
    type Error = IOError;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() == 0 { return Ok(None); }
        let der: Result<Response,_> = deserialize(src);
        println!("decode {:?} [ {:?} ]", der, src);
        match der {
            Ok(v) => {
                src.split_to(serialized_size(&v) as usize); //consume bytes
                Ok(Some(v))
            },
            Err(e) => Err(IOError::new(IOErrorKind::Other, e))
        }
    }
}

impl Encoder for IMClientCodec {
    type Item = Request;
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
