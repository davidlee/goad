//! What a person did, and why an evaluation was asked for — design.md §5.3.
//!
//! `Wire` and `Cancel`, below, are the callback-facing halves of this
//! module: everything a Slint callback may touch, and the level-held stop
//! signal both of `serve`'s `select!` arms watch (`serve` itself lives in
//! `controller.rs`, PHASE-10).

use std::fmt;
use std::future::Future;

use goad_semantics::protocol::canonical::{Event, Timestamp};
use serde_json::Value;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::{mpsc, watch};

use crate::diagnostics::BUSY_NOTICE;
use crate::generated::PromptWindow;

/// What a person did. There is deliberately **no** `Shutdown` variant:
/// stopping is a decision, not a queue position, and it travels out of band
/// (design.md §5.4, F-4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
  Evaluate(Stimulus),
  /// Both strings are opaque **selectors**, matched against retained state
  /// and never parsed back into a value. `view` is the `ViewId` the markup
  /// was given; without it a delayed click answers whichever interaction
  /// happens to be outstanding when it is dequeued (F-13).
  Choose {
    view: String,
    option: String,
  },
  OpenDiagnostics,
  CloseDiagnostics,
}

/// Why an evaluation is being asked for. The variants are the `Event.kind`
/// strings one for one (design.md §6, OQ-7): `"startup"`, `"requested"`,
/// `"scheduled"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stimulus {
  Startup,
  Requested,
  Scheduled,
}

impl Stimulus {
  /// `"startup"` | `"requested"` | `"scheduled"`. The host's own vocabulary,
  /// naming a stimulus and never a domain.
  #[must_use]
  pub fn kind(self) -> &'static str {
    match self {
      Self::Startup => "startup",
      Self::Requested => "requested",
      Self::Scheduled => "scheduled",
    }
  }

  /// The whole envelope, so the loop builds no `Event` by hand: every field
  /// is `pub` (`canonical.rs:490-497`), so no accessor is owed.
  #[must_use]
  pub fn event(self, now: Timestamp) -> Event {
    Event {
      source: "host".to_owned(),
      kind: self.kind().to_owned(),
      timestamp: now,
      data: Value::Null,
    }
  }
}

/// Everything a Slint callback may touch. Cloned into each one.
///
/// The window handle is **weak**: the component owns the callback, so a
/// strong capture is a reference cycle that leaks the window. `Debug` is
/// hand-written — `slint::Weak` implements none, by derive or by impl, and
/// `missing_debug_implementations` is `deny` (design.md §5.3, measured).
#[derive(Clone)]
pub struct Wire {
  commands: mpsc::Sender<Command>,
  cancel: Cancel,
  window: slint::Weak<PromptWindow>,
}

impl fmt::Debug for Wire {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.debug_struct("Wire").finish_non_exhaustive()
  }
}

impl Wire {
  /// The one constructor. The fields are private, so nothing outside this
  /// module can assemble a `Wire` by literal.
  #[must_use]
  pub fn new(
    commands: mpsc::Sender<Command>,
    cancel: Cancel,
    window: slint::Weak<PromptWindow>,
  ) -> Self {
    Self {
      commands,
      cancel,
      window,
    }
  }

  /// Enqueue, or say why not. A person's action is never discarded in
  /// silence.
  ///
  /// It is `try_send`, never `send().await`: a Slint callback is
  /// synchronous and runs on the UI thread, and an awaiting send on a full
  /// capacity-1 channel would block that thread against a loop that is not
  /// reading (design.md §5.3).
  ///
  /// `Full` writes `diagnostics::BUSY_NOTICE` to the window's `notice`
  /// property through the weak handle — or does nothing if the window is
  /// already gone, which is the same "nowhere left to report it" case
  /// `report_platform` documents.
  ///
  /// `Closed` does nothing, deliberately (F-20): it is not reachable while
  /// there is anything to serve, because the receiver is owned by `serve`
  /// and dropped one line before the task's own `quit_event_loop`. It
  /// shares one arm with `Ok(())` — `clippy::match_same_arms` — so the
  /// pattern stays named rather than swept into a wildcard.
  pub fn send(&self, command: Command) {
    match self.commands.try_send(command) {
      Ok(()) | Err(TrySendError::Closed(_)) => (),
      Err(TrySendError::Full(_returned)) => {
        if let Some(window) = self.window.upgrade() {
          window.set_notice(BUSY_NOTICE.into());
        }
      }
    }
  }

  /// Trip the stop signal. Every shutdown source is exactly this call.
  pub fn stop(&self) {
    self.cancel.stop();
  }
}

/// The stop signal. Level-held over `tokio::sync::watch::<bool>`: once
/// tripped it stays tripped, so a waiter arriving after the trip still
/// completes — the failure mode a bare `Notify` has.
#[derive(Debug, Clone)]
pub struct Cancel {
  tx: watch::Sender<bool>,
  rx: watch::Receiver<bool>,
}

impl Default for Cancel {
  fn default() -> Self {
    Self::new()
  }
}

impl Cancel {
  #[must_use]
  pub fn new() -> Self {
    let (tx, rx) = watch::channel(false);
    Self { tx, rx }
  }

  /// Trip it. Synchronous, idempotent, callable from a Slint callback.
  /// The send cannot fail: `self` holds a receiver, so one always exists.
  pub fn stop(&self) {
    self.tx.send(true).ok();
  }

  /// Resolves once tripped, and immediately if it already is —
  /// `wait_for` tests the current value before it waits, which is the
  /// whole of the level-held property. Clones the receiver **before** the
  /// async block, so `Cancel::stopped` keeps its `-> impl Future` shape
  /// without tripping `manual_async_fn`.
  pub fn stopped(&self) -> impl Future<Output = ()> + use<> {
    let mut receiver = self.rx.clone();
    async move {
      receiver.wait_for(|&tripped| tripped).await.ok();
    }
  }
}

// `Wire`'s synchronous paths (`Ok`, `Closed`) need no component and no
// runtime; `Full` writing `notice` through a live weak handle is
// `tests/renderer/wiring.rs`'s (VT-7, item 11g), which has a real window to
// assert against. `Cancel`'s level-held property is unit-tested here;
// `serve`'s own use of it (item 11h, 14a-d) is `wiring.rs`'s (PHASE-10).
#[cfg(test)]
mod tests {
  use goad_semantics::protocol::canonical::Timestamp;
  use serde_json::Value;
  use tokio::sync::mpsc;

  use super::{Cancel, Command, Stimulus, Wire};

  fn instant(rfc3339: &str) -> Timestamp {
    Timestamp::new(rfc3339.parse().unwrap())
  }

  #[test]
  fn send_enqueues_when_the_channel_has_room() {
    let (tx, mut rx) = mpsc::channel(1);
    let wire = Wire::new(tx, Cancel::new(), slint::Weak::default());
    wire.send(Command::OpenDiagnostics);
    assert_eq!(rx.try_recv(), Ok(Command::OpenDiagnostics));
  }

  #[test]
  fn send_does_nothing_once_the_receiver_is_gone() {
    let (tx, rx) = mpsc::channel(1);
    drop(rx);
    let wire = Wire::new(tx, Cancel::new(), slint::Weak::default());
    wire.send(Command::CloseDiagnostics); // must not panic (F-20)
  }

  #[tokio::test]
  async fn stopped_resolves_immediately_when_already_tripped() {
    let cancel = Cancel::new();
    cancel.stop();
    cancel.stopped().await;
  }

  #[tokio::test]
  async fn stopped_does_not_resolve_until_stop_is_called() {
    let cancel = Cancel::new();
    let waiting = tokio::spawn(cancel.stopped());
    tokio::task::yield_now().await;
    assert!(
      !waiting.is_finished(),
      "the level-held signal must not report tripped before stop() is called"
    );
    cancel.stop();
    waiting.await.expect("the waiting task must not panic");
  }

  // ---- VT-3: the third stimulus ----

  #[test]
  fn a_scheduled_stimulus_names_itself_scheduled() {
    assert_eq!(Stimulus::Scheduled.kind(), "scheduled");
  }

  #[test]
  fn a_scheduled_stimulus_s_event_carries_the_three_normative_fields() {
    let now = instant("2026-08-23T04:12:00Z");
    let event = Stimulus::Scheduled.event(now);
    assert_eq!(event.source, "host");
    assert_eq!(event.kind, "scheduled");
    assert_eq!(event.timestamp, now);
    assert_eq!(event.data, Value::Null);
  }
}
