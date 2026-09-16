//! design.md §9 item 13 (AC-8, AC-9, I-2): `receive` and `Diagnostics`,
//! tested directly, with no component and no runtime.
//!
//! `docs/memory/a-bound-is-not-tested-at-the-bound.md`: VT-4 poses the bound
//! tests as an exact character count at *limit − 1*, *limit* and *limit + 1*
//! rather than trusting "it looks truncated" — the marker's presence and the
//! kept prefix's length are asserted separately so a reader that stops one
//! character early or late is caught.

use std::time::Duration;

use goad::diagnostics::{BUSY_NOTICE, Diagnostics, Refused, Reported, TrayState, tooltip};
use goad::reception::receive;
use goad::view_model::{Body, ContentForm, Undrawn};
use goad_semantics::error::ScheduleError;
use goad_semantics::protocol::canonical::{Timestamp, View, ViewId};
use goad_semantics::protocol::normalize::{Discarded, read_response};
use goad_shell::backend::transport::Captured;
use goad_shell::error::{BackendError, CleanupFailure, StateError};
use goad_shell::host::{Failure, Outcome, Presented};

/// design.md's three display bounds, restated here as the contract under
/// test rather than read back from the module: the point of these tests is
/// that the numbers in the design and the numbers in the code agree.
const STDERR_BOUND: usize = 4096;
const LINE_BOUND: usize = 1024;
const TOOLTIP_BOUND: usize = 120;

/// The capture-truncated sentence, restated here as the contract under test
/// (`design.md` §5.4's *exact strings*) — the const in `diagnostics.rs` is
/// deliberately private; `BUSY_NOTICE` is the only one of these the design
/// makes public.
const CAPTURE_TRUNCATED: &str =
  "stderr was cut at the host's capture limit; the backend wrote more than the host kept";

fn now() -> Timestamp {
  Timestamp::new(
    "2026-01-01T00:00:00Z"
      .parse()
      .expect("the fixture must be an instant"),
  )
}

fn view_from(document: &serde_json::Value) -> View {
  let bytes = document.to_string();
  read_response(bytes.as_bytes(), now())
    .expect("the fixture must normalize")
    .value
    .view()
    .expect("the fixture carries a view")
    .clone()
}

fn choice_with_body(body: Option<serde_json::Value>) -> View {
  let options = serde_json::json!([{ "id": "ok", "label": "Fine" }]);
  let mut view = serde_json::json!({ "kind": "choice", "title": "T", "options": options });
  if let Some(body) = body {
    view["body"] = body;
  }
  view_from(&serde_json::json!({ "view": view }))
}

fn choice_with_markdown(source: &str) -> View {
  choice_with_body(Some(
    serde_json::json!({ "kind": "markdown", "value": source }),
  ))
}

/// An `Outcome` with no view, no discard, no stderr and no cleanup —
/// callers override exactly the field the test is about.
fn bare_outcome() -> Outcome {
  Outcome {
    view: None,
    next_check: now(),
    discarded: Vec::new(),
    stderr: Captured::default(),
    failure: None,
    cleanup: None,
  }
}

fn outcome_with_view(view: View) -> Outcome {
  Outcome {
    view: Some(Presented {
      view_id: ViewId::new("v1"),
      view,
    }),
    ..bare_outcome()
  }
}

fn no_outstanding_view(named_len: usize) -> Failure {
  Failure::State(StateError::NoOutstandingView {
    named: ViewId::new("a".repeat(named_len)),
  })
}

/// The line `Diagnostics::of` renders for a lone `failure`, at whatever
/// length `named_len` drives the composed (pre-bound) text to.
fn failure_line(named_len: usize) -> String {
  let reported = Reported {
    failure: Some(no_outstanding_view(named_len)),
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured::default(),
  };
  Diagnostics::of(reported, &[]).lines()[0].clone()
}

fn stderr_line(byte_len: usize) -> String {
  let reported = Reported {
    failure: None,
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured {
      bytes: vec![b'a'; byte_len],
      truncated: false,
    },
  };
  Diagnostics::of(reported, &[]).lines()[0].clone()
}

/// A capture reaching the surface through `Diagnostics::of`, for the one
/// claim about terminators that is made nowhere else.
///
/// The *trailing* terminator is held twice already and not here: by
/// `diagnostics.rs`'s own unit cases, which state the rule (**at most one**,
/// never a `trim_end`), and by `table.rs` rows P2 and T3, which read the line
/// off a real host and are the binding site. A third statement of it would go
/// stale without anything noticing.
fn stderr_line_for(bytes: &str) -> String {
  let reported = Reported {
    failure: None,
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured {
      bytes: bytes.as_bytes().to_vec(),
      truncated: false,
    },
  };
  Diagnostics::of(reported, &[]).lines()[0].clone()
}

/// Only the **last** line's terminator is the writing convention. A capture of
/// two lines still renders as one line of this surface, with the newline
/// between them escaped — that escape is what stops one capture breaking the
/// list's shape, and it is not what the trim is for.
#[test]
fn a_newline_between_two_captured_lines_is_still_escaped() {
  assert_eq!(stderr_line_for("first\nsecond\n"), "stderr: first\\nsecond");
}

/// The `"stderr: "` prefix's length. A byte count of zero produces **no**
/// line at all (an empty capture is not diagnostic), so this is measured at
/// one byte and corrected, rather than read at zero.
fn stderr_line_base() -> usize {
  stderr_line(1).chars().count() - 1
}

// ---------------------------------------------------------------------
// 13a — the signature holds the invariant.
// ---------------------------------------------------------------------

#[test]
fn a_view_with_rejected_markdown_yields_unclear_diagnostics_and_a_prepared_presentation() {
  let received = receive(outcome_with_view(choice_with_markdown("\u{e541}")));
  assert!(received.prepared.is_some());
  assert!(!received.diagnostics.is_clear());
}

#[test]
fn a_clean_view_yields_clear_diagnostics_and_a_prepared_presentation() {
  let body = Some(serde_json::json!({ "kind": "text", "value": "hi" }));
  let received = receive(outcome_with_view(choice_with_body(body)));
  assert!(received.prepared.is_some());
  assert!(received.diagnostics.is_clear());
  assert!(!received.refused);
}

// ---------------------------------------------------------------------
// 13b — ordering: failure, cleanup, undrawn, discarded, capture, stderr.
// ---------------------------------------------------------------------

#[test]
fn six_facts_at_once_render_in_the_stated_order() {
  let html_view = choice_with_body(Some(
    serde_json::json!({ "kind": "html", "value": "<b>hi</b>" }),
  ));
  let outcome = Outcome {
    view: Some(Presented {
      view_id: ViewId::new("v1"),
      view: html_view,
    }),
    next_check: now(),
    discarded: vec![Discarded::Schedule {
      raw: serde_json::json!("18:00:00"),
      reason: ScheduleError::TimeOfDay {
        raw: "18:00:00".to_owned(),
      },
    }],
    stderr: Captured {
      bytes: b"oops".to_vec(),
      truncated: true,
    },
    failure: Some(no_outstanding_view(0)),
    cleanup: Some(CleanupFailure::TimedOut {
      after: Duration::from_millis(500),
    }),
  };

  let received = receive(outcome);
  let lines = received.diagnostics.lines();
  assert_eq!(lines.len(), 6, "{lines:#?}");
  assert_eq!(
    lines[0],
    "no action taken: no interaction is outstanding, so  answers nothing"
  );
  assert_eq!(
    lines[1],
    "cleanup unverified: backend was not disposed of within 500ms"
  );
  assert_eq!(
    lines[2],
    "shown as plain text: this body is HTML, and nothing here draws that form"
  );
  assert_eq!(
    lines[3],
    "next_check discarded: schedule is a time of day, which is neither an instant nor a span: 18:00:00"
  );
  assert_eq!(lines[4], CAPTURE_TRUNCATED);
  assert_eq!(lines[5], "stderr: oops");
}

// ---------------------------------------------------------------------
// 13c — severity.
// ---------------------------------------------------------------------

#[test]
fn stderr_alone_is_idle_and_not_clear() {
  let reported = Reported {
    failure: None,
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured {
      bytes: b"hello".to_vec(),
      truncated: false,
    },
  };
  let diagnostics = Diagnostics::of(reported, &[]);
  assert_eq!(diagnostics.state(), TrayState::Idle);
  assert!(!diagnostics.is_clear());
}

#[test]
fn a_failure_alone_raises_fault() {
  let reported = Reported {
    failure: Some(no_outstanding_view(1)),
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured::default(),
  };
  assert_eq!(Diagnostics::of(reported, &[]).state(), TrayState::Fault);
}

#[test]
fn a_cleanup_failure_alone_raises_fault() {
  let reported = Reported {
    failure: None,
    cleanup: Some(CleanupFailure::TimedOut {
      after: Duration::from_millis(1),
    }),
    discarded: Vec::new(),
    stderr: Captured::default(),
  };
  assert_eq!(Diagnostics::of(reported, &[]).state(), TrayState::Fault);
}

#[test]
fn a_discard_alone_raises_fault() {
  let reported = Reported {
    failure: None,
    cleanup: None,
    discarded: vec![Discarded::Schedule {
      raw: serde_json::json!("x"),
      reason: ScheduleError::Unparseable {
        raw: "x".to_owned(),
      },
    }],
    stderr: Captured::default(),
  };
  assert_eq!(Diagnostics::of(reported, &[]).state(), TrayState::Fault);
}

#[test]
fn an_undrawn_alone_raises_fault() {
  let undrawn = [Undrawn::ContentForm {
    form: ContentForm::Uri,
  }];
  let reported = Reported {
    failure: None,
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured::default(),
  };
  assert_eq!(
    Diagnostics::of(reported, &undrawn).state(),
    TrayState::Fault
  );
}

#[test]
fn a_refused_alone_raises_fault() {
  let refused = Refused::UnknownOption;
  assert_eq!(Diagnostics::refused(&refused).state(), TrayState::Fault);
}

// ---------------------------------------------------------------------
// 13d — the bound, at the bound. Three limits, each at limit-1/limit/limit+1.
// ---------------------------------------------------------------------

#[test]
fn a_line_one_under_the_bound_is_untouched() {
  let base = failure_line(0).chars().count();
  let target = LINE_BOUND - 1;
  let line = failure_line(target - base);
  assert_eq!(line.chars().count(), target);
  assert!(!line.contains("more characters not shown"));
}

#[test]
fn a_line_at_the_bound_is_untouched() {
  let base = failure_line(0).chars().count();
  let target = LINE_BOUND;
  let line = failure_line(target - base);
  assert_eq!(line.chars().count(), target);
  assert!(!line.contains("more characters not shown"));
}

#[test]
fn a_line_one_past_the_bound_is_truncated_naming_one_elided_character() {
  let base = failure_line(0).chars().count();
  let target = LINE_BOUND + 1;
  let line = failure_line(target - base);
  assert!(line.ends_with(" [1 more characters not shown]"), "{line:?}");
  let kept: String = line.chars().take(LINE_BOUND).collect();
  assert_eq!(kept.chars().count(), LINE_BOUND);
}

#[test]
fn a_stderr_line_one_under_the_bound_is_untouched() {
  let base = stderr_line_base();
  let target = STDERR_BOUND - 1;
  let line = stderr_line(target - base);
  assert_eq!(line.chars().count(), target);
  assert!(!line.contains("more characters not shown"));
}

#[test]
fn a_stderr_line_at_the_bound_is_untouched() {
  let base = stderr_line_base();
  let target = STDERR_BOUND;
  let line = stderr_line(target - base);
  assert_eq!(line.chars().count(), target);
  assert!(!line.contains("more characters not shown"));
}

#[test]
fn a_stderr_line_one_past_the_bound_is_truncated_naming_one_elided_character() {
  let base = stderr_line_base();
  let target = STDERR_BOUND + 1;
  let line = stderr_line(target - base);
  assert!(line.ends_with(" [1 more characters not shown]"), "{line:?}");
  let kept: String = line.chars().take(STDERR_BOUND).collect();
  assert_eq!(kept.chars().count(), STDERR_BOUND);
}

#[test]
fn a_tooltip_one_under_the_bound_is_untouched() {
  let base = failure_line(0).chars().count();
  let target = TOOLTIP_BOUND - 1;
  let diagnostics = {
    let reported = Reported {
      failure: Some(no_outstanding_view(target - base)),
      cleanup: None,
      discarded: Vec::new(),
      stderr: Captured::default(),
    };
    Diagnostics::of(reported, &[])
  };
  let tip = tooltip(&diagnostics, false);
  let summary = tip.strip_prefix("goad — ").expect("the tooltip prefix");
  assert_eq!(summary.chars().count(), target, "{tip:?}");
  assert!(!summary.contains("more characters not shown"));
}

#[test]
fn a_tooltip_at_the_bound_is_untouched() {
  let base = failure_line(0).chars().count();
  let target = TOOLTIP_BOUND;
  let diagnostics = {
    let reported = Reported {
      failure: Some(no_outstanding_view(target - base)),
      cleanup: None,
      discarded: Vec::new(),
      stderr: Captured::default(),
    };
    Diagnostics::of(reported, &[])
  };
  let tip = tooltip(&diagnostics, false);
  let summary = tip.strip_prefix("goad — ").expect("the tooltip prefix");
  assert_eq!(summary.chars().count(), target, "{tip:?}");
  assert!(!summary.contains("more characters not shown"));
}

#[test]
fn a_tooltip_one_past_the_bound_is_truncated_naming_one_elided_character() {
  let base = failure_line(0).chars().count();
  let target = TOOLTIP_BOUND + 1;
  let diagnostics = {
    let reported = Reported {
      failure: Some(no_outstanding_view(target - base)),
      cleanup: None,
      discarded: Vec::new(),
      stderr: Captured::default(),
    };
    Diagnostics::of(reported, &[])
  };
  let tip = tooltip(&diagnostics, false);
  assert!(tip.ends_with(" [1 more characters not shown]"), "{tip:?}");
}

// ---------------------------------------------------------------------
// 13e — escaping and decoding.
// ---------------------------------------------------------------------

#[test]
fn non_utf8_stderr_decodes_lossily() {
  let reported = Reported {
    failure: None,
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured {
      bytes: vec![0xFF, 0xFE],
      truncated: false,
    },
  };
  let line = Diagnostics::of(reported, &[]).lines()[0].clone();
  assert!(line.starts_with("stderr: "), "{line:?}");
  assert!(line.contains('\u{FFFD}'), "{line:?}");
}

#[test]
fn a_newline_and_a_backslash_in_stderr_are_escaped_by_name() {
  let reported = Reported {
    failure: None,
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured {
      bytes: b"a\\b\nc".to_vec(),
      truncated: false,
    },
  };
  let line = Diagnostics::of(reported, &[]).lines()[0].clone();
  assert_eq!(line, "stderr: a\\\\b\\nc");
}

#[test]
fn non_latin_text_and_combining_marks_pass_verbatim() {
  let source = "héllo мир 日本語 e\u{0301}"; // e + combining acute accent
  let reported = Reported {
    failure: None,
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured {
      bytes: source.as_bytes().to_vec(),
      truncated: false,
    },
  };
  let line = Diagnostics::of(reported, &[]).lines()[0].clone();
  assert_eq!(line, format!("stderr: {source}"));
}

#[test]
fn a_multi_line_markdown_error_becomes_one_line() {
  // A heading and a block quote in the same source trip two distinct
  // `StyledTextFromMarkdownError` entries, joined with a real `\n`
  // (i-slint-core-1.17.1/styled_text.rs:24-27) — the ordinary path this
  // escaping exists for, not only a hostile one.
  let source = "# Heading\n\n> a quote";
  let presentation = goad::view_model::present(&choice_with_markdown(source));
  let detail = match presentation.undrawn.as_slice() {
    [Undrawn::MarkdownUnsupported { detail }] => detail.clone(),
    other => panic!("expected exactly one MarkdownUnsupported, got {other:?}"),
  };
  assert!(detail.contains('\n'), "fixture assumption: {detail:?}");

  let reported = Reported {
    failure: None,
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured::default(),
  };
  let undrawn = presentation.undrawn;
  let line = Diagnostics::of(reported, &undrawn).lines()[0].clone();
  assert!(!line.contains('\n'), "{line:?}");
  assert!(line.contains("\\n"), "{line:?}");
}

// ---------------------------------------------------------------------
// 13f — ordering of decode, escape and bound.
// ---------------------------------------------------------------------

#[test]
fn stderr_within_the_byte_bound_but_over_the_escaped_bound_is_truncated() {
  // Every byte is a backslash, which escapes to two characters. The raw
  // length is well under `STDERR_BOUND`; the escaped length is not. This
  // is the assertion that fails if the bound is applied to bytes.
  let raw_len = STDERR_BOUND - 100;
  assert!(raw_len < STDERR_BOUND);
  let reported = Reported {
    failure: None,
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured {
      bytes: vec![b'\\'; raw_len],
      truncated: false,
    },
  };
  let line = Diagnostics::of(reported, &[]).lines()[0].clone();
  assert!(line.contains("more characters not shown"), "{line:?}");
}

// ---------------------------------------------------------------------
// 13g — once, exactly.
// ---------------------------------------------------------------------

#[test]
fn a_not_a_string_discard_names_the_raw_value_once() {
  let discarded = Discarded::Schedule {
    raw: serde_json::json!("18:00:00"),
    reason: ScheduleError::NotAString { found: "number" },
  };
  let rendered = discarded.to_string();
  assert_eq!(rendered.matches("18:00:00").count(), 1, "{rendered:?}");
}

#[test]
fn a_raw_carrying_discard_names_the_raw_value_once() {
  let discarded = Discarded::Schedule {
    raw: serde_json::json!("18:00:00"),
    reason: ScheduleError::TimeOfDay {
      raw: "18:00:00".to_owned(),
    },
  };
  let rendered = discarded.to_string();
  assert_eq!(rendered.matches("18:00:00").count(), 1, "{rendered:?}");
}

#[test]
fn a_backend_io_failure_renders_the_os_message_once_and_not_again_from_source() {
  let io_error = std::io::Error::other("goad-test-marker-boom");
  let failure = Failure::Backend(BackendError::Io(io_error));
  let reported = Reported {
    failure: Some(failure),
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured::default(),
  };
  let line = Diagnostics::of(reported, &[]).lines()[0].clone();
  assert_eq!(line.matches("goad-test-marker-boom").count(), 1, "{line:?}");
}

// ---------------------------------------------------------------------
// 13h — the two truncations are distinguishable.
// ---------------------------------------------------------------------

#[test]
fn a_capture_truncated_and_display_truncated_stderr_produce_both_lines_in_order() {
  let reported = Reported {
    failure: None,
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured {
      bytes: vec![b'x'; STDERR_BOUND + 10],
      truncated: true,
    },
  };
  let lines = Diagnostics::of(reported, &[]).lines().to_vec();
  assert_eq!(lines.len(), 2, "{lines:#?}");
  assert_eq!(lines[0], CAPTURE_TRUNCATED);
  assert!(lines[1].starts_with("stderr: "));
  assert!(lines[1].contains("more characters not shown"));
  assert_ne!(lines[0], lines[1]);
}

// ---------------------------------------------------------------------
// 13i — the tooltip, all four forms.
// ---------------------------------------------------------------------

#[test]
fn the_tooltip_has_four_forms() {
  let clear = Diagnostics::default();
  assert_eq!(tooltip(&clear, false), "goad — nothing to show");
  assert_eq!(tooltip(&clear, true), "goad — waiting for an answer");

  let one_line = Diagnostics::refused(&Refused::UnknownOption);
  assert_eq!(
    tooltip(&one_line, false),
    "goad — no action taken: the host could not match that control to the question it is holding"
  );

  let two_lines = {
    let reported = Reported {
      failure: Some(no_outstanding_view(1)),
      cleanup: Some(CleanupFailure::TimedOut {
        after: Duration::from_millis(1),
      }),
      discarded: Vec::new(),
      stderr: Captured::default(),
    };
    Diagnostics::of(reported, &[])
  };
  let tip = tooltip(&two_lines, false);
  assert!(tip.starts_with("goad — no action taken:"), "{tip:?}");
  assert!(tip.ends_with(" (+1 more)"), "{tip:?}");
}

#[test]
fn the_tooltip_summary_is_a_120_character_projection_of_line_zero() {
  let base = failure_line(0).chars().count();
  let named_len = (TOOLTIP_BOUND + 50) - base;
  let reported = Reported {
    failure: Some(no_outstanding_view(named_len)),
    cleanup: None,
    discarded: Vec::new(),
    stderr: Captured::default(),
  };
  let diagnostics = Diagnostics::of(reported, &[]);
  let full_line = diagnostics.lines()[0].clone();
  assert!(full_line.chars().count() > TOOLTIP_BOUND);

  let tip = tooltip(&diagnostics, false);
  assert!(tip.ends_with(" [50 more characters not shown]"), "{tip:?}");
  // The projection is of the reducer's own line, not a re-derivation from
  // the outcome: it agrees with the full line's first TOOLTIP_BOUND chars.
  let prefix: String = full_line.chars().take(TOOLTIP_BOUND).collect();
  assert!(tip.contains(&prefix), "{tip:?}");
}

// ---------------------------------------------------------------------
// 13j — `Diagnostics::refused`.
// ---------------------------------------------------------------------

#[test]
fn every_refused_variant_renders_one_line_with_the_failure_prefix() {
  let superseded = Diagnostics::refused(&Refused::SupersededView);
  assert_eq!(
    superseded.lines().to_vec(),
    vec![
      "no action taken: that answer belongs to a question that has since been replaced".to_owned()
    ]
  );

  let unknown = Diagnostics::refused(&Refused::UnknownOption);
  assert_eq!(
    unknown.lines().to_vec(),
    vec![
      "no action taken: the host could not match that control to the question it is holding"
        .to_owned()
    ]
  );

  let unknown_field = Diagnostics::refused(&Refused::UnknownField);
  assert_eq!(
    unknown_field.lines().to_vec(),
    vec![
      "no action taken: the host could not match that control to a field of the option it names"
        .to_owned()
    ]
  );
  assert_ne!(
    unknown_field.lines(),
    unknown.lines(),
    "the two selectors fail differently and a person must be able to tell which did"
  );

  let no_clock = Diagnostics::refused(&Refused::NoClock {
    detail: "clock error".to_owned(),
  });
  assert_eq!(
    no_clock.lines().to_vec(),
    vec![
      "no action taken: the system clock could not be read, so no request could be stamped (clock error)"
        .to_owned()
    ]
  );

  // The fifth variant, and the one this case's **name** has claimed since
  // before it had four. `reason` is a wire token off `Refusal::reason()` and
  // `detail` is the prose off its `Display`; this module holds neither
  // vocabulary and renders both (`SPEC-003/R-15`).
  let ingress = Diagnostics::refused(&Refused::Ingress {
    reason: "too_soon".to_owned(),
    detail: "the event spacing has not elapsed".to_owned(),
  });
  assert_eq!(
    ingress.lines().to_vec(),
    vec![
      "no action taken: an event was refused (too_soon): the event spacing has not elapsed"
        .to_owned()
    ]
  );
}

// ---------------------------------------------------------------------
// 13k — `ContentForm`, and an HTML body reaching `Body::Plain` intact.
// ---------------------------------------------------------------------

#[test]
fn content_form_renders_html_and_a_uri() {
  assert_eq!(ContentForm::Html.to_string(), "HTML");
  assert_eq!(ContentForm::Uri.to_string(), "a URI");
}

#[test]
fn an_html_body_reaches_the_glass_through_receive_with_the_bytes_the_backend_sent() {
  let view = choice_with_body(Some(
    serde_json::json!({ "kind": "html", "value": "<b>hi</b>" }),
  ));
  let received = receive(outcome_with_view(view));
  let prepared = received.prepared.expect("a view was carried");
  assert_eq!(
    prepared.presentation.body,
    Body::Plain("<b>hi</b>".to_owned())
  );
}

// ---------------------------------------------------------------------
// 13m — AC-3: a field this renderer cannot draw reaches the surface.
// ---------------------------------------------------------------------

/// VT-6 — I-2, end to end through the only path that produces a
/// `Presentation`. `receive` calls `present` and hands the resulting
/// `undrawn` to `Diagnostics::of` in the same expression, so a view carrying
/// an undrawn field cannot reach the glass without the surface saying so.
///
/// The view is **still shown** and the option still answerable: `SPEC-001`
/// R-55 forbids refusing a view over a capability this renderer lacks. What
/// the wording of each line is stays review's, as every diagnostic wording in
/// this project is; what is asserted here is that one line arrives per field
/// and names it.
#[test]
fn a_view_carrying_an_undrawn_field_reaches_the_diagnostic_surface_through_receive() {
  let view = view_from(&serde_json::json!({
    "view": {
      "kind": "choice",
      "title": "T",
      "options": [{
        "id": "opt",
        "label": "Fine",
        "fields": [
          { "id": "note", "kind": "text", "label": "Note" },
          { "id": "counted", "kind": "boolean", "label": "Counted", "group": 7 }
        ]
      }]
    }
  }));
  let received = receive(outcome_with_view(view));

  let prepared = received.prepared.expect("the view is shown, not refused");
  assert_eq!(
    prepared.presentation.options.len(),
    1,
    "the option is still there to answer"
  );
  assert!(
    !prepared.presentation.body_is_degraded(),
    "neither field variant says anything about the body"
  );

  let lines = received.diagnostics.lines();
  assert_eq!(lines.len(), 2, "one line per field, and nothing else");
  assert!(
    lines[0].contains("note"),
    "the undrawn field is named: {:?}",
    lines[0]
  );
  assert!(
    lines[1].contains("counted"),
    "the unhonoured hint names its field: {:?}",
    lines[1]
  );
  assert!(!received.diagnostics.is_clear());
}

// ---------------------------------------------------------------------
// 13l — markdown is parsed once.
// ---------------------------------------------------------------------

#[test]
fn a_rejected_body_is_plain_and_degraded_an_accepted_body_is_rich_and_not() {
  let rejected = receive(outcome_with_view(choice_with_markdown("\u{e541}")))
    .prepared
    .expect("a view was carried")
    .presentation;
  assert_eq!(rejected.body, Body::Plain("\u{e541}".to_owned()));
  assert!(rejected.body_is_degraded());

  let accepted = receive(outcome_with_view(choice_with_markdown("**bold**")))
    .prepared
    .expect("a view was carried")
    .presentation;
  assert!(!accepted.body_is_degraded());

  let text = receive(outcome_with_view(choice_with_body(Some(
    serde_json::json!({ "kind": "text", "value": "hi" }),
  ))))
  .prepared
  .expect("a view was carried")
  .presentation;
  assert!(!text.body_is_degraded());

  let html = receive(outcome_with_view(choice_with_body(Some(
    serde_json::json!({ "kind": "html", "value": "<b>hi</b>" }),
  ))))
  .prepared
  .expect("a view was carried")
  .presentation;
  assert!(html.body_is_degraded());

  let uri = receive(outcome_with_view(choice_with_body(Some(
    serde_json::json!({ "kind": "uri", "value": "https://example.invalid/x" }),
  ))))
  .prepared
  .expect("a view was carried")
  .presentation;
  assert!(uri.body_is_degraded());
}

// ---------------------------------------------------------------------
// `BUSY_NOTICE` and `receive`'s remaining fields — cheap coverage that does
// not belong to any one item above.
// ---------------------------------------------------------------------

#[test]
fn busy_notice_is_the_exact_string() {
  assert_eq!(
    BUSY_NOTICE,
    "still working on the last request — try again in a moment"
  );
}

#[test]
fn receive_carries_next_check_and_refused_through_untouched() {
  let mut outcome = bare_outcome();
  outcome.failure = Some(no_outstanding_view(1));
  let received = receive(outcome);
  assert!(received.refused);
  assert_eq!(received.next_check, now());
  assert!(received.prepared.is_none());
}
