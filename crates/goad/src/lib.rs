// crates/goad/src/lib.rs — the module tree, and nothing else (design.md
// §5.1). One `pub mod` line per phase; ten at PHASE-08, nine after 005
// lifted `clock` to stratum 2 (005/D-9), and ten again with `draft` — the
// one new file 007 adds, and the only pure one in this crate besides
// `view_model`. Slice 009 adds two more: `instant`, which holds the two
// impure reads `draft` may not make, and `pending`, which holds the
// debounce.
// **AC-7's mechanism, held by the gate rather than by review** (`review-code.md`
// F-S3). `drawn_form` matching `FieldKind` exhaustively is what makes a sixth
// protocol kind choose between drawing and reporting; nothing kept that true.
// Two of the three shapes a wildcard could take were already caught — a
// wildcard over one variant by `match_wildcard_for_single_variants`, and one
// whose body disagrees with an absorbed kind by `fields.rs:2120` — and the
// third was not: a wildcard absorbing an existing kind with a body that agrees
// with it compiled, linted clean, left the suite green, and drew a sixth kind
// silently as a checkbox.
//
// This closes that shape. `wildcard_enum_match_arm` is a clippy *restriction*
// lint, off by default, and it is denied for **this crate only**, at no cost in
// exceptions: `goad`'s production code has no wildcard over an enum at all, and
// the two its unit tests had — destructuring the `a_choice()` fixture — are
// `let … else` now, which is the better spelling anyway.
//
// **Not workspace-wide, and the reason is not cost.** The other four such
// matches are the opposite case: `ProtocolError::source`, and three in
// `goad-shell/src/ingress/` (`envelope.rs:106`, `:118`, `mod.rs:752`). Each is
// a match that chooses *no behaviour from the variant* — no source, not an
// object, an invalid envelope — so the wildcard's body is the right answer for
// a variant that does not exist yet, and denying it there would buy four
// `#[expect]`s and no property. Widening it is a decision, not this slice's.
//
// Verified rather than assumed: with F-S3's surviving mutation applied —
// `Boolean` and `Text` arms deleted and `_ => Ok(DrawnKind::Boolean)` added —
// `cargo clippy -p goad --all-targets -- -D warnings` fails.
#![deny(clippy::wildcard_enum_match_arm)]

pub mod controller;
pub mod diagnostics;
pub mod draft;
pub mod generated;
pub mod glass;
pub mod install;
pub mod instant;
pub mod pending;
pub mod reception;
pub mod startup;
pub mod view_model;
pub mod wire;
pub mod zoom;
