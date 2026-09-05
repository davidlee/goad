//! Fixture: a forbidden token named only in a comment, never as code. If the
//! comment cut in `code_of` stopped applying, this line alone would fail
//! `purity.rs`'s
//! `a_forbidden_token_named_only_inside_a_comment_is_not_caught` — which is
//! what makes that control non-vacuous.

// This module deliberately performs no std::fs access; see the allowlist.
fn nothing_reaches_out() {}
