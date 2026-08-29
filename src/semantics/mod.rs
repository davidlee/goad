//! Stratum 1 — protocol types, wire-to-canonical normalization and schedule
//! resolution. No I/O, no async runtime and no clock: `now` is always a
//! parameter (ADR-001; `design.md` I3).

pub mod error;
