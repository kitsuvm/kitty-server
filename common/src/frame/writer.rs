//! A writer that writes framed data to an underlying [`AsyncWrite`] stream using unsigned varint length prefixes.

use futures_core::Stream;
use futures_sink::Sink;
use tokio::io::AsyncWrite;
use tokio_util::{bytes::Bytes, codec::FramedWrite};
use unsigned_varint::codec::UviBytes;

/// A writer that writes framed data to an underlying [`AsyncWrite`] stream using unsigned varint length prefixes.
pub struct Writer<T: AsyncWrite> {
    writer: FramedWrite<T, UviBytes>,
}

impl<T: AsyncWrite> Writer<T> {
    /// Creates a new `Writer` that writes framed data to the given [`AsyncWrite`] stream.
    pub fn new(writer: T) -> Self {
        Self::from(FramedWrite::new(writer, UviBytes::default()))
    }
}

impl<T: AsyncWrite> From<FramedWrite<T, UviBytes>> for Writer<T> {
    fn from(writer: FramedWrite<T, UviBytes>) -> Self {
        Self { writer }
    }
}

impl<T: AsyncWrite + Unpin> Sink<Bytes> for Writer<T> {
    type Error = <FramedWrite<T, UviBytes> as Sink<Bytes>>::Error;

    fn poll_ready(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::pin::Pin::new(&mut self.writer).poll_ready(cx)
    }

    fn start_send(mut self: std::pin::Pin<&mut Self>, item: Bytes) -> Result<(), Self::Error> {
        std::pin::Pin::new(&mut self.writer).start_send(item)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::pin::Pin::new(&mut self.writer).poll_flush(cx)
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::pin::Pin::new(&mut self.writer).poll_close(cx)
    }
}

impl<T: AsyncWrite + Unpin + Stream> Stream for Writer<T> {
    type Item = T::Item;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::pin::Pin::new(&mut self.writer).poll_next(cx)
    }
}
