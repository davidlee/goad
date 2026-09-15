//! Item 12 in full — design.md §9's failure case table (AC-7). Every failure
//! in SPEC-001's taxonomy, driven through one retained `Host<ProcessBackend>`
//! and read off the diagnostics the production reducer produced.
//!
//! §12.1-§12.9. The schema (§12.4) and the `CASES` array (§12.9) are
//! transcribed, not re-derived: §12.9 says so explicitly, and thirty-three
//! rows derived from prose is thirty-three chances to derive one differently.

use std::time::Duration;

use goad_semantics::protocol::canonical::{Timestamp, UserResponse, ViewId};
use goad_shell::config::Command;

use crate::driving::{CLEANUP_LIMIT, answer_first_option, host, instant, presented, quiet_event};
use crate::scripting::{invocations, scripted};
use goad::controller::{Controller, Exchanged, Shift};

// ---------------------------------------------------------------------------
// §12.4 — the schema
// ---------------------------------------------------------------------------

/// Which `Host` a row runs against. Not decoration: AC-7's "one retained
/// `Host`" is a claim about a cohort, and one row is honestly outside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cohort {
  /// The retained `Host` of §12.2, in sequence order.
  Retained,
  /// Its own `Host`. T1 alone, with the reason recorded in §12.6. The driver
  /// runs an `Own` row's turn **twice** and asserts the same `observed` both
  /// times.
  Own { command: &'static str },
}

/// Which prefix the driver writes before a row's text, so no row writes one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Channel {
  /// `no action taken: ` — `Outcome::failure`, through `Failure`'s `Display`.
  Failure,
  /// `no action taken: backend response rejected: ` — the same channel, with
  /// `BackendError::Protocol`'s own prefix folded in, so §12.3's fifteen rows
  /// do not write it fifteen times, and T1-T4, S1 and S2 do not inherit it.
  Protocol,
  /// `cleanup unverified: `
  Cleanup,
  /// No prefix: `Discarded`'s `Display` is already a whole sentence.
  Discard,
  /// `stderr: `
  Stderr,
}

impl Channel {
  fn prefix(self) -> &'static str {
    match self {
      Self::Failure => "no action taken: ",
      Self::Protocol => "no action taken: backend response rejected: ",
      Self::Cleanup => "cleanup unverified: ",
      Self::Discard => "",
      Self::Stderr => "stderr: ",
    }
  }
}

/// One expected diagnostic line, on one channel.
#[derive(Debug, Clone, Copy)]
struct Observed {
  channel: Channel,
  text: Expect,
}

/// How much of a line is pinned, and who owns the rest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Expect {
  /// The whole line after the channel prefix. Every line this repository
  /// authors.
  Exact(&'static str),
  /// The line begins with this and the tail is owned outside this
  /// repository: serde's message (P2, P3) and the OS's (T1).
  Prefixed(&'static str),
  /// The line is there and none of its text is this table's to pin: the
  /// child pid five fixtures write to stderr as R-41 bookkeeping (T2, T4,
  /// C1-C3).
  Unpinned,
}

/// Which entry point the row drives, and with which token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
  Evaluate,
  /// The id the suite cannot mint — S1 and S2.
  RespondFabricated,
  /// The token the last `Replaced` fold installed.
  RespondOutstanding,
}

/// What this exchange must leave `next_check` standing at. §12.2 needs three
/// different answers and a fourth for the exempt cohort, so it is a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Schedule {
  /// `now + default_poll`, untouched. Exchange 0 only.
  Seed,
  /// Whatever the last `MovedTo` set — R-29, and the one-`Host` witness.
  Unchanged,
  /// This exchange moved it, to an instant no other row sets.
  MovedTo(&'static str),
  /// The `Own` cohort has its own seed; §12.6 says why.
  NotAsserted,
}

#[derive(Debug, Clone, Copy)]
struct Case {
  /// Names the row in a failure message. Thirty-three rows through one loop,
  /// and a row without a name is reported by index.
  id: &'static str,
  cohort: Cohort,
  /// `None` = this row consumes no instruction from the scripted list:
  /// either it sends no request at all (S1, S2) **or** it runs against its
  /// own `Host` (T1, which does spawn — or tries to).
  instruction: Option<&'static str>,
  turn: Turn,
  /// The **complete** list of lines this row produces, per channel — not a
  /// sample.
  observed: &'static [Observed],
  /// What the fold did to the outstanding interaction.
  shift: Shift,
  /// `Outcome::failure.is_some()`.
  refused: bool,
  schedule: Schedule,
  /// How far the invocation log moves across this exchange: 1 for every row
  /// that reaches a process, 0 for S1, S2 and T1's refusal-shaped attempt.
  invocations: usize,
}

// ---------------------------------------------------------------------------
// §12.9 — the array itself, preserved
// ---------------------------------------------------------------------------
//
// --- the bodies, each the `input` of the fixture named above it -------------

/// The view exchange 1 mints, and the schedule every later row asserts.
const PRESENTS_A_VIEW: &str = r#"{"view":{"kind":"choice","title":"Still here?","options":[{"id":"yes","label":"Yes"}]},"next_check":"45 minutes"}"#;
/// What the backend answers view **A** with.
const ACCEPTS_A: &str = r#"{"view":null,"next_check":"90 minutes"}"#;
/// What it answers view **B** with — a fourth instant, set by nothing else.
const ACCEPTS_B: &str = r#"{"view":null,"next_check":"150 minutes"}"#;

const UNSUPPORTED_VERSION: &str = r#"{"protocol":2,"view":null}"#;
const TITLE_NOT_A_STRING: &str =
  r#"{"view":{"kind":"choice","title":5,"options":[{"id":"ok","label":"Fine"}]}}"#;
const VIEW_OMITTED: &str = r#"{"next_check":"45 minutes"}"#;
const DUPLICATE_ENVELOPE_KEY: &str = r#"{"view":null,"next_check":"1h","next_check":"2h"}"#;
const NESTED_HINTS: &str = r#"{"view":{"kind":"choice","title":"T","options":[{"id":"ok","label":"Fine","fields":[{"id":"f","kind":"text","label":"L","hints":{"multiline":true}}]}]}}"#;
const UNKNOWN_NESTED_KIND: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine"},{"id":"no","label":"Badly","fields":[{"id":"a","kind":"text","label":"A"},{"id":"b","kind":"text","label":"B"},{"id":"c","kind":"slider","label":"C"}]}]}}"#;
const MIN_ON_A_TEXT_FIELD: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine","fields":[{"id":"f","kind":"text","label":"L","min":1}]}]}}"#;
const OPTIONS_ON_A_NUMBER_FIELD: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine","fields":[{"id":"f","kind":"number","label":"L","options":[{"id":"red","label":"Red"}]}]}]}}"#;
const NO_OPTIONS: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[]}}"#;
const DUPLICATE_OPTION_IDS: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"later","label":"Ask me later"},{"id":"later","label":"Not now"}]}}"#;
const DUPLICATE_FIELD_IDS: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine","fields":[{"id":"note","kind":"text","label":"A"},{"id":"note","kind":"text","label":"B"}]}]}}"#;
const DUPLICATE_ALTERNATIVE_IDS: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine","fields":[{"id":"f","kind":"choice","label":"L","options":[{"id":"red","label":"Red"},{"id":"red","label":"Also red"}]}]}]}}"#;
const NO_ALTERNATIVES: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine","fields":[{"id":"f","kind":"choice","label":"L","options":[]}]}]}}"#;
const INVERTED_BOUNDS: &str = r#"{"view":{"kind":"choice","title":"How did it go?","options":[{"id":"ok","label":"Fine","fields":[{"id":"f","kind":"number","label":"L","min":10,"max":1}]}]}}"#;

const NEXT_CHECK_WRONG_TYPE: &str = r#"{"view":null,"next_check":45}"#;
const NEXT_CHECK_NO_OFFSET: &str = r#"{"view":null,"next_check":"2026-08-22T18:00:00"}"#;
const NEXT_CHECK_TIME_OF_DAY: &str = r#"{"view":null,"next_check":"18:00:00"}"#;
const NEXT_CHECK_CALENDAR_UNIT: &str = r#"{"view":null,"next_check":"1 month"}"#;
const NEXT_CHECK_OUT_OF_RANGE: &str = r#"{"view":null,"next_check":"1000000 weeks"}"#;
const NEXT_CHECK_PROSE: &str = r#"{"view":null,"next_check":"tomorrow morning"}"#;

/// The suite's configured deadline (§12.1). Not the cleanup budget, which
/// happens to be the same number for a different reason.
const DEADLINE: &str = "backend did not respond within 500ms";

/// The transport's private cleanup budget, restated as the sentence
/// `CleanupFailure::TimedOut`'s `Display` produces for it — the driver
/// asserts `CLEANUP_LIMIT.as_millis() == 500` at the top of the table so a
/// change to the real budget fails loudly here rather than silently
/// invalidating every C row (§12.8's keep-in-sync note).
const DISPOSAL: &str = "backend was not disposed of within 500ms";

/// A pid on stderr, written by five fixtures as R-41 bookkeeping.
const PID: Observed = Observed {
  channel: Channel::Stderr,
  text: Expect::Unpinned,
};

const fn nothing() -> &'static [Observed] {
  &[]
}

macro_rules! observed {
  ($($channel:ident : $form:ident ( $text:expr )),* $(,)?) => {
    &[$(Observed { channel: Channel::$channel, text: Expect::$form($text) }),*]
  };
}

const fn retained(
  id: &'static str,
  instruction: &'static str,
  observed: &'static [Observed],
) -> Case {
  Case {
    id,
    cohort: Cohort::Retained,
    instruction: Some(instruction),
    turn: Turn::Evaluate,
    observed,
    shift: Shift::Retained,
    refused: true,
    schedule: Schedule::Unchanged,
    invocations: 1,
  }
}

const fn discard(
  id: &'static str,
  instruction: &'static str,
  observed: &'static [Observed],
) -> Case {
  Case {
    refused: false,
    ..retained(id, instruction, observed)
  }
}

static CASES: &[Case] = &[
  // exchange 0 — the only state in which `NoOutstandingView` is reachable
  Case {
    id: "S1",
    cohort: Cohort::Retained,
    instruction: None,
    turn: Turn::RespondFabricated,
    observed: observed![Failure: Exact(
      "no interaction is outstanding, so 2026-08-23T04:12:00Z#9 answers nothing"
    )],
    shift: Shift::Retained,
    refused: true,
    schedule: Schedule::Seed,
    invocations: 0,
  },
  // exchange 1 — mints A and moves the schedule off its seed
  Case {
    id: "A",
    cohort: Cohort::Retained,
    instruction: Some(PRESENTS_A_VIEW),
    turn: Turn::Evaluate,
    observed: nothing(),
    shift: Shift::Replaced,
    refused: false,
    schedule: Schedule::MovedTo("2026-08-23T04:57:00Z"),
    invocations: 1,
  },
  // A. protocol refusals
  retained(
    "P1",
    UNSUPPORTED_VERSION,
    observed![Protocol: Exact("unsupported protocol version 2")],
  ),
  retained(
    "P2",
    "@garbage",
    observed![
      Protocol: Prefixed("malformed JSON: "),
      Stderr: Exact("config is missing, so this is all you get\\n"),
    ],
  ),
  retained(
    "P3",
    TITLE_NOT_A_STRING,
    observed![Protocol: Prefixed("protocol-invalid message: ")],
  ),
  retained(
    "P4",
    VIEW_OMITTED,
    observed![Protocol: Exact("missing required field `view`")],
  ),
  retained(
    "P5",
    DUPLICATE_ENVELOPE_KEY,
    observed![Protocol: Exact("duplicate key `next_check`")],
  ),
  retained(
    "P6",
    NESTED_HINTS,
    observed![Protocol: Exact(
      "hints are the field's own keys, not a nested `hints` key, at view.options[0].fields[0]"
    )],
  ),
  retained(
    "P7",
    UNKNOWN_NESTED_KIND,
    observed![Protocol: Exact(
      "unsupported primitive `slider` at view.options[1].fields[2].kind"
    )],
  ),
  retained(
    "P8",
    MIN_ON_A_TEXT_FIELD,
    observed![Protocol: Exact(
      "key `min` does not apply to a `text` at view.options[0].fields[0]"
    )],
  ),
  retained(
    "P9",
    OPTIONS_ON_A_NUMBER_FIELD,
    observed![Protocol: Exact(
      "key `options` does not apply to a `number` at view.options[0].fields[0]"
    )],
  ),
  retained(
    "P10",
    NO_OPTIONS,
    observed![Protocol: Exact("no options at view.options")],
  ),
  retained(
    "P11",
    DUPLICATE_OPTION_IDS,
    observed![Protocol: Exact(
      "duplicate option id `later` at view.options"
    )],
  ),
  retained(
    "P12",
    DUPLICATE_FIELD_IDS,
    observed![Protocol: Exact(
      "duplicate field id `note` at view.options[0].fields"
    )],
  ),
  retained(
    "P13",
    DUPLICATE_ALTERNATIVE_IDS,
    observed![Protocol: Exact(
      "duplicate alternative id `red` at view.options[0].fields[0].options"
    )],
  ),
  retained(
    "P14",
    NO_ALTERNATIVES,
    observed![Protocol: Exact(
      "no alternatives at view.options[0].fields[0].options"
    )],
  ),
  retained(
    "P15",
    INVERTED_BOUNDS,
    observed![Protocol: Exact("invalid bounds: min 10 is above max 1")],
  ),
  // E. the second state refusal, mid-sequence, with A outstanding
  Case {
    id: "S2",
    cohort: Cohort::Retained,
    instruction: None,
    turn: Turn::RespondFabricated,
    observed: observed![Failure: Exact(
      "2026-08-23T04:12:00Z#9 is superseded; the outstanding interaction is <A>"
    )],
    shift: Shift::Retained,
    refused: true,
    schedule: Schedule::Unchanged,
    invocations: 0,
  },
  // B. transport failures
  Case {
    id: "T1",
    cohort: Cohort::Own {
      command: "/nonexistent/goad-has-no-such-backend",
    },
    instruction: None,
    turn: Turn::Evaluate,
    observed: observed![Failure: Prefixed("backend could not be spawned: ")],
    shift: Shift::Retained,
    refused: true,
    schedule: Schedule::NotAsserted,
    invocations: 0,
  },
  retained(
    "T2",
    "@hang",
    &[
      Observed {
        channel: Channel::Failure,
        text: Expect::Exact(DEADLINE),
      },
      PID,
    ],
  ),
  retained(
    "T3",
    "@exit1",
    observed![
      Failure: Exact("backend exited with status 1"),
      Stderr: Exact("that answer is not to be trusted\\n"),
    ],
  ),
  retained(
    "T4",
    "@flood",
    &[
      Observed {
        channel: Channel::Failure,
        text: Expect::Exact("backend wrote more than 8388608 bytes to stdout"),
      },
      PID,
    ],
  ),
  // C. cleanup
  discard(
    "C1",
    "@lingers",
    &[
      Observed {
        channel: Channel::Cleanup,
        text: Expect::Exact(DISPOSAL),
      },
      PID,
    ],
  ),
  retained(
    "C2",
    "@lingers-and-hangs",
    &[
      Observed {
        channel: Channel::Failure,
        text: Expect::Exact(DEADLINE),
      },
      Observed {
        channel: Channel::Cleanup,
        text: Expect::Exact(DISPOSAL),
      },
      PID,
    ],
  ),
  // D. discards
  discard(
    "D1",
    NEXT_CHECK_WRONG_TYPE,
    observed![Discard: Exact(
      "next_check 45 discarded: schedule must be a string, found number"
    )],
  ),
  discard(
    "D2",
    NEXT_CHECK_NO_OFFSET,
    observed![Discard: Exact(
      "next_check discarded: schedule has no UTC offset: 2026-08-22T18:00:00"
    )],
  ),
  discard(
    "D3",
    NEXT_CHECK_TIME_OF_DAY,
    observed![Discard: Exact(
      "next_check discarded: schedule is a time of day, which is neither an instant nor a span: 18:00:00"
    )],
  ),
  discard(
    "D4",
    NEXT_CHECK_CALENDAR_UNIT,
    observed![Discard: Exact(
      "next_check discarded: schedule uses a calendar unit, which has no fixed length: 1 month"
    )],
  ),
  discard(
    "D5",
    NEXT_CHECK_OUT_OF_RANGE,
    observed![Discard: Exact(
      "next_check discarded: schedule leaves the representable range: 1000000 weeks"
    )],
  ),
  discard(
    "D6",
    NEXT_CHECK_PROSE,
    observed![Discard: Exact(
      "next_check discarded: unparseable schedule: tomorrow morning"
    )],
  ),
  // the coda
  Case {
    id: "answer-A",
    cohort: Cohort::Retained,
    instruction: Some(ACCEPTS_A),
    turn: Turn::RespondOutstanding,
    observed: nothing(),
    shift: Shift::Closed,
    refused: false,
    schedule: Schedule::MovedTo("2026-08-23T05:44:00Z"),
    invocations: 1,
  },
  Case {
    id: "C3",
    cohort: Cohort::Retained,
    instruction: Some("@lingers-with-a-view"),
    turn: Turn::Evaluate,
    observed: &[
      Observed {
        channel: Channel::Cleanup,
        text: Expect::Exact(DISPOSAL),
      },
      PID,
    ],
    shift: Shift::Replaced,
    refused: false,
    schedule: Schedule::MovedTo("2026-08-23T06:12:00Z"),
    invocations: 1,
  },
  Case {
    id: "answer-B",
    cohort: Cohort::Retained,
    instruction: Some(ACCEPTS_B),
    turn: Turn::RespondOutstanding,
    observed: nothing(),
    shift: Shift::Closed,
    refused: false,
    schedule: Schedule::MovedTo("2026-08-23T06:44:00Z"),
    invocations: 1,
  },
];

// ---------------------------------------------------------------------------
// The driver — VT-1
// ---------------------------------------------------------------------------

/// The transport's configured timeout **and** the suite's deadline: the same
/// 500ms `DEADLINE`'s text depends on (§12.1).
const TIMEOUT: Duration = Duration::from_millis(500);

/// Every `evaluate` in this table runs at this instant (§12.2).
fn evaluate_now() -> Timestamp {
  instant("2026-08-23T04:12:00Z")
}

/// Every `respond` in this table runs at this instant — distinct from
/// `evaluate_now()` so a `next_check` resolved from the wrong one does not
/// pass unnoticed (§12.2).
fn respond_now() -> Timestamp {
  instant("2026-08-23T04:14:00Z")
}

/// `now + default_poll` — the instant exchange 0 must still stand at.
fn seed() -> Timestamp {
  instant("2026-08-23T04:42:00Z")
}

/// The fabricated id S1 and S2 both hand to `Host::respond` — never minted by
/// this suite, so it cannot collide with what `State::issue` produces
/// (§12.3's row S1/S2).
fn fabricated_view_id() -> ViewId {
  ViewId::new("2026-08-23T04:12:00Z#9")
}

/// A `UserResponse` whose content is inert: S1 and S2 are refused before any
/// backend is consulted (R-32), so the option it names is never read. `OptionId`
/// has no public constructor (D30), so it is cloned out of a throwaway view
/// minted on a private log — the same move `crates/goad-shell/tests/integration/
/// host.rs`'s `an_answer` makes for the same reason.
///
/// Takes the row's id because it is called **once per row that reaches it** —
/// S1 and S2, the two of the thirty-three carrying a `RespondFabricated` turn
/// — and `marker` hands a name out once per test binary. A name is a path, and
/// a helper reusing one would have both rows writing to the same file
/// (`review-code.md` F-5, F-12).
async fn inert_answer(row: &str) -> UserResponse {
  let (command, _log) = scripted(&format!("table-inert-option-{row}"), &[PRESENTS_A_VIEW]);
  let mut throwaway = host(command, TIMEOUT, evaluate_now());
  let outcome = throwaway
    .evaluate(evaluate_now(), quiet_event(evaluate_now()))
    .await;
  answer_first_option(&outcome)
}

/// Assert `lines` is exactly `observed`, per channel, in order — §12.4's
/// first assertion: equality, not containment, so an extra or missing line
/// fails too. `outstanding` substitutes S2's `<A>` placeholder for the real
/// id the last `Replaced` fold installed (§12.9, note 1); every other row's
/// text has no `<A>` substring, so the substitution is a no-op for them.
fn assert_observed(id: &str, lines: &[String], observed: &[Observed], outstanding: Option<&str>) {
  assert_eq!(
    lines.len(),
    observed.len(),
    "row {id}: expected {} diagnostic line(s), got {lines:?}",
    observed.len()
  );
  for (line, expectation) in lines.iter().zip(observed) {
    let prefix = expectation.channel.prefix();
    assert!(
      line.starts_with(prefix),
      "row {id}: line {line:?} does not start with the {:?} channel's prefix {prefix:?}",
      expectation.channel
    );
    let tail = &line[prefix.len()..];
    match expectation.text {
      Expect::Exact(text) => {
        let expected = match outstanding {
          Some(real) => text.replace("<A>", real),
          None => text.to_owned(),
        };
        assert_eq!(tail, expected, "row {id}");
      }
      Expect::Prefixed(text) => {
        assert!(
          tail.starts_with(text),
          "row {id}: tail {tail:?} does not start with {text:?}"
        );
      }
      Expect::Unpinned => {
        // Only the prefix is this table's to pin — already checked above.
      }
    }
  }
}

/// The instruction list, ordered by the exchanges that actually reach a
/// process — S1, S2 and T1 consume none (§12.1).
fn instructions() -> Vec<&'static str> {
  CASES.iter().filter_map(|case| case.instruction).collect()
}

/// §12.9's own count: thirty-three rows, no more and no fewer.
#[test]
fn the_array_has_exactly_the_rows_design_md_states() {
  assert_eq!(CASES.len(), 33);
}

#[tokio::test]
async fn every_failure_in_the_taxonomy_is_read_off_one_retained_host() {
  let instructions = instructions();
  let (command, log) = scripted("table", &instructions);
  let mut retained_host = host(command, TIMEOUT, evaluate_now());
  let mut controller = Controller::new();

  // The keep-in-sync witness §12.8 asks for: if `process.rs`'s private
  // budget ever moves, `DISPOSAL` (a literal, like every other pinned string
  // in this table) goes stale silently unless something asserts against the
  // real value. This is that something.
  assert_eq!(
    CLEANUP_LIMIT.as_millis(),
    500,
    "DISPOSAL's pinned text assumes the transport's cleanup budget is 500ms"
  );

  let mut seen_invocations = 0usize;
  let mut current_schedule = seed();
  let mut outstanding: Option<String> = None;
  // The outcome of the exchange that most recently minted a view — carries
  // the ViewId and the option to answer it with, for `RespondOutstanding`.
  let mut open_view: Option<(ViewId, UserResponse)> = None;

  for case in CASES {
    match case.cohort {
      Cohort::Retained => {
        let outcome = match case.turn {
          Turn::Evaluate => {
            let now = evaluate_now();
            retained_host.evaluate(now, quiet_event(now)).await
          }
          Turn::RespondFabricated => {
            retained_host
              .respond(
                respond_now(),
                fabricated_view_id(),
                inert_answer(case.id).await,
              )
              .await
          }
          Turn::RespondOutstanding => {
            let (view_id, answer) = open_view.take().expect("a view must be outstanding");
            retained_host.respond(respond_now(), view_id, answer).await
          }
        };

        let next_check = outcome.next_check;
        let refused = outcome.failure.is_some();
        let minted = outcome
          .view
          .is_some()
          .then(|| (presented(&outcome).clone(), answer_first_option(&outcome)));

        let exchanged = match case.turn {
          Turn::Evaluate => Exchanged::Evaluation,
          Turn::RespondFabricated | Turn::RespondOutstanding => Exchanged::Answer,
        };
        let shift = controller.absorb(exchanged, outcome).shift;
        let lines = controller.frame(false).diagnostics.lines().to_vec();

        assert_eq!(shift, case.shift, "row {}: wrong Shift", case.id);
        assert_eq!(refused, case.refused, "row {}: wrong `refused`", case.id);
        assert_observed(case.id, &lines, case.observed, outstanding.as_deref());

        match case.schedule {
          Schedule::Seed => assert_eq!(next_check, seed(), "row {}", case.id),
          Schedule::Unchanged => assert_eq!(next_check, current_schedule, "row {}", case.id),
          Schedule::MovedTo(text) => {
            let expected = instant(text);
            assert_eq!(next_check, expected, "row {}", case.id);
            current_schedule = expected;
          }
          Schedule::NotAsserted => panic!("no `Retained` row is exempt from the schedule check"),
        }

        if case.invocations > 0 {
          seen_invocations += case.invocations;
        }
        assert_eq!(
          invocations(&log),
          seen_invocations,
          "row {}: invocation count",
          case.id
        );

        if shift == Shift::Replaced {
          let (view_id, answer) = minted.expect("a `Replaced` fold always carries a view");
          outstanding = Some(view_id.as_str().to_owned());
          open_view = Some((view_id, answer));
        }
      }
      Cohort::Own {
        command: own_command,
      } => {
        assert_eq!(case.turn, Turn::Evaluate, "T1 is the only `Own` row today");
        for attempt in 0..2 {
          let mut own_host = host(
            Command::new(own_command, Vec::new()),
            TIMEOUT,
            evaluate_now(),
          );
          let mut scratch = Controller::new();
          let outcome = own_host
            .evaluate(evaluate_now(), quiet_event(evaluate_now()))
            .await;
          let refused = outcome.failure.is_some();
          let shift = scratch.absorb(Exchanged::Evaluation, outcome).shift;
          let lines = scratch.frame(false).diagnostics.lines().to_vec();

          assert_eq!(
            shift, case.shift,
            "row {} (attempt {attempt}): wrong Shift",
            case.id
          );
          assert_eq!(
            refused, case.refused,
            "row {} (attempt {attempt}): wrong `refused`",
            case.id
          );
          assert_observed(case.id, &lines, case.observed, None);
        }
      }
    }
  }

  assert_eq!(
    invocations(&log),
    instructions.len(),
    "every instruction must be consumed by exactly one invocation, and no more"
  );
}

// ---------------------------------------------------------------------------
// VT-2 — the seven reducer rows, asserted as `Shift` values directly
// ---------------------------------------------------------------------------

mod reducer {
  use goad_semantics::protocol::canonical::ViewId;
  use goad_semantics::protocol::normalize::read_response;
  use goad_shell::backend::transport::Captured;
  use goad_shell::error::{BackendError, StateError};
  use goad_shell::host::{Failure, Outcome, Presented};

  use super::{Controller, Exchanged, Shift, evaluate_now};

  fn bare() -> Outcome {
    Outcome {
      view: None,
      next_check: evaluate_now(),
      discarded: Vec::new(),
      stderr: Captured::default(),
      failure: None,
      cleanup: None,
    }
  }

  fn presented_view() -> Presented {
    let bytes = br#"{"view":{"kind":"choice","title":"T","options":[{"id":"ok","label":"Fine"}]}}"#;
    let normalized = read_response(bytes, evaluate_now()).expect("the fixture must normalize");
    let view = normalized
      .value
      .view()
      .expect("the fixture carries a view")
      .clone();
    Presented {
      view_id: ViewId::new("2026-08-23T04:12:00Z#0"),
      view,
    }
  }

  #[test]
  fn row_1_a_view_with_no_failure_replaces_either_entry_point() {
    for exchanged in [Exchanged::Evaluation, Exchanged::Answer] {
      let outcome = Outcome {
        view: Some(presented_view()),
        ..bare()
      };
      assert_eq!(
        Controller::new().absorb(exchanged, outcome).shift,
        Shift::Replaced
      );
    }
  }

  #[test]
  fn row_2_an_evaluation_with_nothing_to_show_retains() {
    assert_eq!(
      Controller::new()
        .absorb(Exchanged::Evaluation, bare())
        .shift,
      Shift::Retained
    );
  }

  #[test]
  fn row_3_an_accepted_answer_with_nothing_further_closes() {
    assert_eq!(
      Controller::new().absorb(Exchanged::Answer, bare()).shift,
      Shift::Closed
    );
  }

  #[test]
  fn row_4_a_failed_evaluation_with_no_view_retains() {
    let outcome = Outcome {
      failure: Some(Failure::Backend(BackendError::Spawn(
        std::io::Error::other("no such program"),
      ))),
      ..bare()
    };
    assert_eq!(
      Controller::new()
        .absorb(Exchanged::Evaluation, outcome)
        .shift,
      Shift::Retained
    );
  }

  #[test]
  fn row_5_a_state_refusal_reached_by_construction_retains() {
    let outcome = Outcome {
      failure: Some(Failure::State(StateError::NoOutstandingView {
        named: ViewId::new("2026-08-23T04:12:00Z#9"),
      })),
      ..bare()
    };
    assert_eq!(
      Controller::new().absorb(Exchanged::Answer, outcome).shift,
      Shift::Retained
    );
  }

  #[test]
  fn row_6_a_backend_failure_on_an_answer_retains() {
    let outcome = Outcome {
      failure: Some(Failure::Backend(BackendError::ExitStatus { code: Some(1) })),
      ..bare()
    };
    assert_eq!(
      Controller::new().absorb(Exchanged::Answer, outcome).shift,
      Shift::Retained
    );
  }

  #[test]
  fn row_7_a_view_and_a_failure_together_still_replaces() {
    // Unreachable in production (`accept` never mints a view alongside a
    // failure), and asserted anyway: row 7 is written as `Replaced`, not
    // `unreachable!()`, so the arm is total on a value the host itself
    // could in principle produce.
    let outcome = Outcome {
      view: Some(presented_view()),
      failure: Some(Failure::Backend(BackendError::ExitStatus { code: Some(1) })),
      ..bare()
    };
    assert_eq!(
      Controller::new()
        .absorb(Exchanged::Evaluation, outcome)
        .shift,
      Shift::Replaced
    );
  }
}

// ---------------------------------------------------------------------------
// VT-3 — `Frame::busy` clears, whatever the outcome
// ---------------------------------------------------------------------------

mod busy {
  use goad_shell::backend::transport::Captured;
  use goad_shell::host::Outcome;

  use super::{Controller, Exchanged, evaluate_now};

  fn bare() -> Outcome {
    Outcome {
      view: None,
      next_check: evaluate_now(),
      discarded: Vec::new(),
      stderr: Captured::default(),
      failure: None,
      cleanup: None,
    }
  }

  #[test]
  fn busy_is_false_after_absorbing_a_success() {
    let mut controller = Controller::new();
    controller.engage();
    assert!(controller.frame(false).busy, "engage() must set it first");
    controller.absorb(Exchanged::Evaluation, bare());
    assert!(
      !controller.frame(false).busy,
      "absorb() must clear it (F-21)"
    );
  }

  #[test]
  fn busy_is_false_after_absorbing_a_failure() {
    use goad_shell::error::BackendError;
    use goad_shell::host::Failure;

    let mut controller = Controller::new();
    controller.engage();
    let outcome = Outcome {
      failure: Some(Failure::Backend(BackendError::ExitStatus { code: Some(1) })),
      ..bare()
    };
    controller.absorb(Exchanged::Evaluation, outcome);
    assert!(
      !controller.frame(false).busy,
      "a failed outcome must clear `engaged` too — the negative control that matters"
    );
  }
}
