//! Waiting on something the driving code does not control directly — an
//! invocation landing, a view arriving — rather than assuming a fixed delay
//! covers it.
//!
//! Shared rather than restated. The poll loop
//! was written twice, five lines apart in shape: once in
//! `crates/goad/tests/renderer/harness.rs`, once in
//! `crates/goad/tests/event_loop_schedule/scheduling.rs`, whose stated reason
//! was that its target includes only `scripting.rs` — a fact about the file
//! layout, not an argument that a second support file was unavailable. The
//! `#[path]`-included support file this workspace already shares between
//! targets is exactly what carries it.
//!
//! The **non-asserting** half lives here, and the asserting wrapper stays in
//! `renderer/harness.rs`. That split is what the event-loop tier needs: a
//! poll that panics
//! inside a Slint event-loop task never reaches the code that stops the loop,
//! so that test wants the bool and the assertion outside the loop, while the
//! twenty-odd renderer-tier call sites want the panic where they stand.

use std::time::Duration;

/// The bound every liveness wait in this workspace is held to.
///
/// Five seconds, not two. The slice-003 timed
/// assertions this bounds measured ~250-290 ms on the gate, which is a ~7x
/// margin against two seconds and near the plan's own 5x STOP floor; the same
/// measurements are ~17-20x against five. Nothing waits longer in the passing
/// case — the poll returns as soon as the predicate holds — so the whole cost
/// of the wider bound is paid only by a test that was going to fail anyway.
pub(crate) const LIVENESS_BOUND: Duration = Duration::from_secs(5);

/// How often the predicate is re-read. Short enough that the observation is
/// not itself a source of margin, long enough not to spin.
const POLL_INTERVAL: Duration = Duration::from_millis(5);

/// Poll `predicate` until it is true or `bound` passes, whichever is first.
/// Returns whether it became true.
pub(crate) async fn within(bound: Duration, mut predicate: impl FnMut() -> bool) -> bool {
  let deadline = std::time::Instant::now() + bound;
  loop {
    if predicate() {
      return true;
    }
    if std::time::Instant::now() >= deadline {
      return false;
    }
    tokio::time::sleep(POLL_INTERVAL).await;
  }
}
