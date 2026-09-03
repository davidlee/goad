//! A `Backend` that answers from a script, for the cases that are about the
//! **host** rather than about a process.
//!
//! `Host` is generic over `Backend` precisely so these cases are cheap: every
//! assertion below the transport — framing, staleness, the schedule across a
//! failure, what an absent view does to the outstanding interaction — needs
//! bytes and an error, not a spawn. The process transport adds nothing to them
//! and costs a fork each.
//!
//! It counts its calls, and that is not a nicety: the claim in AC-8 is that a
//! stale answer never reaches the backend, and a count is the only way to say
//! so from here. PHASE-08/VT-2 makes the same claim through a real process that
//! would fail if it ran — the same assertion at two costs, not a duplication.

use std::collections::VecDeque;
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use goad::semantics::protocol::canonical::Request;
use goad::shell::backend::transport::{Backend, Captured, Exchange};
use goad::shell::error::{BackendError, CleanupFailure};

pub(crate) struct FakeBackend {
  scripted: VecDeque<Exchange>,
  calls: Calls,
}

/// A count the test keeps a handle on after the `Host` has taken the backend.
///
/// Shared rather than read back off the backend because `Host` owns it and
/// exposes no accessor — and adding one to host code so that a test can look
/// inside would be the test dictating the design.
#[derive(Clone, Default)]
pub(crate) struct Calls(Arc<AtomicUsize>);

impl Calls {
  pub(crate) fn count(&self) -> usize {
    self.0.load(Ordering::Relaxed)
  }
}

impl FakeBackend {
  /// A backend that will answer with each of these in turn.
  pub(crate) fn new(scripted: Vec<Exchange>, calls: &Calls) -> Self {
    Self {
      scripted: scripted.into(),
      calls: calls.clone(),
    }
  }
}

impl Backend for FakeBackend {
  /// Not an `async fn`: this backend never awaits anything, and `clippy` is
  /// right that an `async` with no `.await` in it is noise. The seam is
  /// satisfied by a ready future, which is also the honest description of what
  /// a scripted answer is.
  fn exchange(&mut self, _request: &Request) -> impl Future<Output = Exchange> + Send {
    let nth = self
      .calls
      .0
      .fetch_add(1, Ordering::Relaxed)
      .saturating_add(1);
    let answer = self.scripted.pop_front().unwrap_or_else(|| {
      panic!("the backend was asked for exchange {nth} and the script has no such entry")
    });
    std::future::ready(answer)
  }
}

/// A backend that answered, with these bytes.
pub(crate) fn answering(body: &[u8]) -> Exchange {
  Exchange {
    result: Ok(body.to_vec()),
    stderr: Captured::default(),
    cleanup: None,
  }
}

/// A backend that did not answer, for this reason.
pub(crate) fn failing(error: BackendError) -> Exchange {
  Exchange {
    result: Err(error),
    stderr: Captured::default(),
    cleanup: None,
  }
}

/// The same, with something on stderr and a cleanup verdict — the two fields
/// that must survive whatever the result was (R-42, R-54).
pub(crate) fn failing_noisily(
  error: BackendError,
  said: &str,
  cleanup: Option<CleanupFailure>,
) -> Exchange {
  Exchange {
    result: Err(error),
    stderr: Captured {
      bytes: said.as_bytes().to_vec(),
      truncated: false,
    },
    cleanup,
  }
}
