
use std::io::Error as IOError;
use tokio_io::{AsyncRead, AsyncWrite};
use bytes::{BufMut, BytesMut};
use tokio_io::codec::{Framed, Decoder, Encoder};
use tokio_proto::pipeline::ServerProto;

pub struct IMCodec;

impl Decoder for IMCodec {
    type Item = u8;
    type Error = IOError;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(None)
    }
}

impl Encoder for IMCodec {
    type Item = u8;
    type Error = IOError;
    fn encode(&mut self, item: Self::Item, dest: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(dest.put(item))
    }
}

pub struct IMProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for IMProto {
    type Request = u8;
    type Response = u8;
    type Transport = Framed<T, IMCodec>;
    type BindTransport = Result<Self::Transport, IOError>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(IMCodec))
    }
}
