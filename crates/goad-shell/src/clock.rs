//! The wall clock — 003's design.md §5.4, one function wide (D15).
//!
//! Not a timer: slice 003 owns the schedule. This module only answers "what
//! time is it", as a `Result` rather than a panic, so an instant outside
//! jiff's representable range becomes a refusal rather than taking the
//! process down.
//!
//! Stratum 2, not 3: reading a clock is I/O, and by 005 there are two entry
//! points that need the answer. One of them must not link the renderer, so a
//! second copy of this judgement would be the alternative (005/D-9).

use goad_semantics::protocol::canonical::Timestamp;
use std::time::SystemTime;

/// A wall clock. A `fn` pointer, so it is `Copy`, `Send`, needs no trait and
/// no lifetime — a test supplies a fixed instant in one line, and PHASE-10's
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
/// which needs jiff's `std` feature — and features unify across a build, so
/// enabling it anywhere enables it for every member of that build, stratum 1
/// included, which carries `jiff` with `default-features = false` for exactly
/// that reason (the workspace `Cargo.toml`'s `jiff` line, D25). The manifest
/// test cannot see a feature, only a name
/// (`docs/policy/001-the-phase-gate.md`).
///
/// **Slice 009 enables `std` from `crates/goad`, which narrows this
/// workaround's reach rather than retiring it.** Unification is per *build*,
/// and "the workspace build" is one of several. Three of them, three answers:
///
/// - `cargo build --workspace` and `cargo test --workspace` — stratum 1 links
///   a `jiff` built with `std` whatever this function does. That much of the
///   paragraph above has expired.
/// - every build that excludes `crates/goad` — `-p goad-shell`, `-p goad-emit`
///   (stratum 2 without the renderer), and any future member of that shape —
///   resolves `jiff` with no features at all, so `Timestamp::now()` still
///   would not compile here. This is not dead code kept for an expired
///   reason.
/// - `cargo test -p goad-semantics` — the gate's third command builds neither
///   stratum above stratum 1, so it resolves `jiff` with stratum 1's own
///   features either way and never sees the difference.
///
/// `docs/slices/009/design.md` §10 carries the argument that change owed.
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
