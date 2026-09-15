//! What a person did, and why an evaluation was asked for — design.md §5.3.
//!
//! `Wire`, `Cancel` and `Notice`, below, are the callback-facing halves of
//! this module: everything a Slint callback may touch, the level-held stop
//! signal both of `serve`'s `select!` arms watch, and the back-pressure
//! signal `serve` samples at present time (`serve` itself lives in
//! `controller.rs`).

use std::future::Future;

use goad_semantics::protocol::canonical::{Event, Timestamp};
use serde_json::Value;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::{mpsc, watch};

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
/// It names no Slint type and no generated type: everything it holds is one
/// half of a channel, and everything it does is enqueue a command or move a
/// signal. Writing the window is `Glass::present`'s, from the frame
/// (design.md §5.3).
#[derive(Debug, Clone)]
pub struct Wire {
  commands: mpsc::Sender<Command>,
  cancel: Cancel,
  notice: Notice,
}

impl Wire {
  /// The one constructor. The fields are private, so nothing outside this
  /// module can assemble a `Wire` by literal.
  #[must_use]
  pub fn new(commands: mpsc::Sender<Command>, cancel: Cancel, notice: Notice) -> Self {
    Self {
      commands,
      cancel,
      notice,
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
  /// `Full` **raises** the back-pressure signal and `Ok` lowers it. This
  /// writes no window property: `serve` samples the signal at present time
  /// and `Glass::present` writes `diagnostics::BUSY_NOTICE` from the frame,
  /// so the explanation outlives the present that corrects the dropped
  /// action (design.md §5.4).
  ///
  /// `Closed` does nothing, deliberately (F-20): it is not reachable while
  /// there is anything to serve, because the receiver is owned by `serve`
  /// and dropped one line before the task's own `quit_event_loop`. It leaves
  /// the signal exactly as it found it — there is nothing left to explain a
  /// dropped action to, and nothing left to lower it either.
  pub fn send(&self, command: Command) {
    match self.commands.try_send(command) {
      Ok(()) => self.notice.set(false),
      Err(TrySendError::Full(_returned)) => self.notice.set(true),
      Err(TrySendError::Closed(_)) => (),
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

/// The back-pressure signal. `tokio::sync::watch::<bool>` again, and
/// `Cancel`'s route edge for edge: constructed in `main`, cloned into `Wire`
/// for synchronous setting from a Slint callback, and handed to `serve` for
/// the loop to read. The one difference is direction — `Cancel` is one-way
/// and tripped forever, this is set both ways.
///
/// `watch` rather than a flag or a `Notify` because it **retains its last
/// sent value**, which is the whole of the repair: a raised notice has to
/// survive the present that corrects the action it explains, and every
/// present after it, until a successful send lowers it (design.md §5.3,
/// §5.4).
#[derive(Debug, Clone)]
pub struct Notice {
  tx: watch::Sender<bool>,
  rx: watch::Receiver<bool>,
}

impl Default for Notice {
  fn default() -> Self {
    Self::new()
  }
}

impl Notice {
  #[must_use]
  pub fn new() -> Self {
    let (tx, rx) = watch::channel(false);
    Self { tx, rx }
  }

  /// Raise it or lower it. Synchronous, idempotent, callable from a Slint
  /// callback. The send cannot fail: `self` holds a receiver, so one always
  /// exists.
  pub fn set(&self, raised: bool) {
    self.tx.send(raised).ok();
  }

  /// The current value, read without awaiting. Nothing ever waits on this
  /// signal — `serve` samples it at present time — so the read is
  /// `watch::Receiver::borrow` and not `Cancel::stopped`'s `wait_for`.
  #[must_use]
  pub fn raised(&self) -> bool {
    *self.rx.borrow()
  }
}

// All three of `Wire::send`'s arms are synchronous and need no component and
// no runtime, now that `Full` raises a signal rather than writing a window;
// what still needs a real window is the notice **reaching the screen and
// staying there**, which is `tests/renderer/wiring.rs`'s (VT-7, item 11g).
// `Cancel`'s level-held property and `Notice`'s retention are unit-tested
// here; `serve`'s own use of either (item 11h, 14a-d) is `wiring.rs`'s.
#[cfg(test)]
mod tests {
  use goad_semantics::protocol::canonical::Timestamp;
  use serde_json::Value;
  use tokio::sync::mpsc;

  use super::{Cancel, Command, Notice, Stimulus, Wire};

  fn instant(rfc3339: &str) -> Timestamp {
    Timestamp::new(rfc3339.parse().unwrap())
  }

  #[test]
  fn send_enqueues_when_the_channel_has_room() {
    let (tx, mut rx) = mpsc::channel(1);
    let wire = Wire::new(tx, Cancel::new(), Notice::new());
    wire.send(Command::OpenDiagnostics);
    assert_eq!(rx.try_recv(), Ok(Command::OpenDiagnostics));
  }

  #[test]
  fn send_does_nothing_once_the_receiver_is_gone() {
    let (tx, rx) = mpsc::channel(1);
    drop(rx);
    let notice = Notice::new();
    notice.set(true);
    let wire = Wire::new(tx, Cancel::new(), notice.clone());
    wire.send(Command::CloseDiagnostics); // must not panic (F-20)
    assert!(
      notice.raised(),
      "a closed channel leaves the signal as it found it: nothing was delivered, so \
       nothing lowers it"
    );
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

  // ---- the back-pressure signal ----

  #[test]
  fn a_raised_notice_stays_raised_until_it_is_lowered() {
    let notice = Notice::new();
    assert!(!notice.raised(), "a fresh notice is not raised");
    notice.set(true);
    assert!(notice.raised());
    assert!(
      notice.raised(),
      "the signal retains its last sent value: reading it does not consume it"
    );
    notice.set(false);
    assert!(!notice.raised());
  }

  #[test]
  fn a_notice_raised_on_one_clone_is_read_from_another() {
    let held_by_the_edge = Notice::new();
    let read_by_the_loop = held_by_the_edge.clone();
    held_by_the_edge.set(true);
    assert!(
      read_by_the_loop.raised(),
      "the edge sets and the loop reads: they are clones of one signal"
    );
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
