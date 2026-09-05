//! The `include_modules!()` quarantine (design.md §5.2, §5.5 A-1). One
//! module, one blanket suppression, wrapping the Slint compiler's generated
//! code and re-exporting it for the rest of the crate.
//!
//! `expect` rather than `allow`: it self-cleans. If a Slint upgrade stops
//! emitting one of these twelve, the build fails via
//! `unfulfilled_lint_expectations` and the list is corrected here, with the
//! diagnostic that forced the change recorded in `notes.md` — never silently
//! left to rot.
//!
//! The twelve, measured against this build (`research.md:402-408`, ten
//! clippy restriction lints plus two rustc lints unsuppressed by the
//! generated code's own `#![allow(clippy::all, clippy::pedantic,
//! clippy::nursery)]`, which covers none of them):
#![expect(
  clippy::as_conversions,
  clippy::unwrap_used,
  clippy::shadow_unrelated,
  clippy::same_name_method,
  clippy::panic,
  clippy::indexing_slicing,
  clippy::let_underscore_must_use,
  clippy::clone_on_ref_ptr,
  clippy::todo,
  clippy::pub_use,
  unreachable_pub,
  missing_debug_implementations,
  reason = "Slint's generated code trips these restriction lints; the quarantine is the one module that absorbs them (design.md §5.2)"
)]

slint::include_modules!();
