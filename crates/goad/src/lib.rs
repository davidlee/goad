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
// **Not workspace-wide, and widening it is a decision for another slice**
// (`review-code.md` F-T3, which corrected the count and one characterisation
// this comment previously carried). Catch-all arms over an enum outside
// `crates/goad` number **six to eight**, not four: `ProtocolError::source`
// (`goad-semantics/src/error.rs:238`), `goad-shell/src/ingress/envelope.rs:106`
// and `:118`, `ingress/mod.rs:752` and `:585`, `config.rs:47`, and two unit-test
// arms in `state.rs:171` and `:186` — which count, because this deny
// demonstrably reaches `goad`'s own lib *test* target. The three arms in
// `normalize.rs` are correctly excluded: they match on `wire.kind.as_str()`, a
// `&str`, which this lint does not see.
//
// So the reason is **cost**, and more of it than the sentence that first stood
// here claimed: more sites, not fewer. That strengthens the conclusion rather
// than weakening it, which is why the conclusion is unchanged.
//
// **No claim is made here about how many of those arms read the matched
// variant**, and the omission is deliberate. Two attempts at that sentence have
// now been wrong, in opposite directions (`review-code.md` F-T3 and its
// contest): the second called `ingress/mod.rs`'s
// `other => InvalidEnvelope(other)` an arm that chooses no behaviour from the
// variant, when `Refusal::reason`
// splits `InvalidEnvelope(ReservedSource)` from every other and the variant
// therefore reaches the wire. The axis is not load-bearing — the decision is
// about the cost of widening the deny, which the count establishes on its own —
// and a claim nothing checks, restated, is how this comment went wrong twice.
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
