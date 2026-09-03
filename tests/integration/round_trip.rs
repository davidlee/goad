//! The whole stack, against real backend processes — AC-7, AC-8 and AC-12.
//!
//! Everything here spawns. `host.rs` holds the same composition against a fake
//! and is where the host's own rules are stated case by case; this file is the
//! claim that they survive a fork, a pipe and a foreign runtime. Two backends
//! carry it, deliberately: one TypeScript and one bash, because a suite that
//! only ever runs deno cannot tell a transport that works for any configured
//! command from one that works for deno (AC-12).

use std::time::Duration;

use crate::fake::{Calls, FakeBackend, answering};
use crate::harness::{
  config, describe_outcome, example, host, instant, invocations, logging_backend, presented,
  prompting_event, quiet_event, state_error, stderr_of,
};
use goad::semantics::protocol::canonical::{Choice, Timestamp, UserResponse, View, ViewId};
use goad::shell::error::StateError;
use goad::shell::host::{Host, Outcome};

/// Long enough that a healthy exchange cannot flake, and short enough that a
/// hung one does not stall the suite. deno's own startup was measured at 15–20
/// ms and bash's is smaller, so this is three orders of magnitude of headroom.
const TIMEOUT: Duration = Duration::from_secs(5);

/// The instant every case starts from.
fn now() -> Timestamp {
  instant("2026-08-23T04:12:00Z")
}

/// When an answer is given — two minutes after the view was issued, because a
/// `respond` that shares the evaluate's instant would let a `next_check`
/// resolved from the wrong one pass unnoticed.
fn answered_at() -> Timestamp {
  instant("2026-08-23T04:14:00Z")
}

/// The choice a backend returned, or a diagnostic naming what came instead.
fn choice(outcome: &Outcome) -> &Choice {
  match &outcome.view {
    Some(presented) => {
      let View::Choice(choice) = &presented.view;
      choice
    }
    None => panic!("expected a view; got {}", describe_outcome(outcome)),
  }
}

/// An answer naming an option of the view just presented, with a value for
/// whichever field that option carried.
///
/// `OptionId` and `FieldId` have no public constructor (D30, I15), so an answer
/// is assembled out of the view it answers — which is what a renderer does.
fn answer_first_option(outcome: &Outcome) -> UserResponse {
  let option = choice(outcome)
    .options()
    .as_slice()
    .first()
    .expect("a choice carries at least one option");
  let values = option
    .fields()
    .as_slice()
    .iter()
    .map(|field| {
      (
        field.id().clone(),
        serde_json::json!("whatever the user typed"),
      )
    })
    .collect();
  UserResponse {
    option: option.id().clone(),
    values,
  }
}

// ---------------------------------------------------------------------------
// VT-1 — AC-7, against the deno example
// ---------------------------------------------------------------------------

/// The round trip, end to end: nothing to show, then a choice, then an answer
/// the backend accepts.
///
/// One `Host`, three exchanges, three processes. The `view_id` reaches the
/// caller inside `Presented` and goes back out in the `respond` without the
/// caller reaching into host state (F-23, AC-7).
#[tokio::test]
async fn the_deno_example_completes_a_round_trip() {
  let mut host = host(example(), TIMEOUT, now());

  // 1. The backend has nothing to say, and says so.
  let quiet = host.evaluate(now(), quiet_event(now())).await;
  assert!(quiet.failure.is_none(), "{}", describe_outcome(&quiet));
  assert!(quiet.view.is_none(), "{}", describe_outcome(&quiet));
  assert_eq!(quiet.next_check, instant("2026-08-23T04:57:00Z"));
  assert!(quiet.discarded.is_empty());

  // 2. The same backend, a different event, and now there is something to show.
  let prompted = host.evaluate(now(), prompting_event(now())).await;
  assert!(
    prompted.failure.is_none(),
    "{}",
    describe_outcome(&prompted)
  );
  assert_eq!(
    choice(&prompted).title(),
    "Fill in your interstitial journal?"
  );
  assert_eq!(choice(&prompted).options().as_slice().len(), 2);
  assert_eq!(prompted.next_check, instant("2026-08-23T04:57:00Z"));
  assert!(prompted.discarded.is_empty());

  // 3. The answer carries the id the caller was handed, and the backend takes it.
  let view_id = presented(&prompted).clone();
  let answered = host
    .respond(
      answered_at(),
      view_id.clone(),
      answer_first_option(&prompted),
    )
    .await;
  assert!(
    answered.failure.is_none(),
    "{}",
    describe_outcome(&answered)
  );
  assert!(answered.view.is_none(), "{}", describe_outcome(&answered));
  assert_eq!(answered.next_check, instant("2026-08-23T05:14:00Z"));

  // The example reports what it was asked on stderr, which is the one witness
  // to the round trip that is not the host describing itself: the id the caller
  // was given is the id the backend saw.
  assert_eq!(
    stderr_of(&answered).trim(),
    format!("answered {} with yes", view_id.as_str())
  );

  // And the interaction is closed — the same answer a second time has nothing
  // to answer (R-33's `view: null` on a `respond`).
  let again = host
    .respond(answered_at(), view_id, answer_first_option(&prompted))
    .await;
  assert!(matches!(
    state_error(&again),
    StateError::NoOutstandingView { .. }
  ));
}

// ---------------------------------------------------------------------------
// VT-3 — AC-12, the same round trip in bash
// ---------------------------------------------------------------------------

/// A backend that is not TypeScript completes the identical sequence.
///
/// It is invoked as `["bash", "<script>"]` with no shebang and no executable
/// bit, which is R-36's argv rule doing the work: nothing interposes a shell,
/// so nothing needs quoting and `bash` is simply argv[0].
#[tokio::test]
async fn the_bash_backend_completes_the_same_round_trip() {
  let (command, log) = logging_backend("answers-a-round-trip", "bash-round-trip");
  let mut host = host(command, TIMEOUT, now());

  let quiet = host.evaluate(now(), quiet_event(now())).await;
  assert!(quiet.failure.is_none(), "{}", describe_outcome(&quiet));
  assert!(quiet.view.is_none(), "{}", describe_outcome(&quiet));
  assert_eq!(quiet.next_check, instant("2026-08-23T04:57:00Z"));

  let prompted = host.evaluate(now(), prompting_event(now())).await;
  assert!(
    prompted.failure.is_none(),
    "{}",
    describe_outcome(&prompted)
  );
  assert_eq!(choice(&prompted).title(), "Log the last hour?");

  let answered = host
    .respond(
      answered_at(),
      presented(&prompted).clone(),
      answer_first_option(&prompted),
    )
    .await;
  assert!(
    answered.failure.is_none(),
    "{}",
    describe_outcome(&answered)
  );
  assert!(answered.view.is_none(), "{}", describe_outcome(&answered));
  assert_eq!(answered.next_check, instant("2026-08-23T05:14:00Z"));

  // The script echoes back what it read, so this is the request arriving
  // verbatim rather than merely something arriving.
  assert!(
    stderr_of(&answered).contains(r#""type":"respond""#),
    "the backend did not report the request it read: {}",
    stderr_of(&answered).escape_debug()
  );
  assert_eq!(
    invocations(&log),
    3,
    "one process per exchange, and no more"
  );
}

// ---------------------------------------------------------------------------
// VT-2 — AC-8 through the real transport
// ---------------------------------------------------------------------------

/// An answer against an idle host is refused, and no process is spawned.
///
/// The backend here is runnable, and that is the point. The first draft of this
/// case pointed the config at a program that does not exist, on the plan's own
/// suggestion — "a backend that would fail if it ran" — and it was **vacuous**:
/// a host that spawns first and refuses afterwards still returns the refusal,
/// so the `Spawn` failure never reaches the caller and the case stayed green
/// under both of the breaks that reorder the check (recorded in the phase
/// sheet). The invocation log is the discriminator, because it is the one
/// question the host is not the one answering.
#[tokio::test]
async fn an_answer_no_view_asked_for_never_reaches_the_backend() {
  let (command, log) = logging_backend("answers-a-round-trip", "idle-host");
  let mut host = host(command, TIMEOUT, now());

  let refused = host
    .respond(
      now(),
      ViewId::new("2026-08-23T04:12:00Z#0"),
      foreign().await,
    )
    .await;

  assert!(
    matches!(state_error(&refused), StateError::NoOutstandingView { .. }),
    "{}",
    describe_outcome(&refused)
  );
  assert_eq!(
    invocations(&log),
    0,
    "an answer nothing asked for must not reach the backend"
  );
  // Nothing ran, so there is nothing to have captured or disposed of.
  assert_eq!(stderr_of(&refused), "");
  assert!(refused.cleanup.is_none());
  // The failure did not move the schedule (R-29): still the seeded default.
  assert_eq!(refused.next_check, instant("2026-08-23T04:42:00Z"));
}

/// A superseded answer never reaches the backend either, and here the backend
/// *is* runnable — so the evidence is a file it appends to on every invocation,
/// which the host cannot answer for.
///
/// The positive control is the accepted answer at the end. Without it the case
/// would pass against a witness that never moves at all, which is the mistake
/// PHASE-06 made three times.
#[tokio::test]
async fn a_superseded_answer_never_reaches_the_backend() {
  let (command, log) = logging_backend("answers-a-round-trip", "superseded");
  let mut host = host(command, TIMEOUT, now());

  let first = host.evaluate(now(), prompting_event(now())).await;
  let stale = presented(&first).clone();
  let second = host.evaluate(now(), prompting_event(now())).await;
  let live = presented(&second).clone();
  assert_ne!(stale, live, "the second view must supersede the first");
  assert_eq!(invocations(&log), 2);

  let refused = host
    .respond(answered_at(), stale, answer_first_option(&first))
    .await;

  assert!(
    matches!(state_error(&refused), StateError::StaleViewId { .. }),
    "{}",
    describe_outcome(&refused)
  );
  assert_eq!(
    invocations(&log),
    2,
    "a stale answer must not reach the backend"
  );

  // The positive control, and R-34: the refusal left the live interaction alone.
  let answered = host
    .respond(answered_at(), live, answer_first_option(&second))
    .await;
  assert!(
    answered.failure.is_none(),
    "{}",
    describe_outcome(&answered)
  );
  assert_eq!(
    invocations(&log),
    3,
    "an accepted answer does reach the backend"
  );
}

// ---------------------------------------------------------------------------
// VT-5 — R-35: the host validates the `view_id` and nothing else
// ---------------------------------------------------------------------------

/// An answer naming an option the view did not offer, and a field value under a
/// field it did not carry, reaches the backend unchanged.
///
/// D17 and R-35: whether an answer is acceptable is the backend's judgement, so
/// the host forwards it and accepts what comes back. A host that filtered here
/// would make itself the arbiter of a vocabulary it does not own.
#[tokio::test]
async fn an_answer_the_view_did_not_offer_reaches_the_backend_unchanged() {
  let (command, _log) = logging_backend("answers-a-round-trip", "unoffered-answer");
  let mut host = host(command, TIMEOUT, now());
  let presented_view = host.evaluate(now(), prompting_event(now())).await;

  let answered = host
    .respond(
      answered_at(),
      presented(&presented_view).clone(),
      foreign().await,
    )
    .await;

  assert!(
    answered.failure.is_none(),
    "{}",
    describe_outcome(&answered)
  );
  let seen = stderr_of(&answered);
  assert!(
    seen.contains(r#""option":"an-option-no-view-offered""#),
    "the option did not reach the backend: {}",
    seen.escape_debug()
  );
  assert!(
    seen.contains(r#""a-field-no-option-offered":"whatever the user typed""#),
    "the field value did not reach the backend: {}",
    seen.escape_debug()
  );
}

/// An answer whose ids came from a view no process ever returned.
///
/// `OptionId` and `FieldId` are not publicly constructible (D30), so even an
/// answer that names nothing real has to be harvested from *some* view. The
/// fake is the cheapest one: it costs no spawn, and the ids can be spelled to
/// say what they are.
async fn foreign() -> UserResponse {
  const A_VIEW_NOBODY_OFFERED: &[u8] = br#"{"view":{"kind":"choice","title":"T","options":[{"id":"an-option-no-view-offered","label":"L","fields":[{"id":"a-field-no-option-offered","kind":"text","label":"L"}]}]}}"#;
  let calls = Calls::default();
  let backend = FakeBackend::new(vec![answering(A_VIEW_NOBODY_OFFERED)], &calls);
  let mut fake_host = Host::new(
    config(vec!["never-spawned".to_owned()], TIMEOUT),
    backend,
    now(),
  );
  answer_first_option(&fake_host.evaluate(now(), quiet_event(now())).await)
}
