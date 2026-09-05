//! The wall clock — design.md §5.4, one function wide (D15).
//!
//! Not a timer: slice 003 owns the schedule. This module only answers "what
//! time is it", as a `Result` rather than a panic, so an instant outside
//! jiff's representable range becomes a refusal rather than taking the
//! process down.

use goad_semantics::protocol::canonical::Timestamp;
use std::time::SystemTime;

/// A wall clock. A `fn` pointer, so it is `Copy`, `Send`, needs no trait and
/// no lifetime — a test supplies a fixed instant in one line, and PHASE-07's
/// `serve` takes one of these rather than a `dyn Fn`.
pub type Clock = fn() -> Result<Timestamp, ClockError>;

/// Why the wall clock could not be read.
#[derive(Debug)]
pub enum ClockError {
  /// The system clock reads before the Unix epoch.
  BeforeEpoch,
  /// The instant is outside the range jiff represents.
  OutOfRange(jiff::Error),
}

impl std::fmt::Display for ClockError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::BeforeEpoch => {
        write!(f, "the system clock reads before 1970-01-01T00:00:00Z")
      }
      Self::OutOfRange(error) => {
        write!(
          f,
          "the system clock is outside the range this host represents: {error}"
        )
      }
    }
  }
}

impl std::error::Error for ClockError {}

/// `SystemTime::now().duration_since(UNIX_EPOCH)`, then
/// `jiff::Timestamp::from_nanosecond`. **Not** `jiff::Timestamp::now()`,
/// which needs jiff's `std` feature — enabling it in stratum 3 unifies it
/// into stratum 1's build, weakening the purity claim in a way the manifest
/// test cannot see (`Cargo.toml:23`, D25).
///
/// The `SystemTimeError` `duration_since` returns is discarded with a
/// **named** binding: it carries only the size of the negative offset,
/// `BeforeEpoch`'s sentence already says everything a reader needs, and
/// `clippy::map_err_ignore` refuses the wildcard.
///
/// # Errors
///
/// [`ClockError::BeforeEpoch`] when the system clock reads before the Unix
/// epoch; [`ClockError::OutOfRange`] when the instant is outside jiff's
/// range.
pub fn wall_clock() -> Result<Timestamp, ClockError> {
  let since_epoch = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .map_err(|_negative| ClockError::BeforeEpoch)?;

  // Built from the second and sub-second parts rather than `as_nanos()`
  // (`u128`) so the widening conversions are both plain `i128::from` — no
  // `as`, and no fallible width-preserving conversion to thread an error
  // through for a case that cannot occur this side of the heat death of the
  // universe.
  let nanos =
    i128::from(since_epoch.as_secs()) * 1_000_000_000 + i128::from(since_epoch.subsec_nanos());
  let instant = jiff::Timestamp::from_nanosecond(nanos).map_err(ClockError::OutOfRange)?;
  Ok(Timestamp::new(instant))
}
