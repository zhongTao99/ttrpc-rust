use std::io;
use std::os::unix::io::{AsRawFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{ready, Stream};
use tokio::net::{TcpListener, TcpStream};

/// Stream of listeners
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct TcpIncoming {
    inner: TcpListener,
}

impl TcpIncoming {
    pub fn new(listener: TcpListener) -> Self {
        Self { inner: listener }
    }
}

impl Stream for TcpIncoming {
    type Item = io::Result<TcpStream>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let (socket, _) = ready!(self.inner.poll_accept(cx))?;
        Poll::Ready(Some(Ok(socket)))
    }
}

impl AsRawFd for TcpIncoming {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}