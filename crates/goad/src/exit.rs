//! How the process ended, and which number says so (design.md §5.1, §5.2).
//!
//! Stratum 3, and it names Slint types. It owns the value and the number, and
//! nothing else. The line a person reads is `diagnostics.rs`'s, over the same
//! value and never through this module: a supervisor's contract and a person's
//! prose change for different reasons, so they are two readers of one value
//! and not one function answering a pair. The dependency runs one way —
//! `diagnostics` names `Ended`, and **nothing here imports `diagnostics`**
//! (design.md §5.1). The prose below names its two halves only to say which
//! cut they are on.
//!
//! The rule the numbers follow, stated here rather than cited: **the axis is
//! phase, not cause.** 0 is the process doing what it was asked, 2 is a host
//! that never started — whatever the cause — and 1 is a host that started and
//! stopped without being asked to. A supervisor may restart 1 and gains
//! nothing by restarting 2, and no arm below reads a `StartupError` variant to
//! decide that.

use crate::startup::StartupError;

/// How the process ended when it did not fail to start. The `Ok` channel of
/// what `run` answers, and so either an invocation that was a question and has
/// been answered — before the event-loop call is reached — or a host that
/// reached that call. `Err` is the *never started* phase whole; `Ok` is not a
/// phase, and this type is what says which end inside it happened.
///
/// **Reaching the call is what the process observes; that the loop began is
/// not.** The call can fail without the loop ever starting, so
/// `StoppedRunning` is not evidence that any work was done. The spec states
/// this cost at the seam rather than leaving it to a reader.
///
/// Not to be confused with `controller::Ending`, which says why the *serve*
/// loop stopped. This says how the *process* ended, which is a bigger thing
/// and a different one.
#[derive(Debug)]
pub enum Ended {
  /// The process did what it was asked: a question answered, or a running
  /// host asked to stop. Both ends are this one, and neither is a failure.
  AsAsked,
  /// The event-loop call ended and **no stop had been requested**, carrying
  /// the call's error when it answered one.
  ///
  /// `None` is not *no failure*: the platform backend can end the loop and
  /// answer `Ok` having cleared the error that ended it. The host reports what
  /// it was given and invents no error the platform did not raise.
  StoppedRunning(Option<slint::PlatformError>),
}

/// Which end the event-loop call was, given whether a stop had been requested
/// by the time it returned.
///
/// Decided on the request alone. The call's result says nothing about it in
/// either direction — it can answer `Err` for a requested stop and `Ok` for an
/// unrequested one — so it is read only for the error the variant carries, and
/// a call site that classifies on the result is wrong even though it compiles.
#[must_use]
pub fn ended(call: Result<(), slint::PlatformError>, stop_requested: bool) -> Ended {
  if stop_requested {
    Ended::AsAsked
  } else {
    Ended::StoppedRunning(call.err())
  }
}

/// The number, over the whole of what `run` can answer.
///
/// `u8` and not `ExitCode`: `ExitCode` carries no `PartialEq`, so a function
/// answering one could not be asserted by any test, and the constant would go
/// back to living in `main` where nothing reads it. `main` widens — the same
/// pure/impure cut `report_exit_line` and `report_exit` make.
#[must_use]
pub fn status(outcome: &Result<Ended, StartupError>) -> u8 {
  match outcome {
    Ok(Ended::AsAsked) => 0,
    Ok(Ended::StoppedRunning(_)) => 1,
    Err(_) => 2,
  }
}
