//! HTTP Upgrades
//!
//! See [this example][example] showing how upgrades work with both
//! Clients and Servers.
//!
//! [example]: https://github.com/hyperium/hyper/blob/master/examples/upgrades.rs

use std::any::TypeId;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::marker::Unpin;

use bytes::{/*Buf, BufMut, */Bytes};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_sync::oneshot;

use crate::common::io::Rewind;
use crate::common::{Future, Pin, Poll, task};

/// An upgraded HTTP connection.
///
/// This type holds a trait object internally of the original IO that
/// was used to speak HTTP before the upgrade. It can be used directly
/// as a `Read` or `Write` for convenience.
///
/// Alternatively, if the exact type is known, this can be deconstructed
/// into its parts.
pub struct Upgraded {
    io: Rewind<Box<dyn Io + Send>>,
}

/// A future for a possible HTTP upgrade.
///
/// If no upgrade was available, or it doesn't succeed, yields an `Error`.
pub struct OnUpgrade {
    rx: Option<oneshot::Receiver<crate::Result<Upgraded>>>,
}

/// The deconstructed parts of an [`Upgraded`](Upgraded) type.
///
/// Includes the original IO type, and a read buffer of bytes that the
/// HTTP state machine may have already read before completing an upgrade.
#[derive(Debug)]
pub struct Parts<T> {
    /// The original IO object used before the upgrade.
    pub io: T,
    /// A buffer of bytes that have been read but not processed as HTTP.
    ///
    /// For instance, if the `Connection` is used for an HTTP upgrade request,
    /// it is possible the server sent back the first bytes of the new protocol
    /// along with the response upgrade.
    ///
    /// You will want to check for any existing bytes if you plan to continue
    /// communicating on the IO object.
    pub read_buf: Bytes,
    _inner: (),
}

pub(crate) struct Pending {
    tx: oneshot::Sender<crate::Result<Upgraded>>
}

/// Error cause returned when an upgrade was expected but canceled
/// for whatever reason.
///
/// This likely means the actual `Conn` future wasn't polled and upgraded.
#[derive(Debug)]
struct UpgradeExpected(());

pub(crate) fn pending() -> (Pending, OnUpgrade) {
    let (tx, rx) = oneshot::channel();
    (
        Pending {
            tx,
        },
        OnUpgrade {
            rx: Some(rx),
        },
    )
}

pub(crate) trait Io: AsyncRead + AsyncWrite + Unpin + 'static {
    fn __hyper_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl dyn Io + Send {
    fn __hyper_is<T: Io>(&self) -> bool {
        let t = TypeId::of::<T>();
        self.__hyper_type_id() == t
    }

    fn __hyper_downcast<T: Io>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        if self.__hyper_is::<T>() {
            // Taken from `std::error::Error::downcast()`.
            unsafe {
                let raw: *mut dyn Io = Box::into_raw(self);
                Ok(Box::from_raw(raw as *mut T))
            }
        } else {
            Err(self)
        }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl<T: AsyncRead + AsyncWrite + Unpin + 'static> Io for T {}

// ===== impl Upgraded =====

#[cfg_attr(test, ::mutagen::mutate)] impl Upgraded {
    pub(crate) fn new(io: Box<dyn Io + Send>, read_buf: Bytes) -> Self {
        Upgraded {
            io: Rewind::new_buffered(io, read_buf),
        }
    }

    /// Tries to downcast the internal trait object to the type passed.
    ///
    /// On success, returns the downcasted parts. On error, returns the
    /// `Upgraded` back.
    pub fn downcast<T: AsyncRead + AsyncWrite + Unpin + 'static>(self) -> Result<Parts<T>, Self> {
        let (io, buf) = self.io.into_inner();
        match io.__hyper_downcast() {
            Ok(t) => Ok(Parts {
                io: *t,
                read_buf: buf,
                _inner: (),
            }),
            Err(io) => Err(Upgraded {
                io: Rewind::new_buffered(io, buf),
            })
        }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl AsyncRead for Upgraded {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        self.io.prepare_uninitialized_buffer(buf)
    }

    fn poll_read(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.io).poll_read(cx, buf)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl AsyncWrite for Upgraded {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.io).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.io).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.io).poll_shutdown(cx)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl fmt::Debug for Upgraded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Upgraded")
            .finish()
    }
}

// ===== impl OnUpgrade =====

#[cfg_attr(test, ::mutagen::mutate)] impl OnUpgrade {
    pub(crate) fn none() -> Self {
        OnUpgrade {
            rx: None,
        }
    }

    pub(crate) fn is_none(&self) -> bool {
        self.rx.is_none()
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Future for OnUpgrade {
    type Output = Result<Upgraded, crate::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        match self.rx {
            Some(ref mut rx) => Pin::new(rx).poll(cx).map(|res| match res {
                Ok(Ok(upgraded)) => Ok(upgraded),
                Ok(Err(err)) => Err(err),
                Err(_oneshot_canceled) => Err(
                    crate::Error::new_canceled().with(UpgradeExpected(()))
                ),
            }),
            None => Poll::Ready(Err(crate::Error::new_user_no_upgrade())),
        }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl fmt::Debug for OnUpgrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OnUpgrade")
            .finish()
    }
}

// ===== impl Pending =====

#[cfg_attr(test, ::mutagen::mutate)] impl Pending {
    pub(crate) fn fulfill(self, upgraded: Upgraded) {
        trace!("pending upgrade fulfill");
        let _ = self.tx.send(Ok(upgraded));
    }

    /// Don't fulfill the pending Upgrade, but instead signal that
    /// upgrades are handled manually.
    pub(crate) fn manual(self) {
        trace!("pending upgrade handled manually");
        let _ = self.tx.send(Err(crate::Error::new_user_manual_upgrade()));
    }
}

// ===== impl UpgradeExpected =====

#[cfg_attr(test, ::mutagen::mutate)] impl fmt::Display for UpgradeExpected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl StdError for UpgradeExpected {
    fn description(&self) -> &str {
        "upgrade expected but not completed"
    }
}

