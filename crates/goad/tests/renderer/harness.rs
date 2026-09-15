//! What two or more case files in this target need beyond the shared
//! host-driving helpers: a headless window and tray, a glass over them, a
//! fixed clock, the element queries that reach an option and its fields, the
//! backend that logs each raw request, and the poll loop `serve`-driven tests
//! use to observe an event the driving code does not control directly.
//! Anything two or more case files here need lives in this file; anything one
//! of them needs stays where it is.
//! What both *tiers* need lives in `tests/support/driving.rs` instead
//! (`design.md` §12.8, `crates/goad-shell/tests/integration/harness.rs:5-10`).
//!
//! The rule is *two or more*, and the set that satisfies it is not fixed:
//! `tree.rs` became a caller when `wiring.rs` needed the scoped field query
//! too, which is why nothing here names a closed list of case files.

use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use goad::generated::{OptionRow, PromptWindow, Tray};
use goad::glass::SlintGlass;
use goad_semantics::protocol::canonical::Timestamp;
use goad_shell::clock::ClockError;
use goad_shell::config::Command as ShellCommand;
use i_slint_backend_testing::{ElementHandle, ElementQuery, init_no_event_loop};
use slint::{ComponentHandle, Model, VecModel};

use crate::driving::instant;
use crate::scripting::logging_backend;

pub(crate) const TIMEOUT: Duration = Duration::from_secs(2);

pub(crate) fn now() -> Timestamp {
  instant("2026-01-01T00:00:00Z")
}

/// A `Clock` (`fn() -> Result<Timestamp, ClockError>`) fixed to [`now`]. A
/// plain top-level `fn`, not a closure: `Clock` is a `fn` pointer type
/// (`goad_shell::clock`), the same reason `serve` itself takes one rather than a
/// `dyn Fn`. Shared by `mod serving`, `mod interaction` and `mod
/// cancellation`, all of which call `serve` directly.
#[expect(
  clippy::unnecessary_wraps,
  reason = "must match `Clock`'s `fn() -> Result<Timestamp, ClockError>` \
    signature to be passed to `serve`; a test fixture never needs to \
    exercise the error arm (test code, outside VA-3's src/-only budget)"
)]
pub(crate) fn stub_clock() -> Result<Timestamp, ClockError> {
  Ok(now())
}

pub(crate) fn window_and_tray() -> (PromptWindow, Tray) {
  init_no_event_loop();
  (
    PromptWindow::new().expect("a headless window must construct"),
    Tray::new().expect("a headless tray must construct"),
  )
}

pub(crate) fn glass_over(window: &PromptWindow, tray: &Tray) -> SlintGlass {
  SlintGlass::new(
    window.clone_strong(),
    tray.clone_strong(),
    Rc::new(VecModel::<OptionRow>::default()),
  )
}

/// The view token a real click would carry, read off the options model
/// exactly as `app.slint`'s `chosen` callback does (`option.view`) — so a
/// test builds a `Command::Choose` from what the window actually holds,
/// never from a second, independent minting of the same value (PHASE-10
/// repair, VT-4).
pub(crate) fn current_view_token(window: &PromptWindow) -> Option<String> {
  window
    .get_options()
    .row_data(0)
    .map(|row| row.view.to_string())
}

/// The option's own **control**, by the identity the tests select on (R-14):
/// never by label, which two options may share.
///
/// The description alone does not pick one element. The option's field
/// container answers to the same `option.id`, so that a field can be addressed
/// by a query scoped to its option ([`field_described`]), and `find_first`
/// would otherwise return whichever the walk reached first — declaration
/// order, which nothing pins. The type filter is what keeps this helper's
/// contract: a control, with a default action and an item index, and not the
/// group that surrounds it.
///
/// **No case in this target goes red without the filter, measured rather than
/// assumed:** deleting it leaves every case green, because the markup happens
/// to declare an option's control before its field container and the walk
/// reaches the control first anyway. That is the declaration order nothing
/// pins, and the filter is what keeps the helper from silently depending on
/// it. No case is written to pin it, because a case that cannot be made to
/// fail pins nothing.
///
/// Here rather than in one case file because several need it — to ask the
/// markup what it drew, to press the control a person presses, and to read a
/// property off the control that was found. Which files those are is the
/// module rule's business and is deliberately not listed
/// (`review-code.md` F-20).
pub(crate) fn element_described(window: &PromptWindow, description: &str) -> Option<ElementHandle> {
  ElementQuery::from_root(window)
    .match_inherits("Button")
    .match_predicate(described(description))
    .find_first()
}

/// Matches the elements carrying `description` as their accessible description.
///
/// A predicate rather than a query, because its callers scope it differently —
/// a type filter, a descendant walk from the root, a role filter and then a
/// descendant walk, and the predicate applied to a query that `within_option`
/// has *already* scoped. A helper that fused the rule with one of those scopes
/// is what forced the rest to restate it, and they did: the rule stood written
/// five times before it was pulled out here (`review-code.md` F-13, F-20).
///
/// It exists at all because `ElementQuery` has no accessible-description
/// matcher: `search_api.rs:232-287` lists its six builders and none of them
/// reads a description, so it is read through
/// `ElementHandle::accessible_description` inside a predicate.
pub(crate) fn described(description: &str) -> impl Fn(&ElementHandle) -> bool + 'static {
  let description = description.to_string();
  move |element| element.accessible_description().as_deref() == Some(description.as_str())
}

/// Everything under the option that answers to `option.id` — the scope every
/// field query starts from. [`described`] is the predicate; this helper is the
/// scope.
///
/// The option's control answers to `option.id` too, and is reached first; it
/// has no field beneath it, so it contributes no descendant here.
///
/// A **union of every match's descendants**, not a walk that stops at the
/// first: `ElementQuery` matches the predicate over all elements and descends
/// into all of them. Nothing here rests on which is reached first, and nothing
/// that does should be built on this helper — `tree.rs::labels_in_option` needs
/// the container *without* the control and scopes by role instead
/// (`review-code.md` F-6).
pub(crate) fn within_option(window: &PromptWindow, option: &str) -> ElementQuery {
  ElementQuery::from_root(window)
    .match_predicate(described(option))
    .match_descendants()
}

/// A field's identity is **scoped, not composite**: the option's container
/// carries `option.id` and the field's control carries `field.id`, so the
/// query descends from the one to the other. An unscoped `find_first` does not
/// carry over — an option id is unique within a view (R-14) but a field id
/// only within an option (R-52), so an unscoped query would take whichever
/// came first and report no ambiguity, which is exactly the case AC-4 exists
/// to prove. Joining the two into one description was rejected: ids are
/// backend-supplied strings whose characters no requirement constrains, so any
/// separator can occur inside one (design.md §5.2).
///
/// Here rather than in one case file because several need it — to ask the
/// markup what it draws, and to ask a presented window what the draft put on
/// it. Which files those are is the module rule's business and is deliberately
/// not listed (`review-code.md` F-20).
pub(crate) fn field_described(
  window: &PromptWindow,
  option: &str,
  field: &str,
) -> Option<ElementHandle> {
  within_option(window, option)
    .match_predicate(described(field))
    .find_first()
}

/// Like `driving::scripted`, but against `logs-the-request-then-answers.sh`
/// rather than `answers-as-instructed.sh`: the invocation log holds each raw
/// request rather than the literal string `invoked`, so a case can read back
/// what the host actually sent — `event.kind` for the scheduling cases, the
/// submitted `response.values` for the field ones. `answers-as-instructed.sh`
/// never reads its own stdin and cannot report what it received.
///
/// Here rather than in one case file because two of them need it:
/// `scheduling.rs` reads the kind of the request the loop's own timer
/// produced, and `fields.rs` reads what a person's answer carried. Not added
/// to `tests/support/driving.rs` — this target is still its only consumer.
///
/// **The log says an exchange *began*, not that it was absorbed.** The script
/// reads the request, appends it, and only then answers
/// (`tests/backends/logs-the-request-then-answers.sh:19`, `:26`, `:32`), and
/// the host folds the outcome in later still. A case that needs the fold to
/// have happened waits on something the production glass wrote;
/// `scheduling.rs`'s `absorbed_line` is the precedent.
pub(crate) fn logging_scripted(case: &str, instructions: &[&str]) -> (ShellCommand, PathBuf) {
  let (mut command, log) = logging_backend("logs-the-request-then-answers", case);
  command.arguments.extend(
    instructions
      .iter()
      .map(|instruction| (*instruction).to_owned()),
  );
  (command, log)
}

/// `waiting::within`, asserting: panics if `predicate` never becomes true
/// within `bound`. The shape every `serve`-driven test in this target needs
/// to observe an event the driving code does not control directly — an
/// invocation landing, a view arriving — rather than assume a fixed delay
/// covers it (PHASE-10 repair, VT-4/VT-10).
///
/// The poll itself is `tests/support/waiting.rs`'s, shared with the
/// event-loop tier; only the assertion is
/// this target's.
pub(crate) async fn until(bound: Duration, predicate: impl FnMut() -> bool) {
  assert!(
    crate::waiting::within(bound, predicate).await,
    "condition did not become true within {bound:?}"
  );
}
