//! The run: the request plan, the answers, the report and the verdict
//! (`design.md` §5.2.2, §5.2.5).
//!
//! Every backend here is `tests/backends/`' through `scripting`: a scripted
//! one answers one instruction per invocation, in the plan's order — the
//! three host kinds' evaluates, the R-56 probe's, then each event file's,
//! with each view's responds after the exchange that returned it — and past
//! its list it behaves.
use std::path::Path;
use std::process::Output;

use goad_semantics::protocol::canonical::{HOST_SOURCE, Stimulus};
use serde_json::Value;

use crate::process::{
  against, assert_last_stderr_line_is_the_checker_s, check, code_of, fixture, stderr_of, stdout_of,
};
use crate::scripting;

/// What `answers-as-instructed.sh` answers past its list: a conforming
/// evaluate.
const NOTHING: &str = r#"{"view":null,"next_check":"45 minutes"}"#;

/// A view with one option and no fields: `@slow-view`'s body, without the
/// delay.
const A_VIEW: &str = r#"{"view":{"kind":"choice","title":"Still there?","options":[{"id":"ok","label":"OK"}]},"next_check":"45 minutes"}"#;

/// The sentinel for an exchange whose backend exits 1 (`backend`, R-40), and
/// the stderr line it writes.
const EXIT_1: &str = "@exit1";
const EXIT_1_STDERR: &str = "that answer is not to be trusted";

/// The line `render` writes when no exchange returned a view.
const NO_VIEW: &str = "no exchange returned a view, so respond was not exercised";

/// The R-56 probe's kind, as the backend receives it (`design.md` §5.2.2).
const PROBE: &str = "goad-check-unrecognised";

/// A line naming both `side` and `requirement` — a refusal line, or the R-56
/// claim. A line, not the whole report: two lines each naming one would
/// otherwise pass.
fn a_line_names(stdout: &str, side: &str, requirement: &str) -> bool {
  stdout
    .lines()
    .any(|line| line.contains(side) && line.contains(requirement))
}

fn assert_refused(output: &Output) {
  assert_eq!(code_of(output), 1, "{}", report_of(output));
  assert_last_stderr_line_is_the_checker_s(output);
}

/// The report and stderr together, for a failure message.
fn report_of(output: &Output) -> String {
  format!(
    "stdout:\n{}\nstderr:\n{}",
    stdout_of(output),
    stderr_of(output)
  )
}

/// Each request a logging backend received, parsed.
fn requests(log: &Path) -> Vec<Value> {
  std::fs::read_to_string(log)
    .expect("the backend must have logged its requests")
    .lines()
    .map(|line| serde_json::from_str(line).expect("each logged request is one JSON line"))
    .collect()
}

/// The `event.kind` of each evaluate, in the order the backend received them.
fn evaluated_kinds(requests: &[Value]) -> Vec<&str> {
  requests
    .iter()
    .filter(|request| request["type"] == "evaluate")
    .map(|request| request["event"]["kind"].as_str().unwrap_or_default())
    .collect()
}

/// A logging backend that answers `instructions` in turn, then `{"view":null}`.
fn logging(case: &str, instructions: &[&str]) -> (goad_shell::config::Command, std::path::PathBuf) {
  let (mut backend, log) = scripting::logging_backend("logs-the-request-then-answers", case);
  backend.arguments.extend(
    instructions
      .iter()
      .map(|instruction| (*instruction).to_owned()),
  );
  (backend, log)
}

#[test]
fn a_conforming_backend_is_accepted_and_exits_0() {
  let (backend, log) = scripting::scripted("conforming", &[A_VIEW]);
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_eq!(code_of(&output), 0, "{}", report_of(&output));
  assert_eq!(
    scripting::invocations(&log),
    5,
    "four evaluates and one respond"
  );
  assert!(
    stdout
      .lines()
      .last()
      .unwrap_or_default()
      .starts_with("verdict: "),
    "{stdout}"
  );
  assert!(!stdout.contains(NO_VIEW), "a view was answered: {stdout}");
  assert!(!stdout.contains("SPEC-001/R-56"), "{stdout}");
  assert!(output.stderr.is_empty(), "{}", stderr_of(&output));
}

/// The probe alone fails, on the backend's side, and the known kinds do not:
/// the claim is made, beside the failure the host reported, and the backend's
/// own stderr is shown.
#[test]
fn a_backend_that_fails_on_an_unrecognised_host_kind_is_reported_against_r56() {
  let (backend, _log) = scripting::scripted("r56", &[NOTHING, NOTHING, NOTHING, EXIT_1]);
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_refused(&output);
  assert!(
    a_line_names(&stdout, "backend", "SPEC-001/R-56"),
    "{stdout}"
  );
  assert!(
    a_line_names(&stdout, "backend", "SPEC-001/R-40"),
    "{stdout}"
  );
  assert!(stdout.contains(EXIT_1_STDERR), "stderr verbatim: {stdout}");
}

/// `"options":[]` is `EmptyOptions` (backend, R-13).
#[test]
fn a_refused_view_is_reported_with_its_requirement_and_the_backend_side() {
  let (backend, _log) = scripting::scripted(
    "refused-view",
    &[r#"{"view":{"kind":"choice","title":"Nothing to pick","options":[]}}"#],
  );
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_refused(&output);
  assert!(
    a_line_names(&stdout, "backend", "SPEC-001/R-13"),
    "{stdout}"
  );
}

/// A time of day is `ScheduleError::TimeOfDay` (backend, R-21), discarded
/// without failing the exchange. It is the run's only refusal, so the status
/// cut is read at one.
#[test]
fn a_discarded_next_check_is_reported_and_exits_1() {
  let (backend, _log) =
    scripting::scripted("discarded", &[r#"{"view":null,"next_check":"18:00:00"}"#]);
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_refused(&output);
  assert!(
    a_line_names(&stdout, "backend", "SPEC-001/R-21"),
    "{stdout}"
  );
  assert_eq!(
    stdout.matches("SPEC-001/").count(),
    1,
    "exactly one refusal: {stdout}"
  );
}

/// Every kind fails to spawn, on the configuration's side, so the probe's
/// failure is no different from the rest and is not charged with R-56.
#[test]
fn an_unspawnable_command_is_reported_against_the_configuration() {
  let absent = fixture("no-such-program");
  let output = check(&["--".to_owned(), absent.display().to_string()]);

  let stdout = stdout_of(&output);
  assert_refused(&output);
  assert!(
    a_line_names(&stdout, "configuration", "SPEC-001/R-44"),
    "{stdout}"
  );
  assert!(!stdout.contains("SPEC-001/R-56"), "{stdout}");
}

#[test]
fn a_backend_failing_identically_on_every_kind_is_not_charged_with_r56() {
  let (backend, _log) = scripting::scripted("identical", &[EXIT_1, EXIT_1, EXIT_1, EXIT_1]);
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_refused(&output);
  assert!(
    a_line_names(&stdout, "backend", "SPEC-001/R-40"),
    "{stdout}"
  );
  assert!(!stdout.contains("SPEC-001/R-56"), "{stdout}");
}

/// The probe's evaluate returns a view, and the respond that answers it
/// fails. A respond answers the backend's own view, not the unrecognised
/// kind, so it is judged as any other exchange and not charged with R-56.
#[test]
fn a_failure_in_the_probe_s_chain_is_not_charged_with_r56() {
  let (backend, log) =
    scripting::scripted("probe-chain", &[NOTHING, NOTHING, NOTHING, A_VIEW, EXIT_1]);
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_refused(&output);
  assert_eq!(
    scripting::invocations(&log),
    5,
    "the probe's view was answered"
  );
  assert!(
    a_line_names(&stdout, "backend", "SPEC-001/R-40"),
    "{stdout}"
  );
  assert!(!stdout.contains("SPEC-001/R-56"), "{stdout}");
}

/// Every known kind's evaluate succeeds with a view whose respond fails, and
/// the probe's evaluate fails. The condition reads evaluates only, so each
/// known kind made no failure, and the probe is charged.
#[test]
fn a_known_kind_s_chain_failure_does_not_excuse_the_probe() {
  let (backend, log) = scripting::scripted(
    "known-chain",
    &[A_VIEW, EXIT_1, A_VIEW, EXIT_1, A_VIEW, EXIT_1, EXIT_1],
  );
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_refused(&output);
  assert_eq!(
    scripting::invocations(&log),
    7,
    "every chain's respond was made"
  );
  assert!(
    a_line_names(&stdout, "backend", "SPEC-001/R-56"),
    "{stdout}"
  );
}

/// A backend silent at the hour it is checked is accepted, and the report
/// says respond went unexercised, last before the verdict.
#[test]
fn a_backend_that_returns_no_view_is_accepted_and_says_respond_was_not_exercised() {
  let (backend, _log) = scripting::scripted("no-view", &[]);
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_eq!(code_of(&output), 0, "{}", report_of(&output));
  let lines: Vec<&str> = stdout.lines().collect();
  assert_eq!(
    lines.get(lines.len().saturating_sub(2)).copied(),
    Some(NO_VIEW),
    "{stdout}"
  );
}

/// Two views, then `null`: the evaluate's view and the first respond's are
/// each answered, and the chain ends where the backend ends it.
#[test]
fn a_chained_view_is_answered_until_null() {
  let (backend, log) = scripting::scripted("chained", &[A_VIEW, A_VIEW]);
  let output = check(&against(&backend, &[]));

  assert_eq!(code_of(&output), 0, "{}", report_of(&output));
  assert_eq!(
    scripting::invocations(&log),
    6,
    "four evaluates and two responds"
  );
}

/// A view on every answer for one request — the evaluate and eight responds
/// each return one: the chain bound stops it after the eighth respond,
/// says so, and is no refusal.
#[test]
fn a_chain_past_its_bound_is_reported_and_does_not_change_the_status() {
  let (backend, log) = scripting::scripted("past-the-bound", &[A_VIEW; 9]);
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_eq!(code_of(&output), 0, "{}", report_of(&output));
  assert_eq!(
    scripting::invocations(&log),
    12,
    "four evaluates and eight responds"
  );
  assert!(stdout.contains("chain bound"), "{stdout}");
}

/// The respond names the first option, and carries a value for exactly that
/// option's fields: not the second option's, and none skipped. The report
/// shows the values sent, by field.
#[test]
fn a_view_answered_carries_exactly_its_options_fields() {
  let view = r#"{"view":{"kind":"choice","title":"How long?","options":[
    {"id":"logged","label":"Log it","fields":[
      {"id":"minutes","kind":"number","label":"Minutes"},
      {"id":"note","kind":"text","label":"Note"}]},
    {"id":"skipped","label":"Skip","fields":[
      {"id":"reason","kind":"text","label":"Reason"}]}]}}"#
    .replace('\n', "");
  let (backend, log) = logging("options-fields", &[&view]);
  let output = check(&against(&backend, &[]));

  assert_eq!(code_of(&output), 0, "{}", report_of(&output));
  let requests = requests(&log);
  let responds: Vec<&Value> = requests
    .iter()
    .filter(|request| request["type"] == "respond")
    .collect();
  let [respond] = responds.as_slice() else {
    panic!("exactly one respond: {requests:?}");
  };
  assert_eq!(respond["response"]["option"], "logged");
  let keys: Vec<&str> = respond["response"]["values"]
    .as_object()
    .expect("values is an object")
    .keys()
    .map(String::as_str)
    .collect();
  assert_eq!(keys, ["minutes", "note"]);
  let stdout = stdout_of(&output);
  assert!(
    stdout
      .lines()
      .any(|line| line.contains("minutes") && line.contains("note")),
    "the values sent: {stdout}"
  );
}

/// The first exchange fails, and every planned exchange is still made, in
/// the plan's order, each from the host's own source (§5.4).
#[test]
fn a_backend_failing_at_startup_is_still_asked_the_rest() {
  let (backend, log) = logging("startup-fails", &["not JSON at all"]);
  let output = check(&against(&backend, &[]));

  assert_refused(&output);
  let requests = requests(&log);
  assert_eq!(
    evaluated_kinds(&requests),
    [
      Stimulus::Startup.kind(),
      Stimulus::Requested.kind(),
      Stimulus::Scheduled.kind(),
      PROBE
    ]
  );
  for request in &requests {
    assert_eq!(request["event"]["source"], HOST_SOURCE, "{request}");
  }
}

/// Each `--event` file's envelope, after the host's own and the probe, in
/// the order given.
#[test]
fn event_files_are_sent_in_the_order_given() {
  let first = fixture("first-given.json");
  let second = fixture("second-given.json");
  let (backend, log) = logging("events-in-order", &[]);
  let output = check(&against(&backend, &[&first, &second]));

  assert_eq!(code_of(&output), 0, "{}", report_of(&output));
  assert_eq!(
    evaluated_kinds(&requests(&log)),
    [
      Stimulus::Startup.kind(),
      Stimulus::Requested.kind(),
      Stimulus::Scheduled.kind(),
      PROBE,
      "first-given",
      "second-given"
    ]
  );
}

/// The first exchange answers and leaves a grandchild holding stderr past the
/// host's cleanup budget (`@lingers`): the exchange succeeds, and the cleanup
/// failure alone is a refusal (backend, R-48).
#[test]
fn a_cleanup_failure_alone_exits_1() {
  let (backend, _log) = scripting::scripted("cleanup-alone", &["@lingers"]);
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_refused(&output);
  assert!(
    a_line_names(&stdout, "backend", "SPEC-001/R-48"),
    "{stdout}"
  );
  assert_eq!(
    stdout.matches("SPEC-001/").count(),
    1,
    "the cleanup failure is the only refusal: {stdout}"
  );
}

/// Every exchange floods stderr past the host's bound and then answers: the
/// report flags each truncation, and a truncation is no refusal.
#[test]
fn a_truncated_stderr_is_flagged() {
  let backend = scripting::backend("floods-stderr-then-answers");
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_eq!(code_of(&output), 0, "{}", stderr_of(&output));
  assert!(
    stdout.contains("stderr truncated"),
    "{}",
    stderr_of(&output)
  );
}

/// The probe alone fails, on the configuration's side: the program deletes
/// itself on its third run, so the probe's spawn fails (`Spawn`). The host's
/// own kinds made no failure, so only the side clause keeps the probe from
/// being charged with R-56.
#[test]
fn a_probe_failure_on_another_side_is_not_charged_with_r56() {
  let program = scripting::marker("deletes-itself-program");
  let log = scripting::marker("deletes-itself-log");
  std::fs::copy(fixture("deletes-itself-on-its-third-run.sh"), &program)
    .expect("the fixture must be copyable to the temp directory");
  let mut permissions = std::fs::metadata(&program)
    .expect("the copy must exist")
    .permissions();
  std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
  std::fs::set_permissions(&program, permissions).expect("the copy must be made executable");
  let backend = goad_shell::config::Command::new(
    program.display().to_string(),
    vec![log.display().to_string()],
  );
  let output = check(&against(&backend, &[]));

  let stdout = stdout_of(&output);
  assert_refused(&output);
  assert_eq!(scripting::invocations(&log), 3, "the host's own kinds ran");
  assert!(
    a_line_names(&stdout, "configuration", "SPEC-001/R-44"),
    "{stdout}"
  );
  assert!(!stdout.contains("SPEC-001/R-56"), "{stdout}");
}
