//! The transport seam — `design.md` §5.2.
//!
//! One trait, and the value an exchange always produces. The trait owns
//! *framing* as well as transmission: it takes a canonical `Request` and
//! serializes internally, because the two implementations this seam exists for
//! differ in exactly that respect — one JSON document per process here, one
//! JSONL line per exchange on slice 005's persistent socket (brief §6). A trait
//! taking pre-serialized bytes would have baked this transport's frame into the
//! seam.

use std::future::Future;

use crate::semantics::protocol::canonical::Request;
use crate::shell::error::{BackendError, CleanupFailure};

/// A transport that can carry one request and bring back one answer.
///
/// `&mut self`, though the process transport is stateless and spawns per
/// exchange: slice 005's socket transport holds a connection, and a connection
/// is mutable state an exchange advances. `&self` would force that
/// implementation into interior mutability to satisfy a signature — a `Mutex`
/// guarding against concurrency brief §12 says does not exist (F-1, P3). It
/// also gives I6 — at most one exchange in flight — to the compiler rather than
/// to convention.
///
/// Async fn in trait, so no `async_trait` dependency and no `Box::pin` per
/// call. The stated cost: AFIT traits are not `dyn`-compatible, so slice 005's
/// socket-first, process-fallback selection needs an enum over the concrete
/// implementations.
pub trait Backend {
  fn exchange(&mut self, request: &Request) -> impl Future<Output = Exchange> + Send;
}

/// A completed exchange.
///
/// Note there is no outer `Result`: the exchange itself always completes
/// (D22, D33, D40). A `Result<Exchange, BackendError>` would put the capture on
/// the `Ok` side and so lose it on every `Err` — which is to say on exactly the
/// paths stderr exists for. That is D23's rule one layer down: a value every
/// path produces must not live on the success branch (F-24, F-39).
#[derive(Debug)]
pub struct Exchange {
  /// The response body, or the reason there is none. Bytes, not `String`:
  /// invalid UTF-8 becomes a protocol error where `from_slice` runs, rather
  /// than a lossy replacement here.
  pub result: Result<Vec<u8>, BackendError>,
  /// Diagnostic, and carried either way (R-42).
  pub stderr: Captured,
  /// `Some` = the host could not establish that the child was killed, reaped
  /// and its stderr drained within the cleanup budget. Independent of
  /// `result`; all four combinations are meaningful (R-54).
  pub cleanup: Option<CleanupFailure>,
}

impl Exchange {
  /// A failure with nothing captured and nothing to dispose of.
  ///
  /// Only for the paths where no process ever ran: no stderr exists to have
  /// captured, and `cleanup` is `None` because there was never a child. Where a
  /// child *does* exist, `process::cleanup_only` is the constructor to reach
  /// for — I13 is that a child, once spawned, is disposed of on every returning
  /// path.
  pub(super) fn failed(error: BackendError) -> Self {
    Self {
      result: Err(error),
      stderr: Captured::default(),
      cleanup: None,
    }
  }
}

/// Bounded capture. `truncated` is where AC-5's cap becomes observable.
#[derive(Debug, Default)]
pub struct Captured {
  pub bytes: Vec<u8>,
  pub truncated: bool,
}
