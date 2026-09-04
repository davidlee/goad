//! Stratum 2 — backend transport, configuration, host operational state and
//! event ingress. A crate of its own, which is what keeps the runtime out of
//! stratum 1's graph (ADR-001, D49).

pub mod backend;
pub mod config;
pub mod error;
pub mod host;
pub mod state;
