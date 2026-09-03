//! The host as a caller sees it — `design.md` §5.2's Host block and §5.4's
//! sequence, against the fake backend rather than a process.
//!
//! Everything here is about composition: what a caller is handed, what the host
//! keeps, and what it refuses. The transport's own behaviour is `transport.rs`'s
//! subject and is not restated.

use std::collections::BTreeMap;

use crate::fake::{Calls, FakeBackend, answering, failing, failing_noisily};
use crate::harness::{backend_error, describe_outcome, instant, presented, state_error};
use goad::semantics::protocol::canonical::{Event, Timestamp, UserResponse, View, ViewId};
use goad::shell::config::Config;
use goad::shell::error::{BackendError, CleanupFailure, StateError};
use goad::shell::host::Host;

/// The design's own example, minus the command — no process is ever spawned
/// here, and a command that could not run would be a misleading fixture.
const CONFIG: &str = r#"
[backend]
command = ["bash", "-c", "false"]
timeout = "5s"

[schedule]
default_poll = "30m"
"#;

/// The `now` every case starts from, and the seeded check that follows from it:
/// `04:12:00Z` plus the 30-minute default poll.
fn now() -> Timestamp {
  instant("2026-08-23T04:12:00Z")
}

fn seeded_check() -> Timestamp {
  instant("2026-08-23T04:42:00Z")
}

fn host(scripted: Vec<goad::shell::backend::transport::Exchange>) -> (Host<FakeBackend>, Calls) {
  let calls = Calls::default();
  let config = Config::parse(CONFIG).expect("the fixture config must load");
  let backend = FakeBackend::new(scripted, &calls);
  (Host::new(config, backend, now()), calls)
}

fn event() -> Event {
  Event {
    source: "test".to_owned(),
    kind: "poll".to_owned(),
    timestamp: now(),
    data: serde_json::Value::Null,
  }
}

/// An answer naming an option some view offered.
///
/// `OptionId` is deliberately not publicly constructible (D30): a caller
/// answering a view clones the one the view carries, so obtaining one means
/// going through a view — including for the case below whose host has nothing
/// outstanding at all. Which view it came from is immaterial here, because the
/// host validates the `view_id` and nothing else (R-35); PHASE-08/VT-5 is where
/// that is asserted rather than relied on.
async fn an_answer() -> UserResponse {
  let (mut host, _calls) = host(vec![answering(A_CHOICE)]);
  let outcome = host.evaluate(now(), event()).await;
  let View::Choice(choice) = &outcome.view.as_ref().expect("a view").view;
  UserResponse {
    option: choice
      .options()
      .as_slice()
      .first()
      .expect("a choice carries at least one option")
      .id()
      .clone(),
    values: BTreeMap::new(),
  }
}

/// A response body carrying one choice view.
const A_CHOICE: &[u8] =
  br#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine"}]}}"#;

// The describers this file shares with `round_trip.rs` — `instant`,
// `describe_outcome`, `presented`, `backend_error` and `state_error` — live in
// `harness.rs`. They were written here first and moved when the second file
// needed them, which is the refactor step doing its job rather than a second
// copy appearing.

// ---------------------------------------------------------------------------
// EX-4 — what a caller is handed
// ---------------------------------------------------------------------------

/// A view reaches the caller **with** the id that answers it, and the schedule
/// the backend asked for is what the caller is told to wake at (I14, D32, I12).
#[tokio::test]
async fn a_returned_view_arrives_with_its_id_and_a_concrete_next_check() {
  let body = br#"{"view":{"kind":"choice","title":"T","options":[{"id":"ok","label":"Fine"}]},"next_check":"1h"}"#;
  let (mut host, _calls) = host(vec![answering(body)]);

  let outcome = host.evaluate(now(), event()).await;

  assert!(outcome.failure.is_none(), "{}", describe_outcome(&outcome));
  assert_eq!(presented(&outcome).as_str(), "2026-08-23T04:12:00Z#0");
  let View::Choice(choice) = &outcome.view.as_ref().expect("a view").view;
  assert_eq!(choice.title(), "T");
  assert_eq!(outcome.next_check, instant("2026-08-23T05:12:00Z"));
  assert!(outcome.discarded.is_empty());
  assert!(outcome.cleanup.is_none());
}

/// A message can lose its scheduling instruction without losing the message,
/// and the discard is reported rather than absorbed (P2, R-25, R-47).
#[tokio::test]
async fn an_unusable_next_check_is_discarded_and_the_view_still_arrives() {
  let body = br#"{"view":{"kind":"choice","title":"T","options":[{"id":"ok","label":"Fine"}]},"next_check":"1 month"}"#;
  let (mut host, _calls) = host(vec![answering(body)]);

  let outcome = host.evaluate(now(), event()).await;

  assert!(outcome.failure.is_none(), "{}", describe_outcome(&outcome));
  assert!(outcome.view.is_some());
  assert_eq!(outcome.discarded.len(), 1, "the loss must be reported");
  assert_eq!(
    outcome.next_check,
    seeded_check(),
    "a discarded instruction leaves the existing schedule standing"
  );
}

/// Stderr and the cleanup verdict travel on the outcome whatever happened to
/// the result — they are facts any path can produce (R-42, R-54, F-24, F-39).
#[tokio::test]
async fn stderr_and_the_cleanup_verdict_survive_a_failed_exchange() {
  let (mut host, _calls) = host(vec![failing_noisily(
    BackendError::Timeout {
      after: std::time::Duration::from_secs(5),
    },
    "backend: could not reach the calendar\n",
    Some(CleanupFailure::TimedOut {
      after: std::time::Duration::from_millis(500),
    }),
  )]);

  let outcome = host.evaluate(now(), event()).await;

  assert!(matches!(
    backend_error(&outcome),
    BackendError::Timeout { .. }
  ));
  assert_eq!(
    String::from_utf8_lossy(&outcome.stderr.bytes),
    "backend: could not reach the calendar\n"
  );
  assert!(matches!(
    outcome.cleanup,
    Some(CleanupFailure::TimedOut { .. })
  ));
}

// ---------------------------------------------------------------------------
// EX-8, VT-6 — R-38's framing rule, at the one place `from_slice` runs
// ---------------------------------------------------------------------------

/// The three ways a body can fail to be exactly one JSON document. Each is a
/// `serde_json::Error`, so each is `Protocol(Json)`; and none of them may move
/// the schedule, which is EX-5's rule applied to this failure.
#[tokio::test]
async fn a_body_that_is_not_exactly_one_json_document_is_a_protocol_failure() {
  // The bad byte sits in a value the host actually **reads** — a view's title.
  // Measured at expansion: a byte serde *skips* is never decoded, so
  // `{"a":"\xff"}` parses as a `WireResponse` and the third case would have
  // asserted nothing. `design.md:1052` is about the case here — a title that
  // `String::from_utf8_lossy` would have silently turned into U+FFFD, which is
  // why `Exchange.result` carries `Vec<u8>`.
  let mut invalid_utf8 = A_CHOICE.to_vec();
  let title = invalid_utf8
    .iter()
    .position(|byte| *byte == b'H')
    .expect("the fixture title begins with an H");
  invalid_utf8[title] = 0xff;

  let cases: [(&str, &[u8]); 3] = [
    ("empty stdout", b""),
    ("a second document", br#"{"view":null} {"view":null}"#),
    ("bytes that are not UTF-8", &invalid_utf8),
  ];

  for (what, body) in cases {
    let (mut host, _calls) = host(vec![answering(body)]);
    let outcome = host.evaluate(now(), event()).await;

    assert!(
      matches!(
        backend_error(&outcome),
        BackendError::Protocol(goad::semantics::error::ProtocolError::Json(_))
      ),
      "{what} was not refused as a protocol failure: {}",
      describe_outcome(&outcome)
    );
    assert!(outcome.view.is_none(), "{what} produced a view");
    assert_eq!(
      outcome.next_check,
      seeded_check(),
      "{what} moved the schedule"
    );
  }
}

/// The other half of the same rule: trailing **whitespace** is not trailing
/// content. A backend ending its document with a newline is not sending two.
#[tokio::test]
async fn a_document_followed_by_whitespace_is_still_one_document() {
  let (mut host, _calls) = host(vec![answering(b"{\"view\":null}\n  ")]);

  let outcome = host.evaluate(now(), event()).await;

  assert!(outcome.failure.is_none(), "{}", describe_outcome(&outcome));
}

// ---------------------------------------------------------------------------
// EX-3, VT-3 — a caller naming an interaction the host is not holding
// ---------------------------------------------------------------------------

/// Nothing is outstanding, so there is nothing to answer — and the backend is
/// not consulted, because the refusal is a fact about host state (R-32, AC-8).
#[tokio::test]
async fn an_answer_against_an_idle_host_is_refused_and_no_exchange_happens() {
  let (mut host, calls) = host(vec![]);

  let outcome = host
    .respond(
      now(),
      ViewId::new("2026-08-23T04:12:00Z#0"),
      an_answer().await,
    )
    .await;

  assert!(
    matches!(state_error(&outcome), StateError::NoOutstandingView { .. }),
    "{}",
    describe_outcome(&outcome)
  );
  assert_eq!(calls.count(), 0, "the backend must not have been contacted");
  assert_eq!(outcome.next_check, seeded_check());
}

/// A superseded id is refused, the live one is named in the diagnostic, the
/// backend is not contacted, and the outstanding interaction **survives** the
/// rejection — which is R-34, and the reason the last exchange below succeeds.
#[tokio::test]
async fn a_superseded_id_is_refused_and_the_outstanding_interaction_survives() {
  let (mut host, calls) = host(vec![
    answering(A_CHOICE),
    answering(A_CHOICE),
    answering(br#"{"view":null}"#),
  ]);

  let first = presented(&host.evaluate(now(), event()).await).clone();
  let second = presented(&host.evaluate(now(), event()).await).clone();
  assert_ne!(
    first, second,
    "a returned view replaces the outstanding one"
  );
  let contacted = calls.count();

  let refused = host.respond(now(), first.clone(), an_answer().await).await;

  match state_error(&refused) {
    StateError::StaleViewId { named, outstanding } => {
      assert_eq!(named, &first);
      assert_eq!(outstanding, &second, "the diagnostic must name the live id");
    }
    other @ StateError::NoOutstandingView { .. } => {
      panic!("a superseded id was refused as {other}")
    }
  }
  assert_eq!(
    calls.count(),
    contacted,
    "a stale answer must not reach the backend"
  );

  let accepted = host.respond(now(), second, an_answer().await).await;
  assert!(
    accepted.failure.is_none(),
    "the rejection closed an interaction it was not for: {}",
    describe_outcome(&accepted)
  );
}

// ---------------------------------------------------------------------------
// EX-5, VT-4 — a failed exchange does not move the schedule
// ---------------------------------------------------------------------------

/// Three failures, one rule (R-29, P2). A backend that fails every invocation
/// still gets polled on its existing cadence, because the alternative turns a
/// broken backend into a silent host.
#[tokio::test]
async fn no_failure_moves_the_schedule() {
  let failures = [
    (
      "a timeout",
      failing(BackendError::Timeout {
        after: std::time::Duration::from_secs(5),
      }),
    ),
    (
      "a non-zero exit",
      failing(BackendError::ExitStatus { code: Some(1) }),
    ),
    ("malformed JSON", answering(b"not json at all")),
  ];

  for (what, exchange) in failures {
    let (mut host, _calls) = host(vec![exchange]);
    let outcome = host.evaluate(now(), event()).await;

    assert!(outcome.failure.is_some(), "{what} was not a failure");
    assert_eq!(outcome.next_check, seeded_check(), "{what} moved the check");
  }
}

/// The positive control for the case above: the schedule **can** move, so
/// "unchanged" is an observation rather than a property of a host that never
/// updates anything.
#[tokio::test]
async fn a_successful_exchange_does_move_the_schedule() {
  let (mut host, _calls) = host(vec![
    answering(br#"{"view":null,"next_check":"2h"}"#),
    failing(BackendError::ExitStatus { code: Some(1) }),
  ]);

  let moved = host.evaluate(now(), event()).await;
  assert_eq!(moved.next_check, instant("2026-08-23T06:12:00Z"));

  let failed = host.evaluate(now(), event()).await;
  assert_eq!(
    failed.next_check,
    instant("2026-08-23T06:12:00Z"),
    "a failure reports the check as it stood, not as it was seeded"
  );
}

// ---------------------------------------------------------------------------
// EX-6 — what an absent view means, which is the one place the two entry
// points differ (F-46, and §5.5's first two edge rows)
// ---------------------------------------------------------------------------

/// The backend was asked whether it had anything **new**, not whether the open
/// question still stands. So an outstanding interaction survives.
#[tokio::test]
async fn a_null_view_answering_an_evaluate_leaves_the_interaction_open() {
  let (mut host, _calls) = host(vec![
    answering(A_CHOICE),
    answering(br#"{"view":null}"#),
    answering(br#"{"view":null}"#),
  ]);

  let issued = presented(&host.evaluate(now(), event()).await).clone();
  let nothing = host.evaluate(now(), event()).await;
  assert!(nothing.view.is_none(), "{}", describe_outcome(&nothing));

  let accepted = host.respond(now(), issued, an_answer().await).await;
  assert!(
    accepted.failure.is_none(),
    "the interaction was closed by an evaluate that returned nothing: {}",
    describe_outcome(&accepted)
  );
}

/// The answer was taken and there is nothing further to show, so the host
/// returns to idle — and the id that was live is now answerable by nobody.
#[tokio::test]
async fn a_null_view_answering_a_respond_closes_the_interaction() {
  let (mut host, _calls) = host(vec![answering(A_CHOICE), answering(br#"{"view":null}"#)]);

  let issued = presented(&host.evaluate(now(), event()).await).clone();
  let closed = host.respond(now(), issued.clone(), an_answer().await).await;
  assert!(closed.view.is_none(), "{}", describe_outcome(&closed));

  let refused = host.respond(now(), issued, an_answer().await).await;
  assert!(
    matches!(state_error(&refused), StateError::NoOutstandingView { .. }),
    "the host did not return to idle: {}",
    describe_outcome(&refused)
  );
}

/// A view returned *by* a respond replaces the one it answered, rather than
/// closing the interaction (R-33, and the state diagram's self-transition).
#[tokio::test]
async fn a_view_returned_by_a_respond_replaces_the_one_it_answered() {
  let (mut host, _calls) = host(vec![answering(A_CHOICE), answering(A_CHOICE)]);

  let first = presented(&host.evaluate(now(), event()).await).clone();
  let second = host.respond(now(), first.clone(), an_answer().await).await;

  assert_ne!(presented(&second), &first, "a fresh id must be minted");
}
