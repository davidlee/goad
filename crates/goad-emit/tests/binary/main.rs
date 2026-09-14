//! Stratum 3 test target — **the built binary**, run as a process against a
//! real socket.
//!
//! What this tier holds is the binary and nothing else: its exit codes, its
//! stderr, and the bytes it puts on a socket (`design.md` §9). Three things it
//! deliberately does not hold, because each is held properly somewhere else
//! and a second, weaker case here would only look like coverage (005/F-15):
//!
//! - **`SPEC-003/R-6`'s framing** is PHASE-02/VT-1's, against the *real*
//!   listener, where a mis-framed write draws `timed_out` and reds the case.
//!   The fake below reads one line and would accept a framing the host does
//!   not.
//! - **R-7's byte and time bounds** are 004's listener cases'. Emit does not
//!   second-guess them, exactly as it does not pre-empt R-13 (AC-5).
//! - **The backend leg** is AC-8's, and a person's: no test target links both
//!   this binary and a running host.
//!
//! The fake listener is blocking `std::os::unix::net` on its own thread — no
//! runtime, no `tokio`, no `ingress::bind`, none of which this crate may name
//! (AC-7). Every case passes `--socket`, so none reads a configuration file or
//! an environment variable; and every fake either answers or closes, because
//! `send` has no deadline of its own (005/D-10) and a silent fake would hang.

// `#[cfg(test)]` on the declaration, as `goad-shell`'s own integration target
// explains: a `tests/` target is always built with `--test`, so this is never
// off, and without it `clippy::tests_outside_test_module` fires and
// `clippy.toml`'s `allow-expect-in-tests` does not apply.
#[cfg(test)]
mod exchange;
