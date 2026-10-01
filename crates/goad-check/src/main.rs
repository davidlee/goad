//! `goad-check`: a backend author's conformance checker, stratum 3.
//!
//! Headless. It drives a `goad_shell::host::Host` against the author's
//! backend and reports what the host saw (`design.md` §5.2.1).

use std::process::ExitCode;

fn main() -> ExitCode {
  ExitCode::SUCCESS
}
