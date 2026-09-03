//! The paths that work, and the failures this transport can reach.
//!
//! Every case here runs a real process, so every timeout is this case's own
//! rather than a shared constant: the failure cases want one short enough that
//! the suite stays fast, and the success cases want one long enough that a
//! healthy exchange cannot flake.

use std::time::{Duration, Instant};

use crate::harness::{
  self, alive, describe, describe_cleanup, evaluate, padded_evaluate, reported_pid, stderr,
  transport,
};
use goad::shell::backend::process::ProcessBackend;
use goad::shell::backend::transport::Backend;
use goad::shell::error::{BackendError, CleanupFailure};

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
  let pid = reported_pid(&exchange);
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

// ---------------------------------------------------------------------------
// PHASE-06 — the two bounds, disposal, and the two grandchild cases
// ---------------------------------------------------------------------------

/// The transport's own bounds, restated for the same reason `CLEANUP_LIMIT` is:
/// they are private to `process.rs`, and the assertions below are about them.
const STDOUT_LIMIT: usize = 8 * 1024 * 1024;
const STDERR_LIMIT: usize = 256 * 1024;

/// EX-1 — the stdout bound fails the exchange, and the **backend** is what
/// observes the stream closing (R-43's verification row).
///
/// The observation cannot come back in band: the host kills the backend as soon
/// as the bound is hit, so anything the shell would write afterwards races a
/// `SIGKILL` it loses. So the flooder is a grandchild that outlives the kill and
/// writes a marker file when its write fails, and a second grandchild holds
/// stderr so that disposal stalls for the whole cleanup budget. That stall is
/// what makes the assertion a 500 ms window rather than a race: measured, the
/// marker lands 500 ms *before* the exchange returns when the reader owns the
/// stdout handle, and 1.8 ms *after* it when the reader merely borrows it.
///
/// This case also discharges VT-4: disposal that cannot complete inside the
/// budget reports `TimedOut` and the exchange **returns** rather than blocking.
#[tokio::test]
async fn a_stdout_flood_is_refused_and_the_backend_sees_the_stream_close() {
  let marker = harness::marker("broken-pipe");
  let mut command = harness::backend("floods-stdout-and-reports-the-broken-pipe");
  command.push(marker.display().to_string());
  let mut backend = ProcessBackend::new(command, Duration::from_secs(5));

  let started = Instant::now();
  let exchange = backend.exchange(&evaluate()).await;
  let elapsed = started.elapsed();
  let observed = marker.exists();
  harness::clear(&marker);

  match &exchange.result {
    Err(BackendError::OutputTooLarge { limit }) => assert_eq!(*limit, STDOUT_LIMIT),
    other => panic!("expected the stdout bound to fail, got {}", describe(other)),
  }
  assert!(
    observed,
    "the backend never saw the stream close, so the reader is holding the handle past the bound"
  );
  // VT-4: the budget elapsed and the exchange still returned.
  assert!(
    matches!(exchange.cleanup, Some(CleanupFailure::TimedOut { .. })),
    "{}",
    describe_cleanup(exchange.cleanup.as_ref())
  );
  assert!(
    elapsed >= CLEANUP_LIMIT && elapsed < CLEANUP_LIMIT + SLACK,
    "a stalled disposal took {}ms",
    elapsed.as_millis()
  );
  assert!(
    !alive(&reported_pid(&exchange)),
    "the backend survived the exchange"
  );
}

/// The other half of the cap path, which the case above cannot state because it
/// deliberately stalls disposal: a flood with nothing outliving the kill costs
/// **no** part of the cleanup budget. `exec`, so the flooder is the child.
#[tokio::test]
async fn a_stdout_flood_with_nothing_behind_it_is_disposed_of_cleanly() {
  let mut backend = transport("floods-stdout-past-the-cap", Duration::from_secs(5));

  let started = Instant::now();
  let exchange = backend.exchange(&evaluate()).await;
  let elapsed = started.elapsed();

  match &exchange.result {
    Err(BackendError::OutputTooLarge { limit }) => assert_eq!(*limit, STDOUT_LIMIT),
    other => panic!("expected the stdout bound to fail, got {}", describe(other)),
  }
  assert!(
    exchange.cleanup.is_none(),
    "{}",
    describe_cleanup(exchange.cleanup.as_ref())
  );
  assert!(
    elapsed < CLEANUP_LIMIT,
    "refusing a flood took {}ms, which is the cleanup budget's worth",
    elapsed.as_millis()
  );
  assert!(
    !alive(&reported_pid(&exchange)),
    "the flooding backend survived"
  );
}

/// EX-2 — the stderr bound truncates and keeps draining, and the exchange
/// **succeeds** (D34, F-25).
///
/// The asymmetry with the stdout bound is the whole of D34, and this is the case
/// that fails differently if it is lost: a reader that stopped at the bound
/// would leave the backend blocked on a full pipe with 300 KB still to write, so
/// the symptom would be a **hang** rather than a wrong value. The flood is past
/// the 64 KiB pipe buffer as well as past the 256 KiB bound, and the body is
/// reading stdout throughout — the deadlock needs both.
#[tokio::test]
async fn a_stderr_flood_is_truncated_and_the_exchange_still_succeeds() {
  let mut backend = transport("floods-stderr-then-answers", Duration::from_secs(5));

  let started = Instant::now();
  let exchange = backend.exchange(&evaluate()).await;
  let elapsed = started.elapsed();

  let body = exchange
    .result
    .as_ref()
    .expect("a chatty backend that answers is not a failed one");
  assert_eq!(String::from_utf8_lossy(body).trim(), r#"{"view":null}"#);
  assert!(exchange.stderr.truncated, "the flood was not flagged");
  assert_eq!(
    exchange.stderr.bytes.len(),
    STDERR_LIMIT,
    "the host kept a different amount than the bound"
  );
  assert!(
    exchange.cleanup.is_none(),
    "{}",
    describe_cleanup(exchange.cleanup.as_ref())
  );
  // A deadlock is the failure this case exists to catch, and a deadlocked
  // exchange returns at the timeout rather than never — so the bound is what
  // distinguishes the two.
  assert!(
    elapsed < CLEANUP_LIMIT,
    "draining 300 KB took {}ms, which is a stall rather than a read",
    elapsed.as_millis()
  );
  assert!(
    !alive(&reported_pid(&exchange)),
    "the chatty backend survived"
  );
}

// ---------------------------------------------------------------------------
// EX-4 / VT-3 — the two grandchild cases, which differ by one redirection
// ---------------------------------------------------------------------------

/// A grandchild holding **stderr** only: the response is delivered, the child is
/// reaped, and only the drain is stuck (F-48, F-53, F-63).
///
/// The timeout is long on purpose. This case must pay the **cleanup budget
/// alone**, so a 5-second timeout that is never approached is what proves the
/// exchange completed and only disposal failed — a bound of `timeout +
/// CLEANUP_LIMIT` would pass for a case that timed out as well.
#[tokio::test]
async fn a_grandchild_holding_stderr_costs_the_cleanup_budget_and_nothing_else() {
  let mut backend = transport("leaves-a-grandchild-holding-stderr", Duration::from_secs(5));

  let started = Instant::now();
  let exchange = backend.exchange(&evaluate()).await;
  let elapsed = started.elapsed();

  let body = exchange
    .result
    .as_ref()
    .expect("the backend answered before it left anything behind");
  assert_eq!(String::from_utf8_lossy(body).trim(), r#"{"view":null}"#);
  match &exchange.cleanup {
    Some(CleanupFailure::TimedOut { after }) => assert_eq!(*after, CLEANUP_LIMIT),
    other => panic!(
      "expected a cleanup timeout, got {}",
      describe_cleanup(other.as_ref())
    ),
  }
  assert!(
    elapsed >= CLEANUP_LIMIT && elapsed < CLEANUP_LIMIT + SLACK,
    "the stderr-only case took {}ms, which is not the cleanup budget alone",
    elapsed.as_millis()
  );
  // The child itself exited and was reaped, which is why the variant is not
  // called `Orphaned`. What survives is a grandchild, and a grandchild is not
  // this process's to account for.
  assert!(!alive(&reported_pid(&exchange)), "the child was not reaped");
}

/// The same backend, one redirection different: the grandchild holds **stdout**
/// as well, so the body never completes (F-63).
///
/// Both dimensions fail, which no other case in the tier does. The response was
/// written and is never read: the host cannot tell "still writing" from "exited,
/// and something else holds the pipe", and the timeout is the only answer there
/// is.
#[tokio::test]
async fn a_grandchild_holding_stdout_too_fails_both_dimensions() {
  let timeout = Duration::from_millis(300);
  let mut backend = transport("leaves-a-grandchild-holding-stdout-too", timeout);

  let started = Instant::now();
  let exchange = backend.exchange(&evaluate()).await;
  let elapsed = started.elapsed();

  match &exchange.result {
    Err(BackendError::Timeout { after }) => assert_eq!(*after, timeout),
    other => panic!("expected a timeout, got {}", describe(other)),
  }
  match &exchange.cleanup {
    Some(CleanupFailure::TimedOut { after }) => assert_eq!(*after, CLEANUP_LIMIT),
    other => panic!(
      "expected a cleanup timeout, got {}",
      describe_cleanup(other.as_ref())
    ),
  }
  assert!(
    elapsed >= timeout + CLEANUP_LIMIT && elapsed < timeout + CLEANUP_LIMIT + SLACK,
    "the stdout-too case took {}ms, and it should pay both bounds",
    elapsed.as_millis()
  );
  assert!(!alive(&reported_pid(&exchange)), "the child was not reaped");
}

// ---------------------------------------------------------------------------
// EX-3 — the four combinations of §5.4's table, and where each is asserted
// ---------------------------------------------------------------------------
//
// | `result` | `cleanup` | asserted by |
// |---|---|---|
// | `Ok`  | `None` | `a_correct_backend_completes_an_exchange` |
// | `Err` | `None` | `a_backend_that_never_answers_times_out_and_is_disposed_of` |
// | `Ok`  | `Some` | `a_grandchild_holding_stderr_costs_the_cleanup_budget_and_nothing_else` |
// | `Err` | `Some` | `a_grandchild_holding_stdout_too_fails_both_dimensions` |
//
// Four cases, four rows, no fifth test restating them: the point of the two
// dimensions is that each combination is a case someone can meet, not that a
// table exists. Every case above asserts **both** fields, including the ones
// whose interesting half is the other one — which is what makes the `None`s
// load-bearing rather than ignored.

// ---------------------------------------------------------------------------
// EX-5 / VT-5 — nothing this tier spawns outlives the exchange that spawned it
// ---------------------------------------------------------------------------
//
// Every case above asserts this for the child it started, which is EX-5's own
// wording and the sound half under `cargo test`: a target's cases run as threads
// of one process, so an instantaneous global count sees other cases' children
// and fails on their ordinary work. What the aggregate below adds is the claim
// per-case assertions cannot make — that nothing is left behind that no case
// knows about. It runs the misbehaving suite itself and then **settles**: a
// concurrent case's child goes away on its own, and a leak does not.

/// How long the aggregate gives the tier to quieten. Long enough for the slowest
/// case in it (the stdout-too grandchild, at `timeout + CLEANUP_LIMIT`) to
/// finish twice over; short enough that a genuine leak is a failure rather than
/// a wait.
const SETTLE: Duration = Duration::from_secs(3);

/// VT-5 — after the misbehaving suite, no child of this process is a backend.
#[tokio::test]
async fn the_misbehaving_suite_leaves_no_child_behind() {
  let cases = [
    ("hangs-past-the-timeout", Duration::from_millis(200)),
    ("floods-stdout-past-the-cap", Duration::from_secs(5)),
    ("floods-stderr-then-answers", Duration::from_secs(5)),
    ("leaves-a-grandchild-holding-stderr", Duration::from_secs(5)),
    (
      "leaves-a-grandchild-holding-stdout-too",
      Duration::from_millis(200),
    ),
  ];
  for (name, timeout) in cases {
    let mut backend = transport(name, timeout);
    // The outcome is each case's own business; what is asserted here is what is
    // left over afterwards.
    let _ = backend.exchange(&evaluate()).await;
  }

  let deadline = Instant::now() + SETTLE;
  loop {
    let remaining = harness::children();
    if remaining.is_empty() {
      return;
    }
    assert!(
      Instant::now() < deadline,
      "{} process(es) still a child of this one after {}s: {}",
      remaining.len(),
      SETTLE.as_secs(),
      remaining.join(", ")
    );
    tokio::time::sleep(Duration::from_millis(20)).await;
  }
}

/// The guard for the assertion above: a check that cannot see a child it is
/// looking straight at reports "clean" for a process tree it never inspected.
/// `tests/protocol/boundary.rs` and `transport_shape.rs` both carry one of
/// these, and for the same reason.
#[test]
fn a_backend_that_is_running_is_seen_as_a_child() {
  let mut child = std::process::Command::new("bash")
    .args(harness::backend("hangs-past-the-timeout").split_off(1))
    .stdin(std::process::Stdio::null())
    .stdout(std::process::Stdio::null())
    .stderr(std::process::Stdio::null())
    .spawn()
    .expect("bash is on PATH");

  let seen = harness::children();
  let pid = child.id().to_string();

  child.kill().expect("the child is ours to kill");
  child.wait().expect("and ours to reap");

  assert!(
    seen.contains(&pid),
    "the enumeration missed a live child: it saw {}",
    if seen.is_empty() {
      "nothing".to_owned()
    } else {
      seen.join(", ")
    }
  );
  assert!(
    !harness::children().contains(&pid),
    "a reaped child is still being counted"
  );
}

// ---------------------------------------------------------------------------
// VT-6 — cancellation, behaviourally. The structural half is
// `tests/protocol/transport_shape.rs`, which asserts the transport spawns
// nothing to leak in the first place.
// ---------------------------------------------------------------------------

/// A dropped exchange leaves nothing **the host holds** behind.
///
/// That is the narrow claim AC-5 makes and all this can make: on cancellation no
/// code of ours runs, so the child's disposal falls to `kill_on_drop`, which is
/// best-effort by tokio's own documentation (D54, F-60). What is assertable is
/// the task count, and it is only assertable on a runtime this test owns —
/// `num_alive_tasks` is per-runtime, so `#[tokio::test]`'s shared one would
/// count other cases' work.
///
/// The exchange has to be **driven far enough to have started work** before it
/// is dropped, or the assertion is vacuous: a future dropped before its first
/// poll leaves the count at zero however the transport is written (F-12). The
/// spawned child is the evidence that it did — the argv carries a marker unique
/// to this case, so waiting on it cannot be satisfied by another case's backend.
#[test]
fn a_cancelled_exchange_leaves_nothing_of_the_host_behind() {
  let runtime = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("a current-thread runtime");
  let marker = format!("cancellation-{}", std::process::id());

  runtime.block_on(async {
    let handle = tokio::runtime::Handle::current();
    assert_eq!(
      handle.metrics().num_alive_tasks(),
      0,
      "the runtime under measurement is not this test's own"
    );

    let command = marker.clone();
    let task = tokio::spawn(async move {
      // `bash` ignores the trailing argument; it is here to be found in
      // `/proc/<pid>/cmdline`.
      let mut argv = harness::backend("hangs-without-exec");
      argv.push(command);
      let mut backend = ProcessBackend::new(argv, Duration::from_secs(30));
      backend.exchange(&evaluate()).await
    });

    // The positive control: the metric is live and would see a leak, and the
    // exchange has reached its spawn.
    while harness::children_running(&marker).is_empty() {
      tokio::task::yield_now().await;
    }
    assert!(
      handle.metrics().num_alive_tasks() >= 1,
      "the exchange is in flight and the metric cannot see it, so it could not see a leak either"
    );

    task.abort();
    assert!(
      task.await.is_err(),
      "the exchange completed instead of being cancelled, so nothing was dropped"
    );
    tokio::task::yield_now().await;

    assert_eq!(
      handle.metrics().num_alive_tasks(),
      0,
      "something the host holds outlived the cancelled exchange"
    );
  });
}
