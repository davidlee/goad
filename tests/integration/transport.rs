//! The paths that work, and the failures this transport can reach.
//!
//! Every case here runs a real process, so every timeout is this case's own
//! rather than a shared constant: the failure cases want one short enough that
//! the suite stays fast, and the success cases want one long enough that a
//! healthy exchange cannot flake.

use std::time::{Duration, Instant};

use crate::harness::{describe, describe_cleanup, evaluate, padded_evaluate, stderr, transport};
use goad::shell::backend::process::ProcessBackend;
use goad::shell::backend::transport::Backend;
use goad::shell::error::BackendError;

/// The transport's own cleanup budget, restated because it is private to
/// `process.rs` and these bounds are about it. If it changes there, VA-3's
/// assertions below are wrong until this does too.
const CLEANUP_LIMIT: Duration = Duration::from_millis(500);

/// VA-3's slack, stated rather than buried in a widened bound. The probe
/// measured the grandchild case at 902 ms against a 900 ms bound, of which 2 ms
/// was scheduling; `cargo test` runs these cases in parallel, so the machine is
/// loaded while they run.
const SLACK: Duration = Duration::from_millis(500);

// ---------------------------------------------------------------------------
// VT-1 — the normal exchange
// ---------------------------------------------------------------------------

/// A correct backend: the request arrives whole, the response comes back, the
/// exit is clean, and there is nothing left to dispose of.
///
/// This is also the case that proves the host closes stdin. The script reads to
/// EOF, so a host that held the pipe open would hang here and the symptom would
/// be a timeout that looks like a slow backend (R-37).
#[tokio::test]
async fn a_correct_backend_completes_an_exchange() {
  let request = evaluate();
  let mut backend = transport("reads-stdin-then-answers", Duration::from_secs(5));

  let started = Instant::now();
  let exchange = backend.exchange(&request).await;
  let elapsed = started.elapsed();

  let body = exchange.result.as_ref().expect("a correct backend answers");
  assert_eq!(String::from_utf8_lossy(body).trim(), r#"{"view":null}"#);
  // The script echoes back what it read, so this is the request arriving
  // verbatim rather than merely something arriving.
  assert_eq!(
    stderr(&exchange),
    serde_json::to_string(&request).expect("a request serializes"),
  );
  assert!(!exchange.stderr.truncated);
  assert!(
    exchange.cleanup.is_none(),
    "{}",
    describe_cleanup(exchange.cleanup.as_ref())
  );
  // VA-3, the half that catches a structure paying for disposal it does not
  // need: a prompt success must not cost the cleanup budget. The probe measured
  // 2.5 ms against this 500 ms bound.
  assert!(
    elapsed < CLEANUP_LIMIT,
    "a prompt success took {}ms, which is the cleanup budget's worth",
    elapsed.as_millis()
  );
}

// ---------------------------------------------------------------------------
// VT-2 — the timeout, and what it does not imply
// ---------------------------------------------------------------------------

/// A backend that never answers: `Timeout`, the child gone, and — the part
/// worth stating — `cleanup: None`.
///
/// A timeout does not imply a cleanup failure. This backend hangs with nothing
/// holding its pipes, so the kill and the reap finish well inside the budget;
/// the probe measured 601 ms against a 600 ms timeout, not 901 ms. Ignoring
/// `cleanup` here would let a regression that fails every disposal pass.
#[tokio::test]
async fn a_backend_that_never_answers_times_out_and_is_disposed_of() {
  let timeout = Duration::from_millis(300);
  let mut backend = transport("hangs-past-the-timeout", timeout);

  let started = Instant::now();
  let exchange = backend.exchange(&evaluate()).await;
  let elapsed = started.elapsed();

  match &exchange.result {
    Err(BackendError::Timeout { after }) => assert_eq!(*after, timeout),
    other => panic!("expected a timeout, got {}", describe(other)),
  }
  assert!(
    exchange.cleanup.is_none(),
    "{}",
    describe_cleanup(exchange.cleanup.as_ref())
  );
  // R-41's other half, independent of the host's own report: the script wrote
  // its pid, and bash execs its last command, so that pid is the process the
  // host killed.
  let pid = stderr(&exchange).trim().to_owned();
  assert!(!pid.is_empty(), "the script wrote no pid");
  assert!(!alive(&pid), "process {pid} survived the exchange");
  // VA-3. The call waits for the timeout it was given, and at most that plus
  // the cleanup budget.
  assert!(
    elapsed >= timeout,
    "returned early, in {}ms",
    elapsed.as_millis()
  );
  assert!(
    elapsed < timeout + CLEANUP_LIMIT + SLACK,
    "the timeout path took {}ms",
    elapsed.as_millis()
  );
}

/// Is a pid still live? `kill -0` signals nothing and only reports whether it
/// could have.
fn alive(pid: &str) -> bool {
  std::process::Command::new("kill")
    .args(["-0", pid])
    .stderr(std::process::Stdio::null())
    .status()
    .expect("kill is on PATH")
    .success()
}

// ---------------------------------------------------------------------------
// VT-3 — one case per variant this transport can reach
// ---------------------------------------------------------------------------
//
// `Timeout` is the case above. `Protocol` is not reachable here — the transport
// returns bytes and parses nothing, so it moved to PHASE-07 with R-38's framing
// rule. `OutputTooLarge` is PHASE-06's bound, and `PipeMissing` is not
// reachable from outside at all: it is a stdio handle the host itself asked for
// going missing after a successful spawn.

/// `Spawn`. No fixture: a path that does not exist is the whole case, and
/// nothing is spawned, so there is nothing to have cleaned up after.
#[tokio::test]
async fn a_command_that_does_not_exist_fails_to_spawn() {
  let mut backend = ProcessBackend::new(
    vec!["./no-such-backend-exists".to_owned()],
    Duration::from_secs(5),
  );

  let exchange = backend.exchange(&evaluate()).await;

  match &exchange.result {
    Err(BackendError::Spawn(error)) => {
      assert_eq!(error.kind(), std::io::ErrorKind::NotFound, "{error}");
    }
    other => panic!("expected a spawn failure, got {}", describe(other)),
  }
  assert!(
    exchange.cleanup.is_none(),
    "{}",
    describe_cleanup(exchange.cleanup.as_ref())
  );
  assert_eq!(stderr(&exchange), "", "nothing ran, so nothing wrote");
}

/// `ExitStatus`, and the discard that comes with it. The body parsed; the exit
/// status disclaims it, so it does not reach the caller (D15, R-40). The stderr
/// does.
#[tokio::test]
async fn a_non_zero_exit_discards_the_body_it_came_with() {
  let mut backend = transport("answers-then-exits-non-zero", Duration::from_secs(5));

  let exchange = backend.exchange(&evaluate()).await;

  match &exchange.result {
    Err(BackendError::ExitStatus { code }) => assert_eq!(*code, Some(1)),
    other => panic!("expected a non-zero exit, got {}", describe(other)),
  }
  assert!(
    stderr(&exchange).contains("not to be trusted"),
    "stderr was {}",
    stderr(&exchange).escape_debug()
  );
  assert!(
    exchange.cleanup.is_none(),
    "{}",
    describe_cleanup(exchange.cleanup.as_ref())
  );
}

/// `Io`, by the only deterministic route there is: the request must still be
/// being written when the read end closes.
///
/// A request that fits the pipe buffer is accepted by the kernel and outlives
/// the reader, so a backend exiting before reading produces a perfectly normal
/// exchange — measured, 20/20 either way. So the payload is padded past the
/// buffer, which `Event.data` permits because it is opaque to the host (R-9).
#[tokio::test]
async fn a_backend_that_exits_before_reading_breaks_the_pipe() {
  let request = padded_evaluate(&"x".repeat(1024 * 1024));
  let mut backend = transport("exits-without-reading-stdin", Duration::from_secs(5));

  let exchange = backend.exchange(&request).await;

  match &exchange.result {
    Err(BackendError::Io(error)) => {
      assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe, "{error}");
    }
    other => panic!("expected a broken pipe, got {}", describe(other)),
  }
  assert!(
    exchange.cleanup.is_none(),
    "{}",
    describe_cleanup(exchange.cleanup.as_ref())
  );
}

// ---------------------------------------------------------------------------
// VT-4 / EX-4 — stderr is carried on every path this phase reaches
// ---------------------------------------------------------------------------

/// The timeout path (F-3, R-42). Not covered by VT-2, which asserts the
/// disposal rather than the diagnostic: this is the reversal of D18, where the
/// capture was owned by the future the timeout drops and so went with it.
#[tokio::test]
async fn stderr_written_before_a_hang_survives_the_timeout() {
  let mut backend = transport("writes-stderr-then-hangs", Duration::from_millis(300));

  let exchange = backend.exchange(&evaluate()).await;

  assert!(
    matches!(exchange.result, Err(BackendError::Timeout { .. })),
    "expected a timeout, got {}",
    describe(&exchange.result)
  );
  assert_eq!(stderr(&exchange).trim(), "backend about to hang");
}

/// The path stderr exists for (F-24): a clean exit, a body that will not parse,
/// and the reason already on stderr.
///
/// The claim here is about the **stderr**, not the parse. This transport hands
/// the bytes on unparsed — `result` is `Ok` — and the rejection happens where
/// `from_slice` runs, which is PHASE-07. What is asserted is that the bytes do
/// not parse and the explanation arrived anyway.
#[tokio::test]
async fn a_zero_exit_with_an_unparseable_body_still_carries_its_stderr() {
  let mut backend = transport("exits-zero-with-unparseable-stdout", Duration::from_secs(5));

  let exchange = backend.exchange(&evaluate()).await;

  let body = exchange
    .result
    .as_ref()
    .expect("a zero exit is not a transport failure");
  serde_json::from_slice::<serde_json::Value>(body)
    .expect_err("the fixture's whole point is that this does not parse");
  assert!(
    stderr(&exchange).contains("config is missing"),
    "stderr was {}",
    stderr(&exchange).escape_debug()
  );
  assert!(
    exchange.cleanup.is_none(),
    "{}",
    describe_cleanup(exchange.cleanup.as_ref())
  );
}
