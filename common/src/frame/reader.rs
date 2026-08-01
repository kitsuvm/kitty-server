//! A reader that reads frames from a stream of bytes using the unsigned varint length prefix encoding.

use futures_core::Stream;
use futures_sink::Sink;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};
use unsigned_varint::codec::UviBytes;

/// A reader that reads frames from a stream of bytes using the unsigned varint length prefix encoding.
pub struct Reader<T: AsyncRead> {
    /// The underlying framed reader that reads frames from the stream of bytes.
    reader: FramedRead<T, UviBytes>,
}

impl<T: AsyncRead> Reader<T> {
    /// Creates a new [`Reader`] from the given [`AsyncRead`] stream.
    pub fn new(reader: T) -> Self {
        Self::from(FramedRead::new(reader, UviBytes::default()))
    }
}

impl<T: AsyncRead> From<FramedRead<T, UviBytes>> for Reader<T> {
    fn from(reader: FramedRead<T, UviBytes>) -> Self {
        Self { reader }
    }
}

impl<T: Sink<I> + AsyncRead + Unpin, I> Sink<I> for Reader<T> {
    type Error = T::Error;

    fn poll_ready(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::pin::Pin::new(&mut self.reader).poll_ready(cx)
    }

    fn start_send(mut self: std::pin::Pin<&mut Self>, item: I) -> Result<(), Self::Error> {
        std::pin::Pin::new(&mut self.reader).start_send(item)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::pin::Pin::new(&mut self.reader).poll_flush(cx)
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::pin::Pin::new(&mut self.reader).poll_close(cx)
    }
}

impl<T: AsyncRead + Unpin + Stream> Stream for Reader<T> {
    type Item = Result<<UviBytes as Decoder>::Item, <UviBytes as Decoder>::Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::pin::Pin::new(&mut self.reader).poll_next(cx)
    }
}
