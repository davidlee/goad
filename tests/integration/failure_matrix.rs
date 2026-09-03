//! Every failure mode `design.md` §9 names, as the `Outcome` a caller receives.
//!
//! The protocol tier already proves normalization refuses these bytes, over the
//! same bodies — `tests/protocol/fixtures/protocol/` is where each one below was
//! copied from, and each case names its fixture. That is not the same claim:
//! there the input is a `serde_json::Value` handed straight to
//! `normalize_response`, and here it is written by a foreign process, read
//! through a pipe, parsed at `host::read`, and delivered as a `Failure` on an
//! `Outcome`. This tier proves the refusal survives the journey.
//!
//! The bodies are Rust literals rather than the fixture files themselves. A
//! fixture is an envelope with the body nested under `input`, and bash has no
//! JSON parser to lift it out; reading the file whole would also couple the two
//! tiers, and they are worth more making the same claim independently.

use std::time::Duration;

use crate::harness::{
  answer_first_option, backend_error, describe_outcome, host, instant, invocations, only_discard,
  presented, protocol_error, quiet_event, scripted, stderr_of,
};
use goad::semantics::error::{BoundsError, ProtocolError, ScheduleError};
use goad::semantics::protocol::canonical::Timestamp;
use goad::semantics::protocol::normalize::Discarded;
use goad::shell::error::BackendError;
use goad::shell::host::Outcome;

/// Long enough that a healthy exchange cannot flake. Nothing here is timing the
/// backend; the transport tier owns that.
const TIMEOUT: Duration = Duration::from_secs(5);

/// The instant every case starts from.
fn now() -> Timestamp {
  instant("2026-08-23T04:12:00Z")
}

/// What the host is seeded with — `now` plus the default poll.
///
/// Only the spawn case reads it. Everything else asserts the schedule against
/// `accepted_check`, for the reason `instructed` gives: this instant is also
/// what a host that re-resolved on failure would report, so it cannot carry
/// R-29 on its own.
fn seeded_check() -> Timestamp {
  instant("2026-08-23T04:42:00Z")
}

/// When VT-2's answer is given — two minutes after the view was issued, because
/// a `respond` that shared the evaluate's instant would let a `next_check`
/// resolved from the wrong one pass unnoticed.
fn answered_at() -> Timestamp {
  instant("2026-08-23T04:14:00Z")
}

/// One exchange against a backend told to answer with exactly `body` — after a
/// well-behaved one that moves the schedule off its seed.
///
/// **The first exchange is what makes the schedule assertions mean anything.**
/// The seeded check is `now` plus the default poll, and so is what a host that
/// re-resolved the schedule on a failure would report — the two are the same
/// instant, so a case starting from the seed cannot tell "unchanged" from
/// "recomputed" (found by breaking it; recorded in the phase sheet). Moving the
/// check to `accepted_check` first separates them.
///
/// `case` names the invocation log, so concurrent cases cannot collide.
async fn answered_with(case: &str, body: &str) -> Outcome {
  instructed(case, body, TIMEOUT).await
}

/// The same for an instruction that needs its own timeout — the hang wants a
/// short one so the suite stays fast, and the flood wants room to reach the
/// bound before the deadline does.
async fn instructed(case: &str, instruction: &str, timeout: Duration) -> Outcome {
  let (command, _log) = scripted(case, &[WELL_BEHAVED, instruction]);
  let mut host = host(command, timeout, now());

  let moved = host.evaluate(now(), quiet_event(now())).await;
  assert_eq!(
    moved.next_check,
    accepted_check(),
    "the case never got a schedule to leave alone: {}",
    describe_outcome(&moved)
  );

  host.evaluate(now(), quiet_event(now())).await
}

// ---------------------------------------------------------------------------
// The bodies, and where each came from
// ---------------------------------------------------------------------------
//
// Named rather than inlined at the assertion because VT-2 runs all thirteen
// through one `Host` and a second copy of a body is a place for the two claims
// to drift apart. Each is the `input` of the fixture named above it, verbatim.

/// `R-3-protocol-declared-as-a-version-the-host-does-not-implement.json`
const UNSUPPORTED_VERSION: &str = r#"{"protocol":2,"view":null}"#;

/// `R-13-a-choice-with-no-options.json`
const NO_OPTIONS: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[]}}"#;

/// `R-14-duplicate-option-ids.json`
const DUPLICATE_OPTION_IDS: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"later","label":"Ask me later"},{"id":"later","label":"Not now"}]}}"#;

/// `R-52-duplicate-field-ids-within-one-option.json`
const DUPLICATE_FIELD_IDS: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine","fields":[{"id":"note","kind":"text","label":"A"},{"id":"note","kind":"text","label":"B"}]}]}}"#;

/// `R-12-an-unknown-field-kind.json`
const UNKNOWN_NESTED_KIND: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine"},{"id":"no","label":"Badly","fields":[{"id":"a","kind":"text","label":"A"},{"id":"b","kind":"text","label":"B"},{"id":"c","kind":"slider","label":"C"}]}]}}"#;

/// `R-10-view-omitted.json`
const VIEW_OMITTED: &str = r#"{"next_check":"45 minutes"}"#;

/// `R-17-inverted-bounds.json`
const INVERTED_BOUNDS: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine","fields":[{"id":"f","kind":"number","label":"L","min":10,"max":1}]}]}}"#;

/// `R-50-min-on-a-text-field.json`
const MIN_ON_A_TEXT_FIELD: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine","fields":[{"id":"f","kind":"text","label":"L","min":1}]}]}}"#;

/// `R-50-options-on-a-number-field.json`
const OPTIONS_ON_A_NUMBER_FIELD: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine","fields":[{"id":"f","kind":"number","label":"L","options":[{"id":"red","label":"Red"}]}]}]}}"#;

/// `R-25-next-check-of-the-wrong-type.json`
const NEXT_CHECK_OF_THE_WRONG_TYPE: &str = r#"{"view":null,"next_check":45}"#;

/// `schedule/R-23-calendar-unit-months.json`
const NEXT_CHECK_IN_CALENDAR_UNITS: &str = r#"{"view":null,"next_check":"1 month"}"#;

/// `R-51-next-check-null.json`
const NEXT_CHECK_NULL: &str = r#"{"view":null,"next_check":null}"#;

/// `R-51-protocol-null.json`
const PROTOCOL_NULL: &str = r#"{"protocol":null,"view":null}"#;

/// The thirteen, in the order VT-2 sends them.
const PROTOCOL_MODES: [&str; 13] = [
  UNSUPPORTED_VERSION,
  NO_OPTIONS,
  DUPLICATE_OPTION_IDS,
  DUPLICATE_FIELD_IDS,
  UNKNOWN_NESTED_KIND,
  VIEW_OMITTED,
  INVERTED_BOUNDS,
  MIN_ON_A_TEXT_FIELD,
  OPTIONS_ON_A_NUMBER_FIELD,
  NEXT_CHECK_OF_THE_WRONG_TYPE,
  NEXT_CHECK_IN_CALENDAR_UNITS,
  NEXT_CHECK_NULL,
  PROTOCOL_NULL,
];

/// The four sentinels `answers-as-instructed.sh` honours: a timeout, an output
/// flood, a valid answer disclaimed by a non-zero exit, and a clean exit with a
/// body that will not parse. Each is the behaviour of a script in
/// `tests/backends/`, and the transport tier asserts each of them there as a
/// `BackendError`; what is asserted here is the `Outcome` a caller receives.
const TRANSPORT_MODES: [&str; 4] = ["@hang", "@flood", "@exit1", "@garbage"];

/// A body the host accepts, and one that moves the schedule so a success is
/// distinguishable from a failure that left it alone.
const WELL_BEHAVED: &str = r#"{"view":null,"next_check":"45 minutes"}"#;

/// The same, carrying a view — so the host has an outstanding interaction to
/// lose, and VT-2 can tell one `Host` from eighteen of them.
const PRESENTS_A_VIEW: &str = r#"{"view":{"kind":"choice","title":"Still here?","options":[{"id":"yes","label":"Yes"}]},"next_check":"45 minutes"}"#;

/// What the backend answers the surviving view with: taken, nothing further to
/// show, and a schedule 90 minutes out that no other body in this file sets.
const ACCEPTS_THE_ANSWER: &str = r#"{"view":null,"next_check":"90 minutes"}"#;

/// Where `WELL_BEHAVED` puts the next check: `now` plus 45 minutes.
fn accepted_check() -> Timestamp {
  instant("2026-08-23T04:57:00Z")
}

// ---------------------------------------------------------------------------
// VT-1 — the protocol-level modes, each to its own variant
// ---------------------------------------------------------------------------

/// `R-3-protocol-declared-as-a-version-the-host-does-not-implement.json`.
#[tokio::test]
async fn a_protocol_version_the_host_does_not_implement_is_refused() {
  let outcome = answered_with("unsupported-version", UNSUPPORTED_VERSION).await;
  assert!(
    matches!(
      protocol_error(&outcome),
      ProtocolError::UnsupportedProtocolVersion { found: 2 }
    ),
    "{}",
    describe_outcome(&outcome)
  );
  assert_eq!(outcome.next_check, accepted_check());
}

/// `R-13-a-choice-with-no-options.json`.
#[tokio::test]
async fn a_choice_with_no_options_is_refused() {
  let outcome = answered_with("empty-options", NO_OPTIONS).await;
  assert!(
    matches!(
      protocol_error(&outcome),
      ProtocolError::EmptyOptions { at } if at == "view.options"
    ),
    "{}",
    describe_outcome(&outcome)
  );
}

/// `R-14-duplicate-option-ids.json`.
#[tokio::test]
async fn two_options_sharing_an_id_are_refused() {
  let outcome = answered_with("duplicate-option-ids", DUPLICATE_OPTION_IDS).await;
  assert!(
    matches!(
      protocol_error(&outcome),
      ProtocolError::DuplicateOptionId { id, at } if id == "later" && at == "view.options"
    ),
    "{}",
    describe_outcome(&outcome)
  );
}

/// `R-52-duplicate-field-ids-within-one-option.json`.
#[tokio::test]
async fn two_fields_in_one_option_sharing_an_id_are_refused() {
  let outcome = answered_with("duplicate-field-ids", DUPLICATE_FIELD_IDS).await;
  assert!(
    matches!(
      protocol_error(&outcome),
      ProtocolError::DuplicateFieldId { id, at }
        if id == "note" && at == "view.options[0].fields"
    ),
    "{}",
    describe_outcome(&outcome)
  );
}

/// `R-12-an-unknown-field-kind.json` — the path is the point (F-6): the kind is
/// nested two levels down, and naming it without naming the place is a puzzle.
#[tokio::test]
async fn an_unknown_kind_nested_in_a_field_is_refused_with_its_path() {
  let outcome = answered_with("unknown-nested-kind", UNKNOWN_NESTED_KIND).await;
  assert!(
    matches!(
      protocol_error(&outcome),
      ProtocolError::UnsupportedPrimitive { kind, at }
        if kind == "slider" && at == "view.options[1].fields[2].kind"
    ),
    "{}",
    describe_outcome(&outcome)
  );
}

/// `R-10-view-omitted.json` — the message does not survive losing its view, and
/// the `next_check` beside it does not save it.
#[tokio::test]
async fn a_response_omitting_view_is_refused() {
  let outcome = answered_with("view-omitted", VIEW_OMITTED).await;
  assert!(
    matches!(
      protocol_error(&outcome),
      ProtocolError::MissingField { field: "view" }
    ),
    "{}",
    describe_outcome(&outcome)
  );
  assert_eq!(outcome.next_check, accepted_check());
}

/// `R-17-inverted-bounds.json` — bounds are semantics, so an unusable range
/// costs the message rather than being discarded (P2).
#[tokio::test]
async fn inverted_bounds_are_refused() {
  let outcome = answered_with("inverted-bounds", INVERTED_BOUNDS).await;
  assert!(
    matches!(
      protocol_error(&outcome),
      ProtocolError::Bounds(BoundsError::Inverted { min, max })
        if (*min - 10.0).abs() < f64::EPSILON && (*max - 1.0).abs() < f64::EPSILON
    ),
    "{}",
    describe_outcome(&outcome)
  );
}

/// `R-50-min-on-a-text-field.json` — a modelled key the kind does not admit is
/// rejected rather than dropped, because serde consumes it before `kind` is
/// dispatched and ignoring it would lose it silently (D43).
#[tokio::test]
async fn a_text_field_carrying_min_is_refused() {
  let outcome = answered_with("min-on-a-text-field", MIN_ON_A_TEXT_FIELD).await;
  assert!(
    matches!(
      protocol_error(&outcome),
      ProtocolError::InapplicableKey { key: "min", kind, at }
        if kind == "text" && at == "view.options[0].fields[0]"
    ),
    "{}",
    describe_outcome(&outcome)
  );
}

/// `R-50-options-on-a-number-field.json` — the same rule, the other way round.
#[tokio::test]
async fn a_number_field_carrying_options_is_refused() {
  let outcome = answered_with("options-on-a-number-field", OPTIONS_ON_A_NUMBER_FIELD).await;
  assert!(
    matches!(
      protocol_error(&outcome),
      ProtocolError::InapplicableKey { key: "options", kind, at }
        if kind == "number" && at == "view.options[0].fields[0]"
    ),
    "{}",
    describe_outcome(&outcome)
  );
}

// The two scheduling modes are **not** failures. An unusable `next_check` is a
// discard on an otherwise accepted message (P2, R-25), so these cases assert
// there is no failure at all and then read `discarded`. A test that expected an
// `Err` here would be asserting the opposite of the design.

/// `R-25-next-check-of-the-wrong-type.json`.
#[tokio::test]
async fn a_next_check_of_the_wrong_type_is_discarded_and_reported() {
  let outcome = answered_with("next-check-wrong-type", NEXT_CHECK_OF_THE_WRONG_TYPE).await;
  assert!(outcome.failure.is_none(), "{}", describe_outcome(&outcome));
  let Discarded::Schedule { raw, reason } = only_discard(&outcome);
  assert_eq!(raw, &serde_json::json!(45));
  assert!(
    matches!(reason, ScheduleError::NotAString { found: "number" }),
    "{reason}"
  );
  assert_eq!(outcome.next_check, accepted_check());
}

/// `schedule/R-23-calendar-unit-months.json` — a calendar unit has no fixed
/// length, so the host cannot resolve it to an instant.
#[tokio::test]
async fn a_next_check_in_calendar_units_is_discarded_and_reported() {
  let outcome = answered_with("next-check-calendar-unit", NEXT_CHECK_IN_CALENDAR_UNITS).await;
  assert!(outcome.failure.is_none(), "{}", describe_outcome(&outcome));
  let Discarded::Schedule { raw, reason } = only_discard(&outcome);
  assert_eq!(raw, &serde_json::json!("1 month"));
  assert!(
    matches!(reason, ScheduleError::CalendarUnit { raw: unit } if unit == "1 month"),
    "{reason}"
  );
  assert_eq!(outcome.next_check, accepted_check());
}

// And the two `null`s, which are the control for both of the above: an explicit
// `null` means what omission means, so nothing is discarded and nothing is
// reported (R-51, D50). Serializers in common backend languages emit `null` for
// an absent optional as ordinary output, so a host that reported these would
// report on well-behaved backends.

/// `R-51-next-check-null.json`.
#[tokio::test]
async fn an_explicit_null_next_check_discards_nothing() {
  let outcome = answered_with("next-check-null", NEXT_CHECK_NULL).await;
  assert!(outcome.failure.is_none(), "{}", describe_outcome(&outcome));
  assert!(
    outcome.discarded.is_empty(),
    "an explicit null must not be reported as a discard"
  );
  assert_eq!(outcome.next_check, accepted_check());
}

/// `R-51-protocol-null.json`.
#[tokio::test]
async fn an_explicit_null_protocol_discards_nothing() {
  let outcome = answered_with("protocol-null", PROTOCOL_NULL).await;
  assert!(outcome.failure.is_none(), "{}", describe_outcome(&outcome));
  assert!(
    outcome.discarded.is_empty(),
    "an explicit null must not be reported as a discard"
  );
  assert_eq!(outcome.next_check, accepted_check());
}

/// The positive control for the twenty-odd assertions above: this backend is
/// well-behaved, so the same machinery that reports each refusal reports a
/// success, and the schedule the backend asked for is what the caller is told.
/// Without it every case above could pass against a host that refused
/// everything.
#[tokio::test]
async fn the_same_backend_told_to_behave_is_accepted() {
  let outcome = answered_with("well-behaved", ACCEPTS_THE_ANSWER).await;
  assert!(outcome.failure.is_none(), "{}", describe_outcome(&outcome));
  assert!(outcome.discarded.is_empty());
  // 90 minutes on from `now`, and no other body in this file resolves here — so
  // this is the schedule moving *again*, not the first exchange's value read
  // back.
  assert_eq!(outcome.next_check, instant("2026-08-23T05:42:00Z"));
}

// ---------------------------------------------------------------------------
// VT-3 — the transport and lifecycle modes, as the caller receives them
// ---------------------------------------------------------------------------
//
// `transport.rs` asserts each of these as a `BackendError` returned by
// `ProcessBackend`. That is a different subject: here the question is what a
// caller of `Host` is handed, which is a `Failure` on an `Outcome` beside a
// `next_check` that a failure may not have moved (R-29, EX-3). Three of the
// five below — the timeout, the non-zero exit and the malformed body — are
// EX-3's own list, and `host.rs:298` makes that claim against a fake.
//
// The two remaining EX-4 modes, a stale and an unknown `view_id`, are already
// asserted through a `Host` over the real transport at `round_trip.rs:220` and
// `:257`, with an invocation-log witness that no process was spawned. Nothing
// here repeats them.

/// A command that is not there. No fixture and no script: a path that does not
/// exist is the whole case.
#[tokio::test]
async fn a_command_that_cannot_be_spawned_reaches_the_caller_as_a_spawn_failure() {
  let mut host = host(
    vec!["/nonexistent/goad-has-no-such-backend".to_owned()],
    TIMEOUT,
    now(),
  );

  let outcome = host.evaluate(now(), quiet_event(now())).await;

  assert!(
    matches!(backend_error(&outcome), BackendError::Spawn(_)),
    "{}",
    describe_outcome(&outcome)
  );
  // The seed, and only the seed: a host whose command does not exist cannot
  // first succeed, so this case cannot make the stronger R-29 claim the others
  // do. EX-3 does not ask it to.
  assert_eq!(outcome.next_check, seeded_check());
}

/// A backend that never answers.
#[tokio::test]
async fn a_backend_that_never_answers_reaches_the_caller_as_a_timeout() {
  let deadline = Duration::from_millis(300);

  let outcome = instructed("timeout", "@hang", deadline).await;

  assert!(
    matches!(backend_error(&outcome), BackendError::Timeout { after } if *after == deadline),
    "{}",
    describe_outcome(&outcome)
  );
  assert_eq!(outcome.next_check, accepted_check());
}

/// A valid response the exit status then disclaims. The body parsed and is
/// discarded anyway — the backend told us it failed, and trusting output it
/// disclaimed would be the host deciding it knows better (D15, R-40, F-59).
#[tokio::test]
async fn a_non_zero_exit_reaches_the_caller_as_an_exit_status_with_the_body_discarded() {
  let outcome = answered_with("non-zero-exit", "@exit1").await;

  assert!(
    matches!(
      backend_error(&outcome),
      BackendError::ExitStatus { code: Some(1) }
    ),
    "{}",
    describe_outcome(&outcome)
  );
  assert!(outcome.view.is_none(), "{}", describe_outcome(&outcome));
  assert!(
    stderr_of(&outcome).contains("that answer is not to be trusted"),
    "the diagnostic did not survive: {}",
    stderr_of(&outcome).escape_debug()
  );
  assert_eq!(outcome.next_check, accepted_check());
}

/// A clean exit and a body that will not parse. The stderr is often the only
/// explanation there is, which is why it travels beside the failure rather than
/// inside it (R-42, F-24).
#[tokio::test]
async fn a_body_that_will_not_parse_reaches_the_caller_as_a_protocol_failure() {
  let outcome = answered_with("malformed-stdout", "@garbage").await;

  assert!(
    matches!(protocol_error(&outcome), ProtocolError::Json(_)),
    "{}",
    describe_outcome(&outcome)
  );
  assert!(
    stderr_of(&outcome).contains("config is missing"),
    "the diagnostic did not survive: {}",
    stderr_of(&outcome).escape_debug()
  );
  assert_eq!(outcome.next_check, accepted_check());
}

/// A backend that will not stop writing. The bound's *value* is the transport's
/// claim and `transport.rs` asserts it; what reaches a caller is the variant.
#[tokio::test]
async fn output_past_the_cap_reaches_the_caller_as_output_too_large() {
  let outcome = instructed("flood", "@flood", TIMEOUT).await;

  assert!(
    matches!(backend_error(&outcome), BackendError::OutputTooLarge { .. }),
    "{}",
    describe_outcome(&outcome)
  );
  assert_eq!(outcome.next_check, accepted_check());
}

// ---------------------------------------------------------------------------
// VT-2 — R-45: one `Host`, every misbehaviour, and an exchange after them all
// ---------------------------------------------------------------------------

/// A view, then seventeen misbehaving exchanges, then the answer to that view —
/// all through a single `Host`.
///
/// R-45: no backend failure may leave the host unable to invoke the backend
/// again. The transport modes are here as well as the protocol ones by user
/// decision (this phase's sheet) — a protocol refusal never touches a process
/// lifecycle, so a suite of those alone would assert the easy half of a claim
/// about surviving process failure.
///
/// **The first exchange is what makes this a test of reuse.** The plan warns
/// that a fresh `Host` per case satisfies EX-2 by construction; asserting the
/// last exchange succeeds does not catch that, because a fresh host succeeds
/// too. So the sequence begins by putting a view outstanding and moving the
/// schedule, and both are then read back **after** the seventeen failures: the
/// schedule each failure did not move (R-29), and at the end an answer to the
/// view none of them closed (R-34). A fresh host has neither — it would report
/// the seeded check and refuse the answer with `NoOutstandingView`.
#[tokio::test]
async fn one_host_survives_every_misbehaving_backend_and_still_works() {
  // Short, because one instruction is a hang and the suite pays for it once.
  // The flood reaches the 8 MiB bound in single-digit milliseconds (measured at
  // PHASE-06), so it has room.
  let deadline = Duration::from_millis(500);
  let instructions: Vec<&str> = std::iter::once(PRESENTS_A_VIEW)
    .chain(PROTOCOL_MODES)
    .chain(TRANSPORT_MODES)
    .chain(std::iter::once(ACCEPTS_THE_ANSWER))
    .collect();
  let (command, log) = scripted("the-whole-suite", &instructions);
  let mut host = host(command, deadline, now());

  let prompted = host.evaluate(now(), quiet_event(now())).await;
  assert!(
    prompted.failure.is_none(),
    "{}",
    describe_outcome(&prompted)
  );
  let outstanding = presented(&prompted).clone();
  assert_eq!(prompted.next_check, accepted_check());

  let mut refused: usize = 0;
  for instruction in PROTOCOL_MODES.iter().chain(TRANSPORT_MODES.iter()) {
    let outcome = host.evaluate(now(), quiet_event(now())).await;
    if outcome.failure.is_some() {
      refused = refused.saturating_add(1);
    }
    // R-29, once per mode — and the reuse witness: this is the check the *first*
    // exchange asked for, so a host reconstructed anywhere in this loop reports
    // the seeded one instead. The four instructions that are accepted are
    // accepted *without* a usable `next_check`, so the assertion holds for every
    // one of the seventeen rather than only for the failures.
    assert_eq!(
      outcome.next_check,
      accepted_check(),
      "`{instruction}` moved the schedule"
    );
  }

  // Thirteen of the seventeen are refusals; the other four are the two discards
  // and the two nulls, which are accepted messages. Counting them is what stops
  // this case passing against a host that started refusing everything — or
  // accepting everything — halfway through.
  assert_eq!(refused, 13, "the wrong number of exchanges failed");

  // The answer to the view issued before all of it. No failure closed the
  // interaction (R-34), and the host can still invoke a backend (R-45).
  let answered = host
    .respond(answered_at(), outstanding, answer_first_option(&prompted))
    .await;

  assert!(
    answered.failure.is_none(),
    "the host could not complete an exchange after the suite: {}",
    describe_outcome(&answered)
  );
  assert_eq!(
    answered.next_check,
    instant("2026-08-23T05:44:00Z"),
    "the exchange succeeded but the schedule did not move, so success is indistinguishable from another failure"
  );
  assert_eq!(
    invocations(&log),
    instructions.len(),
    "one process per exchange, and no more"
  );
}
